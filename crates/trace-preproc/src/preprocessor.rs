use crate::macros::{lex_macro_body, MacroDef, MacroOp, MacroTable};
use crate::{Diagnostic, DiagnosticSeverity, Lexer, LineMap, PreprocessOptions, Token, TokenKind};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use thiserror::Error;

/// Nested macro-expansion cap (C11 hide-set is the primary recursion brake;
/// this is a backstop for pathological `##` / hide-set edge cases).
const MAX_MACRO_EXPANSION_DEPTH: u32 = 256;

#[derive(Debug, Error)]
pub enum PreprocessError {
    #[error("failed to read {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{message}")]
    Message { message: String },
}

#[derive(Debug, Clone)]
pub struct PreprocessResult {
    pub output: String,
    pub line_map: LineMap,
    pub diagnostics: Vec<Diagnostic>,
    /// Canonical paths processed by this run (`#include` closure).
    pub included_headers: Vec<PathBuf>,
}

#[derive(Debug)]
struct PreprocessorState {
    opts: PreprocessOptions,
    macros: MacroTable,
    /// Names in `macros` defined only by a builtin fallback (see
    /// `install_builtin_macros`). These expand normally but are invisible to
    /// `#ifdef` / `#ifndef` / `defined()` so an `#ifndef`-guarded real
    /// definition in source still takes effect; any source (re)definition or
    /// `#undef` clears the mark. Only ever shrinks during a run.
    fallback_macros: HashSet<String>,
    include_stack: Vec<PathBuf>,
    included_guard: HashSet<PathBuf>,
    conditional_stack: Vec<CondFrame>,
    /// Depth of `conditional_stack` when the current file started. Frames
    /// below it belong to includers: `#elif`/`#else`/`#endif` in this file
    /// may only act on frames at or above it (see `process_file_tokens`).
    cond_base: usize,
    output: String,
    line_map: LineMap,
    diagnostics: Vec<Diagnostic>,
    current_file: PathBuf,
    current_line: u32,
    /// Bytes each processed file contributed to `output`. Files whose
    /// expansion was fully skipped (e.g. by an already-defined include
    /// guard) record 0 and must not be claimed as content-bearing by a
    /// parent's cached `IncludeExpansion::files`.
    emitted_bytes: HashMap<PathBuf, usize>,
    /// Interned index of `current_file` in `line_map.files`; `u32::MAX`
    /// means "not interned yet" (re-interned lazily when the file changes).
    lm_cur_file: u32,
    /// Current nested macro-expansion depth (hide-set rescan frames).
    expansion_depth: u32,
    expansion_limit_warned: bool,
    /// Token-loop iterations this run (macro rescan included).
    tokens_processed: u64,
    /// In-progress cached-header frames (warm pass). Guard-skipped includes
    /// are recorded here so the finished entry can embed nested expansions
    /// without copying them into live `output` (that exponentiates on
    /// diamond include graphs).
    cache_frames: Vec<CacheFrame>,
    /// Macro directives executed while any cache frame is open (nested
    /// replays included), in order. Each frame remembers its start index
    /// and captures its suffix into `IncludeExpansion::ops`; cleared when
    /// the last frame closes.
    macro_ops: Vec<MacroOp>,
}

/// One level of `#if`/`#elif`/`#else` nesting. A per-level bool is not
/// enough: `#elif`/`#else` must know whether *any* earlier branch in the
/// chain was taken (else the `#else` re-activates after a taken `#elif`),
/// and whether the enclosing context is active at all.
#[derive(Debug, Clone, Copy)]
struct CondFrame {
    /// Was the enclosing context active when the `#if` was seen?
    parent_active: bool,
    /// Is the branch currently being processed emitting tokens?
    active: bool,
    /// Has any branch of this chain been taken yet?
    taken: bool,
    /// Has `#else` been seen (a later `#elif` is malformed)?
    else_seen: bool,
    /// Line of the opening `#if`/`#ifdef`/`#ifndef`, for the EOF diagnostic.
    line: u32,
}

/// One cached header being constructed.
#[derive(Debug)]
struct CacheFrame {
    /// Guard-skipped includes at the live-output offset of the `#include`.
    skips: Vec<(usize, PathBuf)>,
}

impl PreprocessorState {
    fn new(opts: PreprocessOptions, file: PathBuf) -> Self {
        let mut state = Self {
            opts,
            macros: MacroTable::new(),
            fallback_macros: HashSet::new(),
            include_stack: vec![file.clone()],
            included_guard: HashSet::new(),
            conditional_stack: Vec::new(),
            cond_base: 0,
            output: String::new(),
            line_map: LineMap::new(),
            diagnostics: Vec::new(),
            current_file: file,
            current_line: 1,
            emitted_bytes: HashMap::new(),
            lm_cur_file: u32::MAX,
            expansion_depth: 0,
            expansion_limit_warned: false,
            tokens_processed: 0,
            cache_frames: Vec::new(),
            macro_ops: Vec::new(),
        };
        if let Some(shared) = &state.opts.shared_macros {
            if let Ok(guard) = shared.read() {
                state.macros = guard.clone();
            }
            // The warm table is normally seeded from the CLI defines, but a
            // name it never accumulated must still beat the builtin fallback
            // installed below. First-wins keeps definitions the warm pass
            // picked up from source.
            state.init_cli_defines_missing_only();
        } else {
            state.init_cli_defines();
        }
        // Builtins are local to each preprocess so they apply even when
        // the shared warm table is cloned (hiview `__UNUSED` lives in .cpp
        // files, not in the header that `#ifndef`s it).
        state.install_builtin_macros();
        state
    }

    /// Install `BUILTIN_FALLBACK_MACROS`, each only when not already defined
    /// and marked in `fallback_macros` so conditionals do not see it and any
    /// real definition (CLI `-D`, source `#define`, cached include delta)
    /// replaces it.
    fn install_builtin_macros(&mut self) {
        for (name, def) in BUILTIN_FALLBACK_MACROS.iter() {
            if !self.macros.contains_key(name.as_str()) {
                self.macros.insert(name.clone(), def.clone());
                self.fallback_macros.insert(name.clone());
            }
        }
    }

    /// A name only a builtin fallback defines does not count as defined for
    /// `#ifdef` / `#ifndef` / `defined()`.
    fn is_defined_for_conditionals(&self, name: &str) -> bool {
        self.macros.contains_key(name) && !self.fallback_macros.contains(name)
    }

    fn init_cli_defines(&mut self) {
        let defines: Vec<_> = self
            .opts
            .defines
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (name, val) in defines {
            self.insert_macro(
                name,
                MacroDef::Object {
                    replacement: lex_macro_body(&val),
                },
            );
        }
    }

    fn init_cli_defines_missing_only(&mut self) {
        let defines: Vec<_> = self
            .opts
            .defines
            .iter()
            .filter(|(k, _)| !self.macros.contains_key(k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (name, val) in defines {
            self.insert_macro(
                name,
                MacroDef::Object {
                    replacement: lex_macro_body(&val),
                },
            );
        }
    }

    /// Record a directive for the enclosing cached-header entries, if any.
    /// Logged unconditionally within a frame — even a `#undef` of an absent
    /// name is a no-op only locally and can still take effect in a
    /// translation unit that replays the entry.
    fn log_macro_op(&mut self, op: MacroOp) {
        if !self.cache_frames.is_empty() {
            self.macro_ops.push(op);
        }
    }

    fn insert_macro(&mut self, name: String, def: MacroDef) {
        self.log_macro_op(MacroOp::Define(name.clone(), def.clone()));
        self.fallback_macros.remove(&name);
        self.macros.insert(name.clone(), def.clone());
        if self.opts.accumulate_macros {
            if let Some(shared) = &self.opts.shared_macros {
                if let Ok(mut guard) = shared.write() {
                    guard.insert(name, def);
                }
            }
        }
    }

    fn remove_macro(&mut self, name: &str) {
        self.log_macro_op(MacroOp::Undef(name.to_string()));
        self.fallback_macros.remove(name);
        self.macros.shift_remove(name);
        if self.opts.accumulate_macros {
            if let Some(shared) = &self.opts.shared_macros {
                if let Ok(mut guard) = shared.write() {
                    guard.shift_remove(name);
                }
            }
        }
    }

    fn is_active(&self) -> bool {
        // A frame's `active` already folds in its parent's state at push /
        // re-evaluation time, so only the innermost frame needs checking.
        self.conditional_stack.last().is_none_or(|f| f.active)
    }

    /// Does the current file have an open conditional of its own for a
    /// `#elif`/`#else`/`#endif` to act on? Includer frames do not count.
    fn has_own_cond(&self) -> bool {
        self.conditional_stack.len() > self.cond_base
    }

    fn push_cond(&mut self, cond: bool, line: u32) {
        let parent_active = self.is_active();
        let active = parent_active && cond;
        self.conditional_stack.push(CondFrame {
            parent_active,
            active,
            taken: active,
            else_seen: false,
            line,
        });
    }

    fn push_expansion(&mut self, line: u32) -> bool {
        if self.expansion_depth >= MAX_MACRO_EXPANSION_DEPTH {
            if !self.expansion_limit_warned {
                self.warn(
                    line,
                    format!(
                        "macro expansion depth exceeded ({MAX_MACRO_EXPANSION_DEPTH}); skipping further expansion"
                    ),
                );
                self.expansion_limit_warned = true;
            }
            return false;
        }
        self.expansion_depth += 1;
        true
    }

    fn pop_expansion(&mut self) {
        self.expansion_depth = self.expansion_depth.saturating_sub(1);
    }

    fn paint_replacement(tokens: &[Token], origin: &Token, name: &str) -> Vec<Token> {
        tokens
            .iter()
            .map(|t| t.with_macro_hide(origin, name))
            .collect()
    }

    /// Intern a path into the line-map file table (no-op if present).
    fn lm_intern(&mut self, path: &Path) -> u32 {
        self.line_map.intern_file(path)
    }

    /// Index of the current file in the line-map table, re-interned only
    /// when `current_file` changed since the last call.
    fn lm_current_file(&mut self) -> u32 {
        if self.lm_cur_file == u32::MAX
            || self.line_map.files.get(self.lm_cur_file as usize) != Some(&self.current_file)
        {
            self.lm_cur_file = self.line_map.intern_file(&self.current_file);
        }
        self.lm_cur_file
    }

    fn emit_token(&mut self, tok: &Token) {
        if matches!(tok.kind, TokenKind::Eof) {
            return;
        }
        if !matches!(tok.kind, TokenKind::Newline) && needs_leading_space(&self.output, &tok.kind) {
            self.output.push(' ');
        }
        let offset = self.output.len();
        let text = token_to_string(&tok.kind);
        self.output.push_str(&text);
        if self.opts.track_line_map {
            let fid = self.lm_current_file();
            let (line, col) = tok.expansion_site();
            self.line_map.push(offset, fid, line, col);
        }
        if matches!(tok.kind, TokenKind::Newline) {
            self.current_line += 1;
        }
    }

    fn emit_str(&mut self, s: &str, line: u32, col: u32) {
        let offset = self.output.len();
        self.output.push_str(s);
        if self.opts.track_line_map {
            let fid = self.lm_current_file();
            self.line_map.push(offset, fid, line, col);
        }
    }

    /// Emit `__FILE__` / `__LINE__` for `tok` if `name` is one of them.
    /// `__LINE__` is the line of the invocation (C11 6.10.8.1), also when
    /// the token comes from a macro body: `expansion_site` carries that
    /// through nested expansions.
    fn emit_predefined(&mut self, tok: &Token, name: &str) -> bool {
        let (line, col) = tok.expansion_site();
        match name {
            "__FILE__" => {
                let text = format!("\"{}\"", self.current_file.display());
                self.emit_str(&text, line, col);
                true
            }
            "__LINE__" => {
                self.emit_str(&line.to_string(), line, col);
                true
            }
            _ => false,
        }
    }

    fn report(&mut self, severity: DiagnosticSeverity, line: u32, message: String) {
        self.diagnostics.push(Diagnostic {
            severity,
            file: Some(self.current_file.clone()),
            line,
            message,
        });
    }

    fn warn(&mut self, line: u32, message: impl Into<String>) {
        self.report(DiagnosticSeverity::Warning, line, message.into());
    }

    /// Records an error diagnostic and returns the matching hard error for
    /// callers that abort the current file.
    fn error(&mut self, line: u32, message: impl Into<String>) -> PreprocessError {
        let msg = message.into();
        self.report(DiagnosticSeverity::Error, line, msg.clone());
        PreprocessError::Message { message: msg }
    }

    fn check_resource_limits(&mut self, line: u32) -> Result<(), PreprocessError> {
        self.tokens_processed = self.tokens_processed.saturating_add(1);
        if self.tokens_processed > self.opts.max_expanded_tokens {
            return Err(self.error(
                line,
                format!(
                    "preprocessed token budget exceeded ({})",
                    self.opts.max_expanded_tokens
                ),
            ));
        }
        if self.output.len() > self.opts.max_output_bytes {
            return Err(self.error(
                line,
                format!(
                    "preprocessed output exceeded {} bytes",
                    self.opts.max_output_bytes
                ),
            ));
        }
        Ok(())
    }

    /// Replay a cached expansion into the output. Returns false when no
    /// entry exists for `canonical`.
    fn splice_cached(&mut self, canonical: &Path) -> bool {
        let Some(cache) = &self.opts.include_expansion_cache else {
            return false;
        };
        let Some(entry) = cache
            .read()
            .ok()
            .and_then(|guard| guard.get(canonical).cloned())
        else {
            return false;
        };
        if !self.opts.inline_include_bodies {
            self.replay_macro_delta(&entry);
            self.included_guard.insert(canonical.to_path_buf());
            self.included_guard.extend(entry.files.iter().cloned());
            return true;
        }
        if self.output.len().saturating_add(entry.text.len()) > self.opts.max_output_bytes {
            self.warn(
                1,
                format!(
                    "skipping cached include {} (would exceed {}-byte output cap)",
                    canonical.display(),
                    self.opts.max_output_bytes
                ),
            );
            self.included_guard.insert(canonical.to_path_buf());
            return true;
        }
        let offset = self.output.len();
        self.output.push_str(&entry.text);
        // Replay the entry's macro side effects. Cached text is spliced
        // without executing the header's directives, so without this a
        // consumer sees none of the macros the header defines — later
        // warm passes then expand dependent headers against a starved
        // table and freeze unexpanded invocations into their own cache
        // entries. Replay mirrors live execution (see replay_macro_delta).
        self.replay_macro_delta(&entry);
        if self.opts.track_line_map {
            // Renumber the cached expansion's file indices into this run's
            // intern table, then splice its entries.
            let mut remap = Vec::with_capacity(entry.line_map.files.len());
            for p in &entry.line_map.files {
                remap.push(self.lm_intern(p));
            }
            let sub = &entry.line_map;
            self.line_map.splice(sub, offset, &remap);
        }
        self.included_guard.extend(entry.files.iter().cloned());
        true
    }

    /// Replay a cached include's macro directives in program order, through
    /// the same mutation helpers live `#define` / `#undef` use — so a cache
    /// hit and a cache miss agree on everything a directive touches: the
    /// local table (overwrite semantics), the fallback marks, the shared
    /// table under `accumulate_macros`, and the op log feeding an enclosing
    /// cached header's own entry.
    fn replay_macro_delta(&mut self, entry: &crate::IncludeExpansion) {
        for op in entry.ops.iter() {
            match op {
                MacroOp::Undef(name) => self.remove_macro(name),
                MacroOp::Define(name, def) => self.insert_macro(name.clone(), def.clone()),
            }
        }
    }

    fn cached_expansion(&self, canonical: &Path) -> Option<crate::IncludeExpansion> {
        let cache = self.opts.include_expansion_cache.as_ref()?;
        cache
            .read()
            .ok()
            .and_then(|guard| guard.get(canonical).cloned())
    }

    fn is_cacheable_header(path: &Path) -> bool {
        path.extension().and_then(|e| e.to_str()).is_some_and(|e| {
            matches!(e, "h" | "H" | "hpp" | "hh" | "hxx" | "inl" | "ipp")
                || e.eq_ignore_ascii_case("h")
        })
    }

    /// Self-contained cache blob: live unique text plus nested expansions
    /// inserted at each first guard-skip include site.
    fn compose_cache_text(
        &self,
        output_start: usize,
        output_end: usize,
        skips: &[(usize, PathBuf)],
    ) -> (String, LineMap, HashSet<PathBuf>) {
        if skips.is_empty() {
            return (
                self.output[output_start..output_end].to_string(),
                self.line_map.slice_from(output_start),
                HashSet::new(),
            );
        }
        let mut text = String::new();
        let mut line_map = LineMap::new();
        let mut extra_files = HashSet::new();
        let mut live_pos = output_start;
        let mut embedded: HashSet<PathBuf> = HashSet::new();
        for (at, path) in skips {
            let at = (*at).min(output_end).max(live_pos);
            Self::append_live_chunk(
                &mut text,
                &mut line_map,
                &self.output,
                &self.line_map,
                live_pos,
                at,
            );
            live_pos = at;
            if !embedded.insert(path.clone()) {
                continue;
            }
            let Some(entry) = self.cached_expansion(path) else {
                continue;
            };
            if text.len().saturating_add(entry.text.len()) > self.opts.max_output_bytes {
                continue;
            }
            extra_files.extend(entry.files.iter().cloned());
            extra_files.insert(path.clone());
            if self.opts.track_line_map {
                let mut remap = Vec::with_capacity(entry.line_map.files.len());
                for p in &entry.line_map.files {
                    remap.push(line_map.intern_file(p));
                }
                line_map.splice(&entry.line_map, text.len(), &remap);
            }
            text.push_str(&entry.text);
        }
        Self::append_live_chunk(
            &mut text,
            &mut line_map,
            &self.output,
            &self.line_map,
            live_pos,
            output_end,
        );
        (text, line_map, extra_files)
    }

    fn append_live_chunk(
        dest_text: &mut String,
        dest_map: &mut LineMap,
        src_text: &str,
        src_map: &LineMap,
        from: usize,
        to: usize,
    ) {
        if from >= to {
            return;
        }
        let dest_off = dest_text.len();
        dest_text.push_str(&src_text[from..to]);
        let chunk_len = to - from;
        let sliced = src_map.slice_from(from);
        let mut remap = Vec::with_capacity(sliced.files.len());
        for p in &sliced.files {
            remap.push(dest_map.intern_file(p));
        }
        for e in &sliced.entries {
            if (e.output_offset as usize) >= chunk_len {
                break;
            }
            dest_map.entries.push(crate::LineMapEntry {
                output_offset: e.output_offset + dest_off as u32,
                file: remap[e.file as usize],
                line: e.line,
                col: e.col,
            });
        }
    }

    fn process_file(&mut self, path: &Path) -> Result<(), PreprocessError> {
        let canonical = trace_ir::canonicalize(path);
        if self.included_guard.contains(&canonical) {
            // Already expanded earlier in this run. Re-splicing the cached
            // subtree into *live* output exponentiates on diamond include
            // graphs (each skip copies a self-contained blob that already
            // contains previous copies). Record the skip on the in-progress
            // cache frame instead; `compose_cache_text` embeds the nested
            // expansion only into that frame's cache entry.
            if !self.opts.frozen_expansion_cache {
                if let Some(frame) = self.cache_frames.last_mut() {
                    frame.skips.push((self.output.len(), canonical.clone()));
                }
            }
            return Ok(());
        }

        if self.include_stack.len() >= self.opts.max_include_depth {
            self.warn(
                1,
                format!(
                    "include depth exceeded ({}); skipping {}",
                    self.opts.max_include_depth,
                    path.display()
                ),
            );
            return Ok(());
        }

        if self.splice_cached(&canonical) {
            return Ok(());
        }

        let cache_header =
            self.opts.include_expansion_cache.is_some() && Self::is_cacheable_header(&canonical);

        let guard_snapshot = if cache_header {
            self.included_guard.clone()
        } else {
            HashSet::new()
        };
        // Everything this header's processing executes (`#define`/`#undef`,
        // nested replays included) from here on lands in `macro_ops`; the
        // suffix becomes the entry's `IncludeExpansion::ops`, replayed by
        // `splice_cached`.
        let ops_start = if cache_header && !self.opts.frozen_expansion_cache {
            Some(self.macro_ops.len())
        } else {
            None
        };
        self.included_guard.insert(canonical.clone());
        let output_start = self.output.len();
        let pushing_frame = cache_header && !self.opts.frozen_expansion_cache;
        if pushing_frame {
            self.cache_frames.push(CacheFrame { skips: Vec::new() });
        }

        let content: Arc<str> = if let Some(cache) = &self.opts.source_cache {
            let key = canonical.clone();
            if let Some(s) = cache.get(&key) {
                Arc::clone(s)
            } else {
                fs::read_to_string(path)
                    .map_err(|source| PreprocessError::Io {
                        path: path.to_path_buf(),
                        source,
                    })?
                    .into()
            }
        } else {
            fs::read_to_string(path)
                .map_err(|source| PreprocessError::Io {
                    path: path.to_path_buf(),
                    source,
                })?
                .into()
        };

        let prev_file = self.current_file.clone();
        self.current_file = path.to_path_buf();
        self.include_stack.push(path.to_path_buf());

        let tokens = Lexer::new(&content).tokenize();
        if let Err(e) = self.process_file_tokens(&tokens) {
            // Attribute the stop to the file being processed when it failed,
            // not the including TU — downstream consumers key fallback and
            // reporting decisions off this message.
            self.warn(
                1,
                format!("preprocess stopped in {}: {e}", self.current_file.display()),
            );
        }

        self.include_stack.pop();
        self.current_file = prev_file;

        let emitted = self.output.len() - output_start;
        self.emitted_bytes.insert(canonical.clone(), emitted);
        let pending_skips = self.cache_frames.last().map(|f| f.skips.len()).unwrap_or(0);
        if cache_header
            && !self.opts.frozen_expansion_cache
            && emitted == 0
            && pending_skips == 0
            && content.chars().any(|c| !c.is_whitespace())
        {
            // The include resolved but its entire body was skipped — almost
            // always an include guard already defined in the shared macro
            // environment. Content silently missing from a cached expansion
            // is the failure mode that starves translation units later, so
            // make it visible during the (sequential) warm/index phases.
            self.diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                file: Some(path.to_path_buf()),
                line: 1,
                message: "resolved include expanded to nothing (guard already defined?)".into(),
            });
        }

        if cache_header && !self.opts.frozen_expansion_cache {
            let frame = self.cache_frames.pop();
            if let Some(cache) = &self.opts.include_expansion_cache {
                let skips = frame.map(|f| f.skips).unwrap_or_default();
                let output_end = self.output.len();
                let (composed, composed_map, extra_files) = if self.opts.inline_include_bodies {
                    self.compose_cache_text(output_start, output_end, &skips)
                } else {
                    (
                        self.output[output_start..output_end].to_string(),
                        self.line_map.slice_from(output_start),
                        HashSet::new(),
                    )
                };
                let mut new_files: HashSet<PathBuf> = self
                    .included_guard
                    .difference(&guard_snapshot)
                    .filter(|p| self.emitted_bytes.get(*p).copied().unwrap_or(0) > 0)
                    .cloned()
                    .collect();
                new_files.extend(extra_files);
                let ops: Arc<Vec<MacroOp>> = match ops_start {
                    Some(start) => Arc::new(self.macro_ops[start..].to_vec()),
                    None => Arc::default(),
                };
                if !composed.is_empty() || !ops.is_empty() || !new_files.is_empty() {
                    if let Ok(mut guard) = cache.write() {
                        guard.entry(canonical).or_insert(crate::IncludeExpansion {
                            text: composed.into(),
                            files: Arc::new(new_files),
                            line_map: Arc::new(composed_map),
                            ops,
                        });
                    }
                }
            }
            // The log only feeds open frames; once the outermost cached
            // header closes, nothing references these entries any more.
            if self.cache_frames.is_empty() {
                self.macro_ops.clear();
            }
        }

        Ok(())
    }

    /// Runs one file's token stream with `conditional_stack` fenced at the
    /// depth it had on entry. In the C11 6.10 grammar an `if-section` is a
    /// group of the `preprocessing-file` that contains it, so a conditional
    /// cannot span files in either direction: a closing or branch directive
    /// here may not act on an includer's frame (`cond_base`), and frames
    /// still open at the end belong to this file alone. Report each of
    /// those at its directive and pop them, leaving the includer in the
    /// state it had at the `#include`. Without this an unterminated `#if 0`
    /// in a header silently blanked the rest of the translation unit, and a
    /// stray `#endif` in a header consumed the includer's frame so its own
    /// `#endif` then failed (#8). A file that stopped early is rebalanced
    /// the same way but keeps only its stop diagnostic: its unprocessed
    /// remainder may still hold the `#endif`.
    fn process_file_tokens(&mut self, tokens: &[Token]) -> Result<(), PreprocessError> {
        let depth = self.conditional_stack.len();
        let outer_base = std::mem::replace(&mut self.cond_base, depth);
        let result = self.process_tokens(tokens);
        self.cond_base = outer_base;
        if result.is_ok() {
            for idx in depth..self.conditional_stack.len() {
                let line = self.conditional_stack[idx].line;
                self.report(
                    DiagnosticSeverity::Error,
                    line,
                    "unterminated #if; conditional closed at end of file".into(),
                );
            }
        }
        self.conditional_stack.truncate(depth);
        result
    }

    fn process_tokens(&mut self, tokens: &[Token]) -> Result<(), PreprocessError> {
        let mut i = 0;
        while i < tokens.len() {
            self.check_resource_limits(tokens[i].line)?;
            let tok = &tokens[i];
            if matches!(tok.kind, TokenKind::Eof) {
                break;
            }

            if matches!(tok.kind, TokenKind::Hash) && at_beginning_of_line(tokens, i) {
                i = self.handle_directive(tokens, i)?;
                continue;
            }

            if self.is_active() {
                if let TokenKind::Identifier(name) = &tok.kind {
                    if self.emit_predefined(tok, name) {
                        i += 1;
                        continue;
                    }
                    if !tok.is_hidden(name) {
                        if let Some(macro_def) = self.macros.get(name).cloned() {
                            match macro_def {
                                MacroDef::Function {
                                    params,
                                    replacement,
                                    variadic,
                                } => {
                                    if self.next_non_newline_is(tokens, i + 1, "(") {
                                        if !self.push_expansion(tok.line) {
                                            self.emit_token(tok);
                                            i += 1;
                                            continue;
                                        }
                                        i += 1;
                                        let args = match self.parse_macro_args(tokens, &mut i) {
                                            Ok(a) => a,
                                            Err(e) => {
                                                self.pop_expansion();
                                                return Err(e);
                                            }
                                        };
                                        let expanded = apply_concatenation(substitute_macro(
                                            name,
                                            tok,
                                            &replacement,
                                            &params,
                                            &args,
                                            variadic,
                                        ));
                                        let r = self.process_tokens(&expanded);
                                        self.pop_expansion();
                                        r?;
                                        continue;
                                    }
                                    self.emit_token(tok);
                                }
                                MacroDef::Object { replacement } => {
                                    if !self.push_expansion(tok.line) {
                                        self.emit_token(tok);
                                        i += 1;
                                        continue;
                                    }
                                    let painted = Self::paint_replacement(&replacement, tok, name);
                                    let r = self.expand_tokens_no_directives(&painted);
                                    self.pop_expansion();
                                    r?;
                                    i += 1;
                                    continue;
                                }
                            }
                        } else {
                            self.emit_token(tok);
                        }
                    } else {
                        self.emit_token(tok);
                    }
                } else {
                    self.emit_token(tok);
                }
            }
            i += 1;
        }
        Ok(())
    }

    /// Expand macro replacement tokens: no `#` directives (a `#` here is
    /// emitted verbatim — `#param` was already stringized by
    /// `substitute_macro`); recurse into object macros.
    fn expand_tokens_no_directives(&mut self, tokens: &[Token]) -> Result<(), PreprocessError> {
        let mut i = 0;
        while i < tokens.len() {
            self.check_resource_limits(tokens[i].line)?;
            let tok = &tokens[i];
            if matches!(tok.kind, TokenKind::Eof) {
                break;
            }
            if matches!(tok.kind, TokenKind::Hash) {
                self.emit_token(tok);
                i += 1;
                continue;
            }
            if self.is_active() {
                if let TokenKind::Identifier(name) = &tok.kind {
                    if self.emit_predefined(tok, name) {
                        i += 1;
                        continue;
                    }
                    if !tok.is_hidden(name) {
                        match self.macros.get(name).cloned() {
                            Some(MacroDef::Object { replacement }) => {
                                if !self.push_expansion(tok.line) {
                                    self.emit_token(tok);
                                    i += 1;
                                    continue;
                                }
                                let painted = Self::paint_replacement(&replacement, tok, name);
                                let r = self.expand_tokens_no_directives(&painted);
                                self.pop_expansion();
                                r?;
                                i += 1;
                                continue;
                            }
                            // Function-like macros appearing inside another
                            // macro's expansion must be invoked and their
                            // expansion rescanned (C11 6.10.3.4); otherwise
                            // nested definitions like
                            // `#define A SHARED_OBJ(T)` leak `SHARED_OBJ(T)`
                            // verbatim into the output.
                            Some(MacroDef::Function {
                                params,
                                replacement,
                                variadic,
                            }) if self.next_non_newline_is(tokens, i + 1, "(") => {
                                if !self.push_expansion(tok.line) {
                                    self.emit_token(tok);
                                    i += 1;
                                    continue;
                                }
                                let mut j = i + 1;
                                let args = match self.parse_macro_args(tokens, &mut j) {
                                    Ok(a) => a,
                                    Err(e) => {
                                        self.pop_expansion();
                                        return Err(e);
                                    }
                                };
                                let expanded = apply_concatenation(substitute_macro(
                                    name,
                                    tok,
                                    &replacement,
                                    &params,
                                    &args,
                                    variadic,
                                ));
                                let r = self.expand_tokens_no_directives(&expanded);
                                self.pop_expansion();
                                r?;
                                i = j;
                                continue;
                            }
                            Some(MacroDef::Function { .. }) | None => {}
                        }
                    }
                }
                self.emit_token(tok);
            }
            i += 1;
        }
        Ok(())
    }

    fn handle_directive(
        &mut self,
        tokens: &[Token],
        start: usize,
    ) -> Result<usize, PreprocessError> {
        let mut i = start + 1;
        // skip to directive name (may be on next line)
        while i < tokens.len() && matches!(tokens[i].kind, TokenKind::Newline) {
            i += 1;
        }
        if i >= tokens.len() {
            return Ok(i);
        }

        let directive = match &tokens[i].kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => {
                return Err(self.error(tokens[i].line, "expected directive name after #"));
            }
        };
        let line = tokens[i].line;
        i += 1;

        match directive.as_str() {
            "include" if self.is_active() => {
                i = self.handle_include(tokens, i)?;
            }
            "define" if self.is_active() => {
                i = self.handle_define(tokens, i)?;
            }
            "include" | "define" if !self.is_active() => {}
            // Inside a skipped group only the nesting matters (C11
            // 6.10.1p6): a malformed operand there must not abort the file.
            "ifdef" => {
                if self.is_active() {
                    let name = self.read_directive_ident(tokens, &mut i)?;
                    let defined = self.is_defined_for_conditionals(&name);
                    self.push_cond(defined, line);
                } else {
                    self.push_cond(false, line);
                }
            }
            "ifndef" => {
                if self.is_active() {
                    let name = self.read_directive_ident(tokens, &mut i)?;
                    let defined = self.is_defined_for_conditionals(&name);
                    self.push_cond(!defined, line);
                } else {
                    self.push_cond(false, line);
                }
            }
            "if" => {
                // Conditions in skipped groups are not evaluated (C11
                // 6.10.1p6); the frame still pushes to keep nesting balanced.
                let cond = if self.is_active() {
                    self.expand_and_eval_condition(tokens, &mut i)
                } else {
                    self.skip_condition_tokens(tokens, &mut i);
                    false
                };
                self.push_cond(cond, line);
            }
            "elif" => {
                if !self.has_own_cond() {
                    return Err(self.error(line, "#elif without #if"));
                }
                let frame = *self.conditional_stack.last().unwrap();
                let cond = if frame.parent_active && !frame.taken && !frame.else_seen {
                    self.expand_and_eval_condition(tokens, &mut i)
                } else {
                    self.skip_condition_tokens(tokens, &mut i);
                    false
                };
                if frame.else_seen {
                    self.warn(line, "#elif after #else; branch ignored");
                }
                let f = self.conditional_stack.last_mut().unwrap();
                f.active = f.parent_active && !f.taken && cond;
                f.taken |= f.active;
            }
            "else" => {
                if !self.has_own_cond() {
                    return Err(self.error(line, "#else without #if"));
                }
                let f = self.conditional_stack.last_mut().unwrap();
                f.active = f.parent_active && !f.taken;
                f.taken = true;
                f.else_seen = true;
            }
            "endif" => {
                if !self.has_own_cond() {
                    return Err(self.error(line, "#endif without #if"));
                }
                self.conditional_stack.pop();
            }
            // Directives whose operands we ignore: the shared skip below
            // consumes the rest of the line. Calling skip_to_newline here as
            // well would eat the newline AND the whole following line
            // (e.g. `#pragma pack(push, 4)` swallowing the struct after it).
            "line" => {}
            "pragma" => {}
            "undef" if self.is_active() => {
                let name = self.read_directive_ident(tokens, &mut i)?;
                self.remove_macro(&name);
            }
            "undef" if !self.is_active() => {}
            _ => {
                self.warn(
                    tokens[i.saturating_sub(1)].line,
                    format!("unknown directive #{directive}"),
                );
            }
        }
        i = self.skip_to_newline(tokens, i);
        Ok(i)
    }

    fn handle_include(&mut self, tokens: &[Token], mut i: usize) -> Result<usize, PreprocessError> {
        while i < tokens.len() && matches!(tokens[i].kind, TokenKind::Newline) {
            i += 1;
        }
        let line = tokens.get(i).map(|t| t.line).unwrap_or(1);
        // C11 6.10.2: a header-name (`"..."` / `<...>`) is taken as-is;
        // otherwise the rest of the line is macro-expanded and must then
        // form a header-name (`#include FOO` with `#define FOO "n.h"`).
        let path = if let Some(p) = parse_include_header(&tokens[i..]) {
            p
        } else {
            let mut end = i;
            while end < tokens.len()
                && !matches!(tokens[end].kind, TokenKind::Newline | TokenKind::Eof)
            {
                end += 1;
            }
            let expanded = self.expand_include_operand(&tokens[i..end])?;
            match parse_include_header(&expanded) {
                Some(p) => p,
                None => {
                    return Err(self.error(line, "expected string or <...> after #include"));
                }
            }
        };

        let include_path = match self.resolve_include(&path) {
            Ok(p) => p,
            Err(_) => {
                self.warn(line, format!("include file not found, skipping: {path}"));
                return Ok(i);
            }
        };
        let live_at = self.output.len();
        if let Err(e) = self.process_file(&include_path) {
            self.warn(
                line,
                format!("include preprocessing failed for {path}: {e}"),
            );
        }
        // File-local output: drop a nested cacheable header's tokens from
        // the *parent* buffer after the child has been cached. The child's
        // IR is merged at index time (PCH-style) instead of re-parsed in
        // every consumer.
        if !self.opts.inline_include_bodies
            && !self.opts.frozen_expansion_cache
            && Self::is_cacheable_header(&include_path)
        {
            self.output.truncate(live_at);
            self.line_map.truncate_at(live_at);
            if let Some(frame) = self.cache_frames.last_mut() {
                frame
                    .skips
                    .push((live_at, trace_ir::canonicalize(&include_path)));
            }
        }
        Ok(i)
    }

    /// Macro-expand tokens on a `#include` line until they form a header-name.
    fn expand_include_operand(&mut self, tokens: &[Token]) -> Result<Vec<Token>, PreprocessError> {
        let mut out = Vec::new();
        let mut i = 0;
        while i < tokens.len() {
            if matches!(tokens[i].kind, TokenKind::Newline | TokenKind::Eof) {
                i += 1;
                continue;
            }
            let TokenKind::Identifier(name) = &tokens[i].kind else {
                out.push(tokens[i].clone());
                i += 1;
                continue;
            };
            if tokens[i].is_hidden(name) {
                out.push(tokens[i].clone());
                i += 1;
                continue;
            }
            let Some(def) = self.macros.get(name).cloned() else {
                out.push(tokens[i].clone());
                i += 1;
                continue;
            };
            match def {
                MacroDef::Object { replacement } => {
                    if !self.push_expansion(tokens[i].line) {
                        out.push(tokens[i].clone());
                        i += 1;
                        continue;
                    }
                    let painted = Self::paint_replacement(&replacement, &tokens[i], name);
                    let nested = self.expand_include_operand(&painted)?;
                    self.pop_expansion();
                    out.extend(nested);
                    i += 1;
                }
                MacroDef::Function {
                    params,
                    replacement,
                    variadic,
                } if self.next_non_newline_is(tokens, i + 1, "(") => {
                    if !self.push_expansion(tokens[i].line) {
                        out.push(tokens[i].clone());
                        i += 1;
                        continue;
                    }
                    let origin = tokens[i].clone();
                    i += 1;
                    let args = match self.parse_macro_args(tokens, &mut i) {
                        Ok(a) => a,
                        Err(e) => {
                            self.pop_expansion();
                            return Err(e);
                        }
                    };
                    let expanded = apply_concatenation(substitute_macro(
                        name,
                        &origin,
                        &replacement,
                        &params,
                        &args,
                        variadic,
                    ));
                    let nested = self.expand_include_operand(&expanded)?;
                    self.pop_expansion();
                    out.extend(nested);
                }
                MacroDef::Function { .. } => {
                    out.push(tokens[i].clone());
                    i += 1;
                }
            }
        }
        Ok(out)
    }

    fn resolve_include(&self, path: &str) -> Result<PathBuf, PreprocessError> {
        let candidate = if path.starts_with('/') || path.contains('\\') {
            PathBuf::from(path)
        } else {
            self.current_file
                .parent()
                .unwrap_or(Path::new("."))
                .join(path)
        };
        if candidate.exists() {
            return Ok(candidate);
        }
        for inc in &self.opts.include_paths {
            let p = inc.join(path);
            if p.is_file() {
                return Ok(p);
            }
        }
        if let Some(index) = &self.opts.basename_index {
            if let Some(name) = Path::new(path).file_name().and_then(|n| n.to_str()) {
                if let Some(matches) = index.get(name) {
                    if matches.len() == 1 {
                        return Ok(matches[0].clone());
                    }
                }
            }
        }
        Err(PreprocessError::Message {
            message: format!("include file not found: {path}"),
        })
    }

    fn handle_define(&mut self, tokens: &[Token], mut i: usize) -> Result<usize, PreprocessError> {
        let name = self.read_directive_ident(tokens, &mut i)?;
        // `read_directive_ident` consumed exactly the name token.
        if let Some(open) = parameter_list_open(tokens, i - 1) {
            i = open + 1;
            let Some((params, variadic)) = self.parse_macro_param_list(tokens, &mut i) else {
                skip_directive_line(tokens, &mut i);
                return Ok(i);
            };
            let mut replacement = read_replacement_list(tokens, &mut i);
            // Normalize at define time: a named GNU variadic (`args...`)
            // whose body nevertheless spells `__VA_ARGS__` (gcc rejects the
            // mix; real corpora contain it) aliases the tail parameter, so
            // expansion needs only plain parameter lookups.
            if variadic {
                if let Some(tail) = params.last() {
                    if tail != "__VA_ARGS__" {
                        for tok in &mut replacement {
                            if matches!(&tok.kind, TokenKind::Identifier(n) if n == "__VA_ARGS__") {
                                tok.kind = TokenKind::Identifier(tail.clone());
                            }
                        }
                    }
                }
            }
            self.insert_macro(
                name,
                MacroDef::Function {
                    params,
                    replacement,
                    variadic,
                },
            );
            return Ok(i);
        }
        let replacement = read_replacement_list(tokens, &mut i);
        self.insert_macro(name, MacroDef::Object { replacement });
        Ok(i)
    }

    /// Parse the parameter list after `NAME(`. `None` means the list was
    /// unterminated or malformed: a warning has been emitted and the caller
    /// drops the definition (gcc reports an error and keeps preprocessing).
    fn parse_macro_param_list(
        &mut self,
        tokens: &[Token],
        i: &mut usize,
    ) -> Option<(Vec<String>, bool)> {
        let mut params = Vec::new();
        let mut variadic = false;
        loop {
            skip_param_ws(tokens, i);
            if self.token_is_ellipsis(tokens, *i) {
                // Anonymous `...`: register the variadic under its standard
                // name so substitution, `##` comma elision, and the
                // "last param collects the rest" rule all treat it exactly
                // like a named `args...` variadic.
                variadic = true;
                params.push("__VA_ARGS__".to_string());
                *i = self.skip_ellipsis(tokens, *i);
                return self
                    .finish_param_list_tail(tokens, i)
                    .then_some((params, variadic));
            }
            match tokens.get(*i).map(|t| &t.kind) {
                Some(TokenKind::Punct(s)) if s == ")" => {
                    *i += 1;
                    break;
                }
                Some(TokenKind::Identifier(name)) => {
                    params.push(name.clone());
                    *i += 1;
                }
                _ => return self.malformed_param_list(tokens, *i),
            }
            // Line splicing makes `args \`-newline-`...` equivalent to
            // `args...`, so skip continuations before the ellipsis check.
            skip_param_ws(tokens, i);
            if self.token_is_ellipsis(tokens, *i) {
                variadic = true;
                *i = self.skip_ellipsis(tokens, *i);
                return self
                    .finish_param_list_tail(tokens, i)
                    .then_some((params, variadic));
            }
            match tokens.get(*i).map(|t| &t.kind) {
                Some(TokenKind::Punct(s)) if s == ")" => {
                    *i += 1;
                    break;
                }
                Some(TokenKind::Punct(s)) if s == "," => {
                    *i += 1;
                }
                _ => return self.malformed_param_list(tokens, *i),
            }
        }
        Some((params, variadic))
    }

    /// Warn about a parameter list that ends at a newline / EOF or contains
    /// an unexpected token, then yield `None` so the definition is dropped.
    fn malformed_param_list(&mut self, tokens: &[Token], i: usize) -> Option<(Vec<String>, bool)> {
        let line = tokens.get(i).map(|t| t.line).unwrap_or(1);
        let message = match tokens.get(i).map(|t| &t.kind) {
            None | Some(TokenKind::Eof) | Some(TokenKind::Newline) => {
                "unterminated macro parameter list; definition ignored"
            }
            _ => "expected , or ) in macro parameters; definition ignored",
        };
        self.warn(line, message);
        None
    }

    /// Consume the closing `)` after `...`. Tokens before it are dropped
    /// rather than leaked into the replacement list. Returns `false` (after
    /// warning) when the line ends first — the list must not run on to a
    /// `)` on a later line, which would swallow following code.
    fn finish_param_list_tail(&mut self, tokens: &[Token], i: &mut usize) -> bool {
        loop {
            skip_param_ws(tokens, i);
            match tokens.get(*i).map(|t| &t.kind) {
                Some(TokenKind::Punct(s)) if s == ")" => {
                    *i += 1;
                    return true;
                }
                None | Some(TokenKind::Eof) | Some(TokenKind::Newline) => {
                    return self.malformed_param_list(tokens, *i).is_some();
                }
                Some(_) => *i += 1,
            }
        }
    }

    fn token_is_ellipsis(&self, tokens: &[Token], i: usize) -> bool {
        matches!(&tokens.get(i).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == "...")
            || (matches!(&tokens.get(i).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == ".")
                && matches!(&tokens.get(i + 1).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == ".")
                && matches!(&tokens.get(i + 2).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == "."))
    }

    fn skip_ellipsis(&self, tokens: &[Token], i: usize) -> usize {
        if matches!(&tokens.get(i).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == "...") {
            i + 1
        } else {
            i + 3
        }
    }

    fn next_non_newline_is(&self, tokens: &[Token], mut i: usize, punct: &str) -> bool {
        while i < tokens.len() && matches!(tokens[i].kind, TokenKind::Newline) {
            i += 1;
        }
        matches!(
            tokens.get(i).map(|t| &t.kind),
            Some(TokenKind::Punct(s)) if s == punct
        )
    }

    fn parse_macro_args(
        &mut self,
        tokens: &[Token],
        i: &mut usize,
    ) -> Result<MacroArgs, PreprocessError> {
        while *i < tokens.len() && matches!(tokens[*i].kind, TokenKind::Newline) {
            *i += 1;
        }
        if !matches!(tokens.get(*i).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == "(") {
            return Ok(MacroArgs::default());
        }
        *i += 1;
        let mut args = MacroArgs::default();
        let mut current: Vec<Token> = Vec::new();
        let mut depth = 0u32;
        // Carries a tight `\`-newline run (see below) onto the token that
        // follows it, which is the only place its zero width is observable.
        let mut spliced_before = false;
        // End of the last token consumed, whichever argument it went to: a
        // top-level `,` empties `current`, and a splice right after it must
        // still be measured against the `,`.
        let mut last_end: Option<(u32, u32)> = None;
        while *i < tokens.len() {
            if is_line_continuation(tokens, *i) {
                // Phase 2 deletes `\`-newline before tokenizing (C11
                // 5.1.1.2p1), so a continuation is zero-width: with no gap
                // on either side its neighbours are adjacent in the spliced
                // source and `#` must spell them with no space, while a
                // space before the `\` or after the newline is real
                // whitespace. Walking a run of them tracks the position the
                // next character would occupy, exactly as
                // `parameter_list_open` does for the `(` that makes a
                // `#define` function-like.
                let mut adjacent = last_end;
                while is_line_continuation(tokens, *i) {
                    if adjacent != Some((tokens[*i].line, tokens[*i].col)) {
                        adjacent = None;
                    }
                    *i += 2;
                    adjacent = adjacent.map(|(line, _)| (line + 1, 1));
                }
                spliced_before = matches!((adjacent, tokens.get(*i)),
                    (Some(pos), Some(next)) if (next.line, next.col) == pos);
                continue;
            }
            let mut tok = tokens[*i].clone();
            // Keep a flag the token already carries: during rescanning the
            // argument tokens come from an outer substitution, and the
            // splice they record was consumed by the outer invocation.
            tok.spliced_before |= std::mem::take(&mut spliced_before);
            last_end = Some(token_end(&tok));
            match &tok.kind {
                TokenKind::Punct(s) if s == "(" => {
                    depth += 1;
                    current.push(tok);
                    *i += 1;
                }
                TokenKind::Punct(s) if s == ")" && depth == 0 => {
                    args.args.push(current);
                    *i += 1;
                    break;
                }
                TokenKind::Punct(s) if s == ")" => {
                    depth -= 1;
                    current.push(tok);
                    *i += 1;
                }
                TokenKind::Punct(s) if s == "," && depth == 0 => {
                    args.args.push(current);
                    args.separators.push(tok);
                    current = Vec::new();
                    *i += 1;
                }
                TokenKind::Eof => {
                    return Err(self.error(tok.line, "unterminated macro argument list"));
                }
                _ => {
                    current.push(tok);
                    *i += 1;
                }
            }
        }
        Ok(args)
    }

    fn read_directive_ident(
        &mut self,
        tokens: &[Token],
        i: &mut usize,
    ) -> Result<String, PreprocessError> {
        while *i < tokens.len() && matches!(tokens[*i].kind, TokenKind::Newline) {
            *i += 1;
        }
        match tokens.get(*i).map(|t| &t.kind) {
            Some(TokenKind::Identifier(s)) => {
                let name = s.clone();
                *i += 1;
                Ok(name)
            }
            _ => Err(self.error(
                tokens.get(*i).map(|t| t.line).unwrap_or(1),
                "expected identifier in directive",
            )),
        }
    }

    /// Evaluate the controlling expression of `#if` / `#elif`.
    ///
    /// `defined X` / `defined(X)` is resolved as an operator over the
    /// *unexpanded* tokens (C11 6.10.1p4: the operand of `defined` is never
    /// macro-expanded), object macros are expanded recursively (hide-set
    /// painted, depth-capped), and the result is parsed with C operator
    /// precedence by `eval_pp_tokens`.
    fn expand_and_eval_condition(&mut self, tokens: &[Token], i: &mut usize) -> bool {
        // The lexer does not splice `\`-newline, so conditions spanning
        // continuation lines must be stitched here (same handling as
        // `parse_macro_args`), else the tail tokens leak as ordinary output.
        let mut cond: Vec<Token> = Vec::new();
        while *i < tokens.len() {
            match &tokens[*i].kind {
                TokenKind::Newline | TokenKind::Eof => break,
                TokenKind::Punct(p)
                    if p == "\\"
                        && matches!(
                            tokens.get(*i + 1).map(|t| &t.kind),
                            Some(TokenKind::Newline)
                        ) =>
                {
                    *i += 2;
                }
                _ => {
                    cond.push(tokens[*i].clone());
                    *i += 1;
                }
            }
        }
        match self.expand_condition_tokens(&cond) {
            Some(expanded) => eval_pp_tokens(&expanded),
            None => false,
        }
    }

    /// Macro-expand condition tokens, treating `defined` as an operator
    /// whose operand is consumed unexpanded. `defined` introduced *by* an
    /// expansion is also resolved here (common `#define HAS_X defined(X)`
    /// pattern; undefined behavior per C11, resolved like gcc/clang do).
    /// Object and function-like macros expand on a flat worklist with
    /// rescanning, so an object macro naming a function-like macro still
    /// sees the `(args)` that follow it in the condition. Hide sets stop
    /// self-reference; explicit step/size budgets stop pathological growth
    /// (this engine bypasses the text path's `check_resource_limits`), in
    /// which case `None` is returned and the condition evaluates false.
    fn expand_condition_tokens(&mut self, toks: &[Token]) -> Option<Vec<Token>> {
        const MAX_TOKENS: usize = 1 << 16;
        const MAX_STEPS: u64 = 1 << 20;
        let mut work: Vec<Token> = toks.to_vec();
        let mut out: Vec<Token> = Vec::new();
        let mut i = 0usize;
        let mut steps: u64 = 0;
        while i < work.len() {
            steps += 1;
            if steps > MAX_STEPS || work.len() > MAX_TOKENS || out.len() > MAX_TOKENS {
                let line = work.get(i).map(|t| t.line).unwrap_or(1);
                self.warn(
                    line,
                    "macro expansion budget exceeded in #if condition; treating as false"
                        .to_string(),
                );
                return None;
            }
            let tok = work[i].clone();
            if let TokenKind::Identifier(name) = &tok.kind {
                if name == "defined" {
                    let (val, consumed) =
                        defined_operand(&work, i, &self.macros, &self.fallback_macros);
                    out.push(Token::new(
                        TokenKind::Number(if val { "1" } else { "0" }.into()),
                        tok.line,
                        tok.col,
                    ));
                    i += consumed;
                    continue;
                }
                if name == "__LINE__" {
                    out.push(Token::new(
                        TokenKind::Number(tok.expansion_site().0.to_string()),
                        tok.line,
                        tok.col,
                    ));
                    i += 1;
                    continue;
                }
                // A fallback must behave as undefined throughout conditional
                // evaluation: expanding it here (often to nothing) would
                // mangle the expression (`1 || __init` -> `1 ||`), while an
                // unexpanded identifier correctly evaluates to 0.
                if !tok.is_hidden(name) && !self.fallback_macros.contains(name.as_str()) {
                    match self.macros.get(name) {
                        Some(MacroDef::Object { replacement }) => {
                            let painted = Self::paint_replacement(replacement, &tok, name);
                            work.splice(i..i + 1, painted);
                            continue; // rescan at i
                        }
                        Some(MacroDef::Function {
                            params,
                            replacement,
                            variadic,
                        }) => {
                            if let Some((args, next)) = parse_cond_macro_args(&work, i + 1) {
                                let substituted = apply_concatenation(substitute_macro(
                                    name,
                                    &tok,
                                    replacement,
                                    params,
                                    &args,
                                    *variadic,
                                ));
                                work.splice(i..next, substituted);
                                continue; // rescan at i
                            }
                        }
                        None => {}
                    }
                }
            }
            out.push(tok);
            i += 1;
        }
        Some(out)
    }

    /// Advance past a condition's tokens, including `\`-newline
    /// continuations, without evaluating anything — used for `#if`/`#elif`
    /// in skipped groups (C11 6.10.1p6). Must consume exactly what
    /// `expand_and_eval_condition`'s collector would, or the conditional
    /// stack desyncs when a continuation line starts with a directive.
    fn skip_condition_tokens(&self, tokens: &[Token], i: &mut usize) {
        while *i < tokens.len() {
            match &tokens[*i].kind {
                TokenKind::Newline | TokenKind::Eof => break,
                TokenKind::Punct(p)
                    if p == "\\"
                        && matches!(
                            tokens.get(*i + 1).map(|t| &t.kind),
                            Some(TokenKind::Newline)
                        ) =>
                {
                    *i += 2;
                }
                _ => *i += 1,
            }
        }
    }

    fn skip_to_newline(&self, tokens: &[Token], mut i: usize) -> usize {
        while i < tokens.len() && !matches!(tokens[i].kind, TokenKind::Newline | TokenKind::Eof) {
            i += 1;
        }
        if i < tokens.len() && matches!(tokens[i].kind, TokenKind::Newline) {
            i += 1;
        }
        i
    }

    fn finish(self) -> PreprocessResult {
        PreprocessResult {
            output: self.output,
            line_map: self.line_map,
            diagnostics: self.diagnostics,
            included_headers: self.included_guard.into_iter().collect(),
        }
    }
}

fn parse_include_header(tokens: &[Token]) -> Option<String> {
    let mut i = 0;
    while i < tokens.len() && matches!(tokens[i].kind, TokenKind::Newline) {
        i += 1;
    }
    match tokens.get(i).map(|t| &t.kind) {
        Some(TokenKind::String(s)) => Some(s.clone()),
        Some(TokenKind::Punct(s)) if s == "<" => {
            let mut header = String::new();
            i += 1;
            while i < tokens.len() {
                match &tokens[i].kind {
                    TokenKind::Identifier(s) | TokenKind::Number(s) | TokenKind::Punct(s)
                        if s != ">" =>
                    {
                        header.push_str(s);
                    }
                    TokenKind::Punct(s) if s == ">" => return Some(header),
                    _ => return None,
                }
                i += 1;
            }
            None
        }
        _ => None,
    }
}

fn at_beginning_of_line(tokens: &[Token], i: usize) -> bool {
    if i == 0 {
        return true;
    }
    matches!(tokens[i - 1].kind, TokenKind::Newline)
}

/// Skip `\`-newline continuations inside a parameter list. A bare newline is
/// the end of the directive and is deliberately not skipped.
fn skip_param_ws(tokens: &[Token], i: &mut usize) {
    while is_line_continuation(tokens, *i) {
        *i += 2;
    }
}

/// Advance to the end of the current directive line (continuations
/// included), leaving `i` on the newline / EOF token.
fn skip_directive_line(tokens: &[Token], i: &mut usize) {
    while *i < tokens.len() && !matches!(tokens[*i].kind, TokenKind::Newline | TokenKind::Eof) {
        *i += if is_line_continuation(tokens, *i) {
            2
        } else {
            1
        };
    }
}

/// A `\` token followed by a newline token — a line continuation the lexer
/// does not splice, so token consumers skip the pair.
fn is_line_continuation(tokens: &[Token], i: usize) -> bool {
    matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == "\\")
        && matches!(tokens.get(i + 1).map(|t| &t.kind), Some(TokenKind::Newline))
}

/// Whether `idx` is the variadic collector — by construction always the
/// last parameter (`parse_macro_param_list` names an anonymous `...`
/// "__VA_ARGS__"; see the invariant on `MacroDef::Function`).
fn is_variadic_tail(params: &[String], variadic: bool, idx: usize) -> bool {
    variadic && idx + 1 == params.len()
}

fn arg_is_blank(arg: &[Token]) -> bool {
    arg.iter().all(|t| matches!(t.kind, TokenKind::Newline))
}

/// Whether the arguments from `idx` on carry no real tokens (absent, or
/// whitespace/newline only). Allocation-free — this runs on every `##`
/// parameter operand.
fn args_are_blank(args: &[Vec<Token>], idx: usize) -> bool {
    args.iter().skip(idx).all(|a| arg_is_blank(a))
}

/// GNU `, ## __VA_ARGS__` deletes the comma only when the variable
/// arguments are OMITTED — an explicitly supplied empty argument keeps it
/// (`F(1)` -> `g(1)` but `F(1,)` -> `g(1,)`; verified against gcc and
/// clang, whose behavior is stricter than the manual's "omitted or empty"
/// wording). A lone blank argument (`G()`, `G( )`) supplies zero
/// arguments, not one empty one.
fn varargs_omitted(args: &[Vec<Token>], idx: usize) -> bool {
    if args.len() <= idx {
        return true;
    }
    idx == 0 && args.len() == 1 && arg_is_blank(&args[0])
}

/// Index of the `(` that opens a function-like macro's parameter list, if
/// the token after the macro name at `name_idx` is one. C11 6.10.3p10: the
/// definition is function-like only when `(` immediately follows the macro
/// name with no intervening whitespace, so `#define ALIAS (VALUE)` and
/// `#define HALF (.5)` are object macros whose replacement starts with `(`.
/// Tokens carry no whitespace, so adjacency is decided from positions (the
/// lexer counts columns per character). A `\`-newline pair is deleted in
/// translation phase 2 and therefore zero-width: `F\`-newline-`(x)` is
/// function-like when `(` starts the next line.
fn parameter_list_open(tokens: &[Token], name_idx: usize) -> Option<usize> {
    let name = &tokens[name_idx];
    let TokenKind::Identifier(ident) = &name.kind else {
        return None;
    };
    let (mut line, mut col) = (name.line, name.col + ident.chars().count() as u32);
    let mut i = name_idx + 1;
    while is_line_continuation(tokens, i) && tokens[i].line == line && tokens[i].col == col {
        i += 2;
        line += 1;
        col = 1;
    }
    let next = tokens.get(i)?;
    let adjacent = next.line == line && next.col == col;
    (adjacent && matches!(&next.kind, TokenKind::Punct(s) if s == "(")).then_some(i)
}

/// Collect a `#define` replacement list up to the end of the line, splicing
/// `\`-newline continuations.
fn read_replacement_list(tokens: &[Token], i: &mut usize) -> Vec<Token> {
    let mut replacement = Vec::new();
    while *i < tokens.len() && !matches!(tokens[*i].kind, TokenKind::Newline) {
        if is_line_continuation(tokens, *i) {
            *i += 2;
            continue;
        }
        replacement.push(tokens[*i].clone());
        *i += 1;
    }
    replacement
}

/// The arguments of one function-like macro invocation. `separators[k]` is
/// the top-level `,` token between `args[k]` and `args[k + 1]`; keeping it
/// lets the variadic collector be re-spelled exactly as written
/// (`#__VA_ARGS__` on `F(p,q)` is `"p,q"`, on `F(p , q)` it is `"p , q"`).
#[derive(Debug, Default)]
struct MacroArgs {
    args: Vec<Vec<Token>>,
    separators: Vec<Token>,
}

impl MacroArgs {
    /// The variadic collector's tokens from `idx` on, commas included, in
    /// source order — the one argument C11 6.10.3p12 says they form.
    fn variadic_tokens(&self, idx: usize) -> Vec<Token> {
        let mut out = Vec::new();
        for (ai, arg) in self.args.iter().enumerate().skip(idx) {
            if ai > idx {
                out.push(self.separators[ai - 1].clone());
            }
            out.extend(arg.iter().cloned());
        }
        out
    }
}

fn substitute_macro(
    macro_name: &str,
    origin: &Token,
    body: &[Token],
    params: &[String],
    args: &MacroArgs,
    variadic: bool,
) -> Vec<Token> {
    debug_assert!(
        !variadic || !params.is_empty(),
        "a variadic MacroDef must name its tail parameter \
         (\"__VA_ARGS__\" for the anonymous form; see parse_macro_param_list)"
    );
    let mut out: Vec<Token> = Vec::new();
    let mut i = 0;
    while i < body.len() {
        let concat_width = concat_width_at(body, i);
        if concat_width > 0 && i + concat_width < body.len() {
            if let TokenKind::Identifier(name) = &body[i + concat_width].kind {
                if let Some(idx) = params.iter().position(|p| p == name) {
                    let is_va_tail = is_variadic_tail(params, variadic, idx);
                    if is_va_tail
                        && matches!(
                            out.last().map(|t| &t.kind),
                            Some(TokenKind::Punct(s)) if s == ","
                        )
                    {
                        // GNU `, ## args`: with the varargs omitted the
                        // comma is deleted; otherwise the `##` is inert — it
                        // must NOT reach apply_concatenation, which would
                        // fuse the comma with the first vararg token
                        // (destroying string/char literals and breaking
                        // rescan). An explicitly empty argument substitutes
                        // to nothing and the comma stays, like gcc.
                        if varargs_omitted(&args.args, idx) {
                            out.pop();
                            i += concat_width + 1;
                        } else {
                            i += concat_width;
                        }
                        continue;
                    }
                    let blank = if is_va_tail {
                        args_are_blank(&args.args, idx)
                    } else {
                        args.args.get(idx).is_none_or(|a| arg_is_blank(a))
                    };
                    if blank {
                        // C99 placemarker: an empty `##` operand makes the
                        // paste a no-op; the left operand stays as-is.
                        i += concat_width + 1;
                        continue;
                    }
                }
            }
        }
        if matches!(body[i].kind, TokenKind::Hash) {
            if let Some(TokenKind::Identifier(name)) = body.get(i + 1).map(|t| &t.kind) {
                if let Some(idx) = params.iter().position(|p| p == name) {
                    // C11 6.10.3.2: `#param` becomes a string literal
                    // spelling the argument as written (not expanded). Like
                    // every replacement-list token it keeps the definition
                    // coordinates and attributes to the expansion site via
                    // `with_macro_hide`.
                    let text = if is_variadic_tail(params, variadic, idx) {
                        stringize_spelling(&args.variadic_tokens(idx))
                    } else {
                        args.args
                            .get(idx)
                            .map(|a| stringize_spelling(a))
                            .unwrap_or_default()
                    };
                    out.push(
                        Token::new(TokenKind::String(text), body[i].line, body[i].col)
                            .with_macro_hide(origin, macro_name),
                    );
                    i += 2;
                    continue;
                }
            }
        }
        if let TokenKind::Identifier(name) = &body[i].kind {
            // An anonymous `...` registers `__VA_ARGS__` as the last param
            // (see parse_macro_param_list), and handle_define rewrites a
            // stray `__VA_ARGS__` in a named-variadic body to the tail
            // param, so plain position lookup covers both variadic styles.
            if let Some(idx) = params.iter().position(|p| p == name) {
                if is_variadic_tail(params, variadic, idx) {
                    out.extend(args.variadic_tokens(idx));
                } else if let Some(arg) = args.args.get(idx) {
                    out.extend(arg.iter().cloned());
                }
                i += 1;
                continue;
            }
        }
        // Replacement-list tokens (not from arguments) inherit the hide set.
        out.push(body[i].with_macro_hide(origin, macro_name));
        i += 1;
    }
    out
}

/// Apply `##` token pasting after parameter substitution.
fn apply_concatenation(mut tokens: Vec<Token>) -> Vec<Token> {
    loop {
        let mut next: Vec<Token> = Vec::new();
        let mut pasted = false;
        let mut i = 0;
        while i < tokens.len() {
            let width = concat_width_at(&tokens, i);
            if width > 0 {
                // Paste the previously emitted token with the operand after
                // `##`, so chains (`a ## b ## c`) collapse left to right. A
                // dangling `##` with no operand on either side is dropped.
                if let Some(left) = next.pop() {
                    if i + width < tokens.len() {
                        next.push(paste_two_tokens(&left, &tokens[i + width]));
                        i += width + 1;
                        pasted = true;
                    } else {
                        next.push(left);
                        i += width;
                    }
                } else {
                    i += width;
                }
            } else {
                next.push(tokens[i].clone());
                i += 1;
            }
        }
        // Rescan only when a paste ran: pasting `#` with `#` can re-form a
        // `##` operator; merely dropping a dangling `##` cannot.
        if !pasted {
            return next;
        }
        tokens = next;
    }
}

fn concat_width_at(tokens: &[Token], i: usize) -> usize {
    if matches!(&tokens[i].kind, TokenKind::Punct(s) if s == "##") {
        return 1;
    }
    if matches!(&tokens[i].kind, TokenKind::Hash)
        && i + 1 < tokens.len()
        && matches!(tokens[i + 1].kind, TokenKind::Hash)
    {
        return 2;
    }
    0
}

/// Fallback definitions for macros whose real definitions live in headers the
/// indexed tree does not ship (gtest, kernel headers, `<inttypes.h>`). Left
/// unexpanded they produce tree-sitter ERROR nodes and whole functions get
/// dropped from the index (docs/PARSE_FAILURES.md catalogs the impact).
/// Built once; `install_builtin_macros` clones entries per preprocess.
static BUILTIN_FALLBACK_MACROS: LazyLock<Vec<(String, MacroDef)>> = LazyLock::new(|| {
    let object = |name: &str, replacement: &str| {
        (
            name.to_string(),
            MacroDef::Object {
                replacement: lex_macro_body(replacement),
            },
        )
    };
    let function = |name: &str, params: &[&str], replacement: &str| {
        (
            name.to_string(),
            MacroDef::Function {
                params: params.iter().map(|s| s.to_string()).collect(),
                replacement: lex_macro_body(replacement),
                variadic: false,
            },
        )
    };
    let mut table = Vec::new();
    // GNU/MSVC unused-parameter markers. Without this, an undefined
    // `__UNUSED` after a reference declarator (`T &event __UNUSED`) is
    // parsed as a broken `declaration` and the function body is dropped.
    table.push(object("__UNUSED", ""));
    // Linux kernel address-space / section annotations: `char __user *buf`
    // and `int __init foo(void)` are syntax errors when unexpanded.
    for name in [
        "__user",
        "__iomem",
        "__percpu",
        "__rcu",
        "__force",
        "__init",
        "__exit",
        "__initdata",
        "__exitdata",
        "__read_mostly",
    ] {
        table.push(object(name, ""));
    }
    // <inttypes.h> format-specifier strings: `"%" PRIu64` must expand to
    // a string literal or the adjacent-literal concatenation mis-parses.
    for (width, prefix) in [("8", "hh"), ("16", "h"), ("32", ""), ("64", "ll")] {
        for conv in ["d", "i", "u", "x", "X", "o"] {
            table.push(object(
                &format!("PRI{conv}{width}"),
                &format!("\"{prefix}{conv}\""),
            ));
        }
    }
    // `container_of(ptr, struct T, member)` puts a type keyword in
    // expression position; keep the pointer flow and the target type. The
    // `member` argument is deliberately dropped: an offsetof-shaped body
    // yields no additional call/flow facts and routes the pointer through
    // arithmetic the flow analysis tracks less precisely.
    table.push(function(
        "container_of",
        &["ptr", "type", "member"],
        "( ( type * ) ( void * ) ( ptr ) )",
    ));
    // gtest/OpenHarmony test macros: `HWTEST_F(Suite, Name, TestSize.Level1)`
    // followed by a body is unparseable unexpanded and every test body is
    // lost. Expand to a plain function definition so bodies get indexed.
    for name in ["HWTEST", "HWTEST_F", "HWTEST_P"] {
        table.push(function(
            name,
            &["a", "b", "level"],
            "static void a ## _ ## b ()",
        ));
    }
    table
});

fn paste_two_tokens(left: &Token, right: &Token) -> Token {
    let text = format!(
        "{}{}",
        token_paste_fragment(&left.kind),
        token_paste_fragment(&right.kind)
    );
    Token {
        kind: TokenKind::Identifier(text),
        line: left.line,
        col: left.col,
        hidden: Token::union_hidden(left, right),
        origin: left.origin.or(right.origin),
        // Whatever separated `left` from the token before it still
        // separates the pasted result from it.
        spliced_before: left.spliced_before,
    }
}

fn token_paste_fragment(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Identifier(s) => s.clone(),
        TokenKind::Number(s) => s.clone(),
        TokenKind::Punct(s) if s != "##" => s.clone(),
        _ => String::new(),
    }
}

fn needs_leading_space(output: &str, kind: &TokenKind) -> bool {
    if output.is_empty() {
        return false;
    }
    if output.ends_with('\n') {
        return false;
    }
    let last = output.chars().last().unwrap();
    if last == ' ' {
        return false;
    }
    match kind {
        // Closing `)` / `]` must not gain a space (`operator()`, `foo[]`).
        // After a template `>`, a space before `&` / `*` keeps
        // `shared_ptr<T> &p` from gluing into `>&` which tree-sitter
        // fails to parse as a reference parameter.
        TokenKind::Punct(s) => match s.as_str() {
            ";" | "," | "}" | "::" | "." => true,
            "&" | "*" => last == '>',
            _ => false,
        },
        TokenKind::Newline => false,
        _ => !matches!(last, '(' | '[' | '{' | '.' | ';'),
    }
}

/// Body of the string literal `#param` produces for one argument (C11
/// 6.10.3.2p2): each token's spelling, whitespace between tokens collapsed
/// to a single space, leading/trailing whitespace dropped, and `"` / `\`
/// inside string and character literals escaped. Tokens carry no
/// whitespace, so "was there whitespace" is decided from positions: a token
/// that does not start exactly where the previous one ended (or that sits
/// on a later line) was separated by whitespace.
fn stringize_spelling(arg: &[Token]) -> String {
    let mut text = String::new();
    let mut prev_end: Option<(u32, u32)> = None;
    for tok in arg {
        let spelling = match &tok.kind {
            TokenKind::Newline | TokenKind::Eof => continue,
            TokenKind::Identifier(s) | TokenKind::Number(s) | TokenKind::Punct(s) => s.clone(),
            TokenKind::Hash => "#".to_string(),
            TokenKind::String(s) => format!("\\\"{}\\\"", escape_for_stringize(s)),
            TokenKind::Char(s) => format!("'{}'", escape_for_stringize(s)),
        };
        if let Some((line, col)) = prev_end {
            // A `\`-newline splice is deleted in phase 2, so a token it
            // alone separates from its predecessor is adjacent in the
            // spliced source (`a\`+newline+`b` spells "ab") even though the
            // positions differ; `parse_macro_args` flags exactly that case.
            if !tok.spliced_before && (tok.line != line || tok.col != col) {
                text.push(' ');
            }
        }
        text.push_str(&spelling);
        prev_end = Some(token_end(tok));
    }
    text
}

/// Where the character after `tok` sits in the source: the lexer counts one
/// column per character, and string/char literals carry their delimiters.
/// Shared with `parse_macro_args`, which compares this against a `\`-newline's
/// position to decide whether the splice was tight.
fn token_end(tok: &Token) -> (u32, u32) {
    let width = match &tok.kind {
        TokenKind::Identifier(s) | TokenKind::Number(s) | TokenKind::Punct(s) => s.chars().count(),
        TokenKind::String(s) | TokenKind::Char(s) => s.chars().count() + 2,
        TokenKind::Hash => 1,
        TokenKind::Newline | TokenKind::Eof => 0,
    };
    (tok.line, tok.col + width as u32)
}

fn escape_for_stringize(literal_body: &str) -> String {
    let mut out = String::with_capacity(literal_body.len());
    for c in literal_body.chars() {
        if matches!(c, '\\' | '"') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

fn token_to_string(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Identifier(s) => s.clone(),
        TokenKind::Number(s) => s.clone(),
        TokenKind::String(s) => format!("\"{s}\""),
        TokenKind::Char(s) => format!("'{s}'"),
        TokenKind::Punct(s) => s.clone(),
        TokenKind::Hash => "#".to_string(),
        TokenKind::Newline => "\n".to_string(),
        TokenKind::Eof => String::new(),
    }
}

/// Collect a function-like macro invocation's arguments from condition
/// tokens. `i` points just past the macro name; returns the argument token
/// lists and the index after the closing `)`, or `None` when the next token
/// is not `(` (uninvoked name) or the list is unterminated.
fn parse_cond_macro_args(toks: &[Token], mut i: usize) -> Option<(MacroArgs, usize)> {
    if !matches!(toks.get(i).map(|t| &t.kind), Some(TokenKind::Punct(p)) if p == "(") {
        return None;
    }
    i += 1;
    let mut args = MacroArgs::default();
    let mut current: Vec<Token> = Vec::new();
    let mut depth = 0u32;
    while i < toks.len() {
        match &toks[i].kind {
            TokenKind::Punct(p) if p == "(" => {
                depth += 1;
                current.push(toks[i].clone());
            }
            TokenKind::Punct(p) if p == ")" && depth == 0 => {
                args.args.push(current);
                return Some((args, i + 1));
            }
            TokenKind::Punct(p) if p == ")" => {
                depth -= 1;
                current.push(toks[i].clone());
            }
            TokenKind::Punct(p) if p == "," && depth == 0 => {
                args.args.push(current);
                args.separators.push(toks[i].clone());
                current = Vec::new();
            }
            _ => current.push(toks[i].clone()),
        }
        i += 1;
    }
    None
}

/// Resolve one `defined X` / `defined(X)` operator at `toks[i]`
/// (which is the `defined` identifier). Returns the truth value and how
/// many tokens the operator consumed; malformed operands conservatively
/// evaluate to false.
fn defined_operand(
    toks: &[Token],
    i: usize,
    macros: &MacroTable,
    fallbacks: &HashSet<String>,
) -> (bool, usize) {
    let is_defined = |n: &str| macros.contains_key(n) && !fallbacks.contains(n);
    match toks.get(i + 1).map(|t| &t.kind) {
        Some(TokenKind::Punct(p)) if p == "(" => {
            if let (Some(TokenKind::Identifier(n)), Some(TokenKind::Punct(c))) = (
                toks.get(i + 2).map(|t| &t.kind),
                toks.get(i + 3).map(|t| &t.kind),
            ) {
                if c == ")" {
                    return (is_defined(n), 4);
                }
            }
            (false, 2)
        }
        Some(TokenKind::Identifier(n)) => (is_defined(n), 2),
        _ => (false, 1),
    }
}

/// Evaluate a fully expanded `#if` condition with C operator precedence.
/// Identifiers that survived expansion evaluate to 0 (C11 6.10.1p4), with
/// `true`/`false` as boolean literals (C++/C23; also the previous
/// evaluator's behavior); an unexpanded function-like call form
/// `ident(...)` swallows its argument list and evaluates to 0. Errors are
/// conservative: malformed input (parse error or trailing tokens) yields
/// false (branch skipped).
fn eval_pp_tokens(toks: &[Token]) -> bool {
    let mut p = PpExprParser {
        toks,
        pos: 0,
        err: false,
    };
    let v = p.ternary();
    // Malformed input is conservative: a parse error or unconsumed trailing
    // tokens must not activate a branch.
    if p.err || p.pos != p.toks.len() {
        return false;
    }
    v.truthy()
}

/// A preprocessor arithmetic value: 64-bit two's-complement bits plus the
/// C signedness of the expression, modeling intmax_t/uintmax_t evaluation
/// (C11 6.10.1p4). Binary operators apply the usual arithmetic
/// conversions: if either operand is unsigned the operation is unsigned
/// (so `-1 < 1U` is false — the -1 converts to uintmax_t).
#[derive(Clone, Copy)]
struct PpVal {
    bits: u64,
    unsigned_: bool,
}

impl PpVal {
    fn signed(v: i64) -> Self {
        Self {
            bits: v as u64,
            unsigned_: false,
        }
    }

    fn from_bool(b: bool) -> Self {
        Self::signed(b as i64)
    }

    fn truthy(self) -> bool {
        self.bits != 0
    }

    fn as_i64(self) -> i64 {
        self.bits as i64
    }

    fn either_unsigned(self, other: Self) -> bool {
        self.unsigned_ || other.unsigned_
    }
}

struct PpExprParser<'a> {
    toks: &'a [Token],
    pos: usize,
    /// Set on any syntax error (missing `)`/`:`, dangling operator,
    /// unterminated call form, non-expression token).
    err: bool,
}

impl<'a> PpExprParser<'a> {
    fn peek_punct(&self) -> Option<&str> {
        match self.toks.get(self.pos).map(|t| &t.kind) {
            Some(TokenKind::Punct(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    fn eat(&mut self, p: &str) -> bool {
        if self.peek_punct() == Some(p) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn ternary(&mut self) -> PpVal {
        let c = self.logical_or();
        if self.eat("?") {
            let a = self.ternary();
            let b = if self.eat(":") {
                self.ternary()
            } else {
                self.err = true;
                PpVal::signed(0)
            };
            // The result type is the common type of BOTH arms (usual
            // arithmetic conversions), regardless of which arm is taken:
            // `1 ? -1 : 1U` is unsigned.
            let chosen = if c.truthy() { a } else { b };
            return PpVal {
                bits: chosen.bits,
                unsigned_: a.either_unsigned(b),
            };
        }
        c
    }

    fn logical_or(&mut self) -> PpVal {
        let mut v = self.logical_and();
        while self.eat("||") {
            let r = self.logical_and();
            v = PpVal::from_bool(v.truthy() || r.truthy());
        }
        v
    }

    fn logical_and(&mut self) -> PpVal {
        let mut v = self.bit_or();
        while self.eat("&&") {
            let r = self.bit_or();
            v = PpVal::from_bool(v.truthy() && r.truthy());
        }
        v
    }

    fn bit_or(&mut self) -> PpVal {
        let mut v = self.bit_xor();
        while self.peek_punct() == Some("|") {
            self.pos += 1;
            let r = self.bit_xor();
            v = PpVal {
                bits: v.bits | r.bits,
                unsigned_: v.either_unsigned(r),
            };
        }
        v
    }

    fn bit_xor(&mut self) -> PpVal {
        let mut v = self.bit_and();
        while self.eat("^") {
            let r = self.bit_and();
            v = PpVal {
                bits: v.bits ^ r.bits,
                unsigned_: v.either_unsigned(r),
            };
        }
        v
    }

    fn bit_and(&mut self) -> PpVal {
        let mut v = self.equality();
        while self.peek_punct() == Some("&") {
            self.pos += 1;
            let r = self.equality();
            v = PpVal {
                bits: v.bits & r.bits,
                unsigned_: v.either_unsigned(r),
            };
        }
        v
    }

    fn equality(&mut self) -> PpVal {
        let mut v = self.relational();
        loop {
            if self.eat("==") {
                v = PpVal::from_bool(v.bits == self.relational().bits);
            } else if self.eat("!=") {
                v = PpVal::from_bool(v.bits != self.relational().bits);
            } else {
                return v;
            }
        }
    }

    fn relational(&mut self) -> PpVal {
        // Comparisons follow the converted common type: unsigned if either
        // side is unsigned, else signed.
        fn lt(a: PpVal, b: PpVal) -> bool {
            if a.either_unsigned(b) {
                a.bits < b.bits
            } else {
                a.as_i64() < b.as_i64()
            }
        }
        let mut v = self.shift();
        loop {
            if self.eat("<=") {
                let r = self.shift();
                v = PpVal::from_bool(!lt(r, v));
            } else if self.eat(">=") {
                let r = self.shift();
                v = PpVal::from_bool(!lt(v, r));
            } else if self.eat("<") {
                let r = self.shift();
                v = PpVal::from_bool(lt(v, r));
            } else if self.eat(">") {
                let r = self.shift();
                v = PpVal::from_bool(lt(r, v));
            } else {
                return v;
            }
        }
    }

    fn shift(&mut self) -> PpVal {
        // Result type follows the left operand; `>>` is logical for
        // unsigned, arithmetic for signed. Amounts are masked to 0..63
        // (out-of-range shifts are UB in C).
        let mut v = self.additive();
        loop {
            if self.eat("<<") {
                let sh = self.additive().bits as u32 & 63;
                v = PpVal {
                    bits: v.bits.wrapping_shl(sh),
                    unsigned_: v.unsigned_,
                };
            } else if self.eat(">>") {
                let sh = self.additive().bits as u32 & 63;
                v = PpVal {
                    bits: if v.unsigned_ {
                        v.bits.wrapping_shr(sh)
                    } else {
                        v.as_i64().wrapping_shr(sh) as u64
                    },
                    unsigned_: v.unsigned_,
                };
            } else {
                return v;
            }
        }
    }

    fn additive(&mut self) -> PpVal {
        let mut v = self.multiplicative();
        loop {
            if self.eat("+") {
                let r = self.multiplicative();
                v = PpVal {
                    bits: v.bits.wrapping_add(r.bits),
                    unsigned_: v.either_unsigned(r),
                };
            } else if self.eat("-") {
                let r = self.multiplicative();
                v = PpVal {
                    bits: v.bits.wrapping_sub(r.bits),
                    unsigned_: v.either_unsigned(r),
                };
            } else {
                return v;
            }
        }
    }

    fn multiplicative(&mut self) -> PpVal {
        let mut v = self.unary();
        loop {
            if self.eat("*") {
                let r = self.unary();
                v = PpVal {
                    bits: v.bits.wrapping_mul(r.bits),
                    unsigned_: v.either_unsigned(r),
                };
            } else if self.eat("/") {
                let r = self.unary();
                v = self.divide(v, r, false);
            } else if self.eat("%") {
                let r = self.unary();
                v = self.divide(v, r, true);
            } else {
                return v;
            }
        }
    }

    /// `/` and `%` under the usual arithmetic conversions; division by
    /// zero conservatively yields 0.
    fn divide(&mut self, a: PpVal, b: PpVal, rem: bool) -> PpVal {
        let unsigned_ = a.either_unsigned(b);
        if b.bits == 0 {
            return PpVal { bits: 0, unsigned_ };
        }
        let bits = if unsigned_ {
            if rem {
                a.bits % b.bits
            } else {
                a.bits / b.bits
            }
        } else if rem {
            a.as_i64().wrapping_rem(b.as_i64()) as u64
        } else {
            a.as_i64().wrapping_div(b.as_i64()) as u64
        };
        PpVal { bits, unsigned_ }
    }

    fn unary(&mut self) -> PpVal {
        if self.eat("!") {
            return PpVal::from_bool(!self.unary().truthy());
        }
        if self.eat("~") {
            let v = self.unary();
            return PpVal {
                bits: !v.bits,
                unsigned_: v.unsigned_,
            };
        }
        if self.eat("-") {
            // Negation keeps the operand's signedness (`-1U` stays
            // unsigned in C and wraps).
            let v = self.unary();
            return PpVal {
                bits: v.bits.wrapping_neg(),
                unsigned_: v.unsigned_,
            };
        }
        if self.eat("+") {
            return self.unary();
        }
        self.primary()
    }

    fn primary(&mut self) -> PpVal {
        let Some(tok) = self.toks.get(self.pos) else {
            // Dangling operator with no operand.
            self.err = true;
            return PpVal::signed(0);
        };
        match &tok.kind {
            TokenKind::Number(s) => {
                let v = parse_pp_int(s);
                self.pos += 1;
                v
            }
            TokenKind::Char(s) => {
                let v = PpVal::signed(char_value(s));
                self.pos += 1;
                v
            }
            TokenKind::Punct(p) if p == "(" => {
                self.pos += 1;
                let v = self.ternary();
                if !self.eat(")") {
                    self.err = true;
                }
                v
            }
            TokenKind::Identifier(name) => {
                // C++ / C23 boolean literals (also matches the previous
                // evaluator's behavior for `#if true`).
                if name == "true" {
                    self.pos += 1;
                    return PpVal::signed(1);
                }
                if name == "false" {
                    self.pos += 1;
                    return PpVal::signed(0);
                }
                self.pos += 1;
                // Unexpanded function-like form: swallow the balanced
                // argument list so the caller's operator loop resumes
                // cleanly after it.
                if self.peek_punct() == Some("(") {
                    let mut depth = 0i32;
                    let mut closed = false;
                    while let Some(t) = self.toks.get(self.pos) {
                        match &t.kind {
                            TokenKind::Punct(p) if p == "(" => depth += 1,
                            TokenKind::Punct(p) if p == ")" => {
                                depth -= 1;
                                if depth == 0 {
                                    self.pos += 1;
                                    closed = true;
                                    break;
                                }
                            }
                            _ => {}
                        }
                        self.pos += 1;
                    }
                    if !closed {
                        self.err = true;
                    }
                }
                PpVal::signed(0)
            }
            // Non-expression token (string literal, stray punct): malformed.
            _ => {
                self.pos += 1;
                self.err = true;
                PpVal::signed(0)
            }
        }
    }
}

/// Parse a C preprocessor integer literal (decimal, hex, octal, binary,
/// with optional u/U/l/L suffixes). Unparseable text evaluates to 0.
/// The value is unsigned when it carries a `u`/`U` suffix or does not fit
/// in a signed 64-bit intmax_t (hex/octal ladder reaching uintmax_t).
fn parse_pp_int(s: &str) -> PpVal {
    let t = s.trim_end_matches(['u', 'U', 'l', 'L']);
    let unsigned_suffix = s[t.len()..].contains(['u', 'U']);
    let (digits, radix) = if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        (h, 16)
    } else if let Some(b) = t.strip_prefix("0b").or_else(|| t.strip_prefix("0B")) {
        (b, 2)
    } else if t.len() > 1 && t.starts_with('0') {
        (&t[1..], 8)
    } else {
        (t, 10)
    };
    match u128::from_str_radix(digits, radix) {
        Ok(v) => PpVal {
            bits: v as u64,
            unsigned_: unsigned_suffix || v > i64::MAX as u128,
        },
        Err(_) => PpVal::signed(0),
    }
}

/// Value of a character constant's content (quotes already stripped by the
/// lexer; escapes kept verbatim).
fn char_value(s: &str) -> i64 {
    let mut chars = s.chars().peekable();
    match chars.next() {
        Some('\\') => match chars.next() {
            Some('n') => 10,
            Some('t') => 9,
            Some('r') => 13,
            Some('a') => 7,
            Some('b') => 8,
            Some('f') => 12,
            Some('v') => 11,
            Some('\\') => 92,
            Some('\'') => 39,
            Some('"') => 34,
            Some('?') => 63,
            // \x… hexadecimal escape.
            Some('x') => {
                let mut v: i64 = 0;
                while let Some(d) = chars.peek().and_then(|c| c.to_digit(16)) {
                    v = v.wrapping_mul(16).wrapping_add(d as i64);
                    chars.next();
                }
                v
            }
            // \ooo octal escape (1-3 digits, first already consumed).
            Some(d @ '0'..='7') => {
                let mut v: i64 = d as i64 - '0' as i64;
                for _ in 0..2 {
                    match chars.peek().and_then(|c| c.to_digit(8)) {
                        Some(o) => {
                            v = v * 8 + o as i64;
                            chars.next();
                        }
                        None => break,
                    }
                }
                v
            }
            Some(c) => c as i64,
            None => 0,
        },
        Some(c) => c as i64,
        None => 0,
    }
}

pub fn preprocess_file(
    path: &Path,
    opts: &PreprocessOptions,
) -> Result<PreprocessResult, PreprocessError> {
    let mut state = PreprocessorState::new(opts.clone(), path.to_path_buf());
    state.process_file(path)?;
    Ok(state.finish())
}

pub fn preprocess_string(source: &str, file: &Path, opts: &PreprocessOptions) -> PreprocessResult {
    let mut state = PreprocessorState::new(opts.clone(), file.to_path_buf());
    let tokens = Lexer::new(source).tokenize();
    if let Err(e) = state.process_file_tokens(&tokens) {
        state.warn(1, format!("preprocess stopped: {e}"));
    }
    state.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::IncludeExpansion;
    use std::sync::{Arc, RwLock};

    #[test]
    fn expands_function_like_macro() {
        let src = "#define SQUARE(x) ((x) * (x))\nint y = SQUARE(n);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("((n") && result.output.contains("*"),
            "{}",
            result.output
        );
        assert!(!result.output.contains("SQUARE"));
    }

    #[test]
    fn expands_function_like_field_macro() {
        let src = "#define FIELD_P(o) ((o)->inner.p)\nFIELD_P(obj);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("inner") && result.output.contains("obj"),
            "{}",
            result.output
        );
        assert!(!result.output.contains("FIELD_P"));
    }

    #[test]
    fn expands_token_paste_concat() {
        let src = "#define CAT(a,b) a ## b\nint CAT(x, y);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("xy") || result.output.contains("x y"),
            "{}",
            result.output
        );
        assert!(!result.output.contains("CAT"));
    }

    #[test]
    fn expands_chained_token_paste() {
        let src = "#define CAT3(a,b,c) a ## b ## c\nint CAT3(x, y, z);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("xyz"), "{}", result.output);
    }

    #[test]
    fn expands_object_macro() {
        let opts = PreprocessOptions::new().with_define("NULL", "0");
        let result = preprocess_string("int *p = NULL;", Path::new("test.c"), &opts);
        assert!(result.output.contains("int") && result.output.contains("0"));
        assert!(!result.output.contains("NULL"));
    }

    /// Regression (#6): a body starting with `(.` is an object macro, not a
    /// function-like macro whose parameter list begins with `...`. The old
    /// classifier matched a bare `.` and then aborted the whole file when the
    /// parameter list failed to parse.
    #[test]
    fn object_macro_body_starting_with_dot_does_not_abort() {
        let src = "#define HALF (.5)\n#define ORIGIN (.x = 0, .y = 0)\nint x = HALF;\nstruct p o = ORIGIN;\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        let flat = result.output.replace([' ', '\n'], "");
        assert!(flat.contains("intx=(.5);"), "{}", result.output);
        assert!(flat.contains("o=(.x=0,.y=0);"), "{}", result.output);
        assert!(flat.contains("intafter;"), "{}", result.output);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    }

    /// Regression (#7): `#define ALIAS (VALUE)` is an object macro whose body
    /// is `(VALUE)`. Whitespace separates the `(` from the name, so it does
    /// not open a parameter list (C11 6.10.3p10).
    #[test]
    fn object_macro_parenthesized_identifier_expands() {
        let src = "#define VALUE 42\n#define ALIAS (VALUE)\nint x = ALIAS;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        let flat = result.output.replace([' ', '\n'], "");
        assert!(flat.contains("intx=(42);"), "{}", result.output);
        assert!(!result.output.contains("ALIAS"), "{}", result.output);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    }

    /// `#define F (x) x` is an object macro even though `(x)` would be a
    /// valid parameter list; `F(1)` therefore expands to `(x) x(1)` (gcc).
    #[test]
    fn function_like_macro_requires_paren_adjacent_to_name() {
        let src = "#define F (x) x\nint y = F(1);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        let flat = result.output.replace([' ', '\n'], "");
        assert!(flat.contains("inty=(x)x(1);"), "{}", result.output);
    }

    /// `\`-newline is spliced before the `(` adjacency test (translation
    /// phase 2), so `F\` + `(x)` on the next line is function-like, while a
    /// space before the `\` still separates the `(` from the name.
    #[test]
    fn function_like_macro_name_split_by_line_splice() {
        let src = "#define F\\\n(x) x\n#define G \\\n(x) x\nint a = F(7);\nint b = G(7);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        let flat = result.output.replace([' ', '\n'], "");
        assert!(flat.contains("inta=7;"), "{}", result.output);
        assert!(flat.contains("intb=(x)x(7);"), "{}", result.output);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    }

    #[test]
    fn preproc_if0_skips_define_in_dead_branch() {
        let src = "#if 0\n#define HIDDEN 42\n#endif\nint x = 1;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("42"));
        assert!(result.output.contains("x = 1") || result.output.contains("int x"));
    }

    /// Regression: `#pragma pack(push, 4)` immediately followed by a struct
    /// definition (e.g. OpenHarmony pwm_if.h) must not swallow the next line.
    #[test]
    fn pragma_keeps_next_line_and_does_not_warn() {
        let src = "#pragma pack(push, 4)\nstruct PwmConfig {\n    int duty;\n};\n#pragma pack(pop)\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("struct PwmConfig"),
            "line after #pragma was swallowed: {}",
            result.output
        );
        assert!(result.output.contains("after"), "{}", result.output);
        assert!(
            !result.output.contains("pack"),
            "pragma text must not leak into output: {}",
            result.output
        );
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("unknown directive")),
            "#pragma is a standard directive, no warning expected: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn line_directive_keeps_next_line() {
        let src = "#line 100 \"orig.c\"\nint kept;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
    }

    #[test]
    fn unknown_directive_warns_but_keeps_next_line() {
        let src = "#frobnicate all the things\nint kept;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("unknown directive")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn handles_ifdef() {
        let opts = PreprocessOptions::new().with_define("FEATURE", "1");
        let src = "#ifdef FEATURE\nint x;\n#else\nint y;\n#endif\n";
        let result = preprocess_string(src, Path::new("test.c"), &opts);
        assert!(result.output.contains("int x") || result.output.contains("int  x"));
        assert!(!result.output.contains("int y"));
    }

    #[test]
    fn handles_ifdef_file() {
        use std::path::PathBuf;
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/preproc/ifdef.c");
        let opts = PreprocessOptions::new().with_define("FEATURE", "1");
        let result = preprocess_file(&path, &opts).unwrap();
        assert!(
            result.output.contains("enabled") && result.output.contains("1"),
            "output was: {}",
            result.output
        );
    }

    #[test]
    fn if_else_selects_active_branch_only() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/if_else.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(
            result.output.contains("active"),
            "expected #if FEATURE branch, got: {}",
            result.output
        );
        assert!(
            !result.output.contains("dead"),
            "dead branch must not appear: {}",
            result.output
        );
        assert!(
            !result.output.contains("also_dead"),
            "inverse branch must not appear: {}",
            result.output
        );
        assert!(
            result.output.contains("also_active"),
            "expected #else after !FEATURE, got: {}",
            result.output
        );
    }

    #[test]
    fn if_macro_value_expands_in_condition() {
        let src = "#define OUTER 1\n#if OUTER\nint on;\n#else\nint off;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("on"), "{}", result.output);
        assert!(!result.output.contains("off"), "{}", result.output);
    }

    #[test]
    fn nested_if_respects_inner_else() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/nested_if.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(result.output.contains("outer_on"), "{}", result.output);
        assert!(!result.output.contains("inner_on"), "{}", result.output);
        assert!(result.output.contains("inner_off"), "{}", result.output);
        assert!(!result.output.contains("outer_off"), "{}", result.output);
    }

    #[test]
    fn ifndef_and_else_inverse() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/ifndef_else.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(result.output.contains("guarded"), "{}", result.output);
        assert!(!result.output.contains("unguarded"), "{}", result.output);
        assert!(result.output.contains("present"), "{}", result.output);
        assert!(!result.output.contains("missing"), "{}", result.output);
    }

    #[test]
    fn self_referential_object_macro_is_not_reexpanded() {
        // Hiview `PRIVATE_MESSAGE_TYPE` X-macro: the replacement list starts
        // with the macro's own name (an enumerator). C11 6.10.3.4 paints
        // that token so expansion terminates; without a hide set this
        // recurses until the stack overflows.
        let src = "\
#define PRIVATE_MESSAGE_TYPE \\\n\
        PRIVATE_MESSAGE_TYPE, \\\n\
        ENGINE_UPLOAD_READY_MSG\n\
enum { PRIVATE_MESSAGE_TYPE };\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("PRIVATE_MESSAGE_TYPE")
                && result.output.contains("ENGINE_UPLOAD_READY_MSG"),
            "{}",
            result.output
        );
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("expansion depth exceeded")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn mutual_object_macros_terminate() {
        let src = "#define A B+B\n#define B A\nint x = A;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        let compact: String = result
            .output
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        assert!(
            compact.contains("A+A") || compact.contains("x=A+A"),
            "{}",
            result.output
        );
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("expansion depth exceeded")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn nested_same_function_macro_still_expands() {
        let src = "#define MIN(a, b) ((a) < (b) ? (a) : (b))\nint x = MIN(MIN(1, 2), 3);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            !result.output.contains("MIN"),
            "nested MIN must fully expand: {}",
            result.output
        );
        assert!(
            result.output.contains("1") && result.output.contains("3"),
            "{}",
            result.output
        );
    }

    #[test]
    fn cpp_operator_call_keeps_adjacent_parens() {
        let src = "struct Fn { void operator()() {} };\n";
        let result = preprocess_string(src, Path::new("t.cpp"), &PreprocessOptions::new());
        assert!(
            result.output.contains("operator()"),
            "operator() must not become operator( ): {}",
            result.output
        );
        assert!(!result.output.contains("operator( )"), "{}", result.output);
        let src = "void f(const std::shared_ptr<Plugin> &p);\n";
        let result = preprocess_string(src, Path::new("t.cpp"), &PreprocessOptions::new());
        assert!(
            result.output.contains("> &") || result.output.contains("> &p"),
            "template-id and reference must not glue: {}",
            result.output
        );
    }

    #[test]
    fn unused_macro_is_predefined_empty() {
        let src = "void f(int &x __UNUSED) { (void)x; }\n";
        let result = preprocess_string(src, Path::new("t.cpp"), &PreprocessOptions::new());
        assert!(
            !result.output.contains("__UNUSED"),
            "__UNUSED must expand away: {}",
            result.output
        );
        assert!(
            result.output.contains("int") && result.output.contains("&"),
            "{}",
            result.output
        );
    }

    #[test]
    fn unused_macro_applies_with_shared_table() {
        let shared = Arc::new(RwLock::new(MacroTable::new()));
        let opts = PreprocessOptions::new().with_shared_macros(Arc::clone(&shared));
        let src = "void f(int &x __UNUSED) {}\n";
        let result = preprocess_string(src, Path::new("t.cpp"), &opts);
        assert!(
            !result.output.contains("__UNUSED"),
            "builtins must apply after cloning the shared table: {}",
            result.output
        );
    }

    #[test]
    fn kernel_annotation_macros_predefined_empty() {
        let src = "static long Read(struct file* f, char __user* buf);\n\
                   static int __init DriverInit(void) { return 0; }\n\
                   static void __exit DriverExit(void) {}\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        for name in ["__user", "__init", "__exit"] {
            assert!(
                !result.output.contains(name),
                "{name} must expand away: {}",
                result.output
            );
        }
        assert!(result.output.contains("DriverInit"), "{}", result.output);
    }

    #[test]
    fn container_of_macro_predefined() {
        let src = "void f(struct Node* p) { struct Dev* d = container_of(p, struct Dev, node); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            !result.output.contains("container_of"),
            "container_of must expand away: {}",
            result.output
        );
        assert!(
            result.output.contains("struct Dev *") || result.output.contains("struct Dev*"),
            "expansion must cast to the requested type: {}",
            result.output
        );
    }

    #[test]
    fn pri_format_macros_predefined() {
        let src = "void f(unsigned long long v) { printf(\"val %\" PRIu64 \"\\n\", v); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            !result.output.contains("PRIu64"),
            "PRIu64 must expand to a string literal: {}",
            result.output
        );
        assert!(result.output.contains("\"llu\""), "{}", result.output);
    }

    #[test]
    fn hwtest_macros_predefined_as_functions() {
        let src = "HWTEST_F(FooTest, Bar, TestSize.Level1)\n{\n    int x = 0;\n    (void)x;\n}\n";
        let result = preprocess_string(src, Path::new("t.cpp"), &PreprocessOptions::new());
        assert!(
            !result.output.contains("HWTEST_F"),
            "HWTEST_F must expand away: {}",
            result.output
        );
        assert!(
            result.output.contains("FooTest_Bar"),
            "expansion must produce a pasted function name: {}",
            result.output
        );
        assert!(
            !result.output.contains("TestSize"),
            "the level argument must be dropped: {}",
            result.output
        );
    }

    #[test]
    fn ifndef_guard_defines_real_macro_over_builtin() {
        let src = "#ifndef container_of\n\
                   #define container_of(p, t, m) CUSTOM_CONTAINER(p)\n\
                   #endif\n\
                   int x = container_of(q, struct D, f);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("CUSTOM_CONTAINER"),
            "an #ifndef-guarded real definition must beat the builtin fallback: {}",
            result.output
        );
    }

    #[test]
    fn builtin_fallback_invisible_to_conditionals() {
        let src = "#ifdef __user\nint user_visible;\n#endif\n\
                   #if defined(__init)\nint init_visible;\n#endif\n\
                   int done;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            !result.output.contains("user_visible") && !result.output.contains("init_visible"),
            "builtin fallbacks must not satisfy #ifdef/defined(): {}",
            result.output
        );
        assert!(result.output.contains("done"), "{}", result.output);
    }

    #[test]
    fn cli_define_overrides_builtin_with_shared_table() {
        let shared = Arc::new(RwLock::new(MacroTable::new()));
        let opts = PreprocessOptions::new()
            .with_shared_macros(shared)
            .with_define("__init", "KEEP_ME");
        let result = preprocess_string("int __init x;\n", Path::new("t.c"), &opts);
        assert!(
            result.output.contains("KEEP_ME"),
            "a -D define must override the builtin even in the shared-table path: {}",
            result.output
        );
    }

    #[test]
    fn cached_include_delta_carries_guarded_redefinition() {
        let dir = unique_tmp_dir("fallback_delta");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("c.h"),
            "#ifndef container_of\n#define container_of(p, t, m) REAL_CONTAINER(p)\n#endif\n",
        )
        .unwrap();
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let opts = PreprocessOptions::new()
            .with_include(dir.clone())
            .with_include_expansion_cache(cache);
        let src = "#include \"c.h\"\nint a = container_of(x, struct D, f);\n";
        let r1 = preprocess_string(src, &dir.join("a.c"), &opts);
        assert!(
            r1.output.contains("REAL_CONTAINER"),
            "first TU must use the header's definition: {}",
            r1.output
        );
        // Second TU replays the cached include; the header's redefinition
        // must survive the delta capture and beat this TU's fallback.
        let r2 = preprocess_string(src, &dir.join("b.c"), &opts);
        assert!(
            r2.output.contains("REAL_CONTAINER"),
            "cache replay must carry the header's redefinition over the fallback: {}",
            r2.output
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn fallback_stays_identifier_in_if_expression() {
        let src = "#if 1 || __init\nint kept;\n#endif\n\
                   #if __init\nint dropped;\n#endif\n\
                   int done;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("kept"),
            "a fallback in a #if expression must evaluate as an undefined \
             identifier (0), not expand to nothing and mangle the expression: {}",
            result.output
        );
        assert!(!result.output.contains("dropped"), "{}", result.output);
        assert!(result.output.contains("done"), "{}", result.output);
    }

    #[test]
    fn cached_header_replays_undef_of_fallback() {
        let dir = unique_tmp_dir("fallback_undef");
        fs::create_dir_all(&dir).unwrap();
        // The declaration makes the header content-bearing so a cache entry
        // is actually stored and the second TU takes the replay path.
        fs::write(dir.join("u.h"), "int u_decl;\n#undef __init\n").unwrap();
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let opts = PreprocessOptions::new()
            .with_include(dir.clone())
            .with_include_expansion_cache(cache);
        let src = "#include \"u.h\"\nint __init marker;\n";
        let r1 = preprocess_string(src, &dir.join("a.c"), &opts);
        assert!(
            r1.output.contains("__init"),
            "after the header's #undef the name must stay an identifier: {}",
            r1.output
        );
        // Cache hit must replay the #undef, not leave the fallback installed.
        let r2 = preprocess_string(src, &dir.join("b.c"), &opts);
        assert!(
            r2.output.contains("__init"),
            "cache replay must apply the header's #undef of the fallback: {}",
            r2.output
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn cached_header_replays_noop_undef() {
        let dir = unique_tmp_dir("noop_undef");
        fs::create_dir_all(&dir).unwrap();
        // X is undefined when the entry is created, so a state diff records
        // nothing — only a log of executed directives catches this #undef.
        fs::write(dir.join("u.h"), "int u_decl;\n#undef X\n").unwrap();
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let opts = PreprocessOptions::new()
            .with_include(dir.clone())
            .with_include_expansion_cache(cache);
        let warm = preprocess_string("#include \"u.h\"\n", &dir.join("a.c"), &opts);
        assert!(warm.output.contains("u_decl"), "{}", warm.output);
        let src = "#define X 7\n#include \"u.h\"\nint arr = X;\n";
        let hit = preprocess_string(src, &dir.join("b.c"), &opts);
        assert!(
            hit.output.contains('X') && !hit.output.contains('7'),
            "cache replay must apply the header's #undef even though X was \
             absent when the entry was created: {}",
            hit.output
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn cached_header_replays_undef_then_redefine_of_existing_macro() {
        let dir = unique_tmp_dir("undef_redef");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("r.h"), "int r_decl;\n#undef X\n#define X 9\n").unwrap();
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let opts = PreprocessOptions::new()
            .with_include(dir.clone())
            .with_include_expansion_cache(cache);
        let src = "#define X 1\n#include \"r.h\"\nint a = X;\n";
        let miss = preprocess_string(src, &dir.join("a.c"), &opts);
        assert!(
            miss.output.contains('9') && !miss.output.contains('1'),
            "{}",
            miss.output
        );
        // X existed at header entry AND exit, so a state diff records
        // neither the undef nor the redefinition.
        let hit = preprocess_string(src, &dir.join("b.c"), &opts);
        assert!(
            hit.output.contains('9') && !hit.output.contains('1'),
            "cache replay must reproduce undef-then-redefine of a macro that \
             existed when the entry was created: {}",
            hit.output
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn cached_header_define_overwrites_like_live_execution() {
        let dir = unique_tmp_dir("replay_overwrite");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("r.h"), "int r_decl;\n#define X 9\n").unwrap();
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let opts = PreprocessOptions::new()
            .with_include(dir.clone())
            .with_include_expansion_cache(cache);
        let src = "#define X 1\n#include \"r.h\"\nint a = X;\n";
        let miss = preprocess_string(src, &dir.join("a.c"), &opts);
        assert!(
            miss.output.contains('9') && !miss.output.contains('1'),
            "{}",
            miss.output
        );
        let hit = preprocess_string(src, &dir.join("b.c"), &opts);
        assert!(
            hit.output.contains('9') && !hit.output.contains('1'),
            "a replayed #define must overwrite like live execution: {}",
            hit.output
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn cached_replay_accumulates_to_shared_table() {
        let dir = unique_tmp_dir("replay_accum");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("m.h"), "int m_decl;\n#define FROM_HDR 5\n").unwrap();
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let src = "#include \"m.h\"\n";
        let shared1 = Arc::new(RwLock::new(MacroTable::new()));
        let opts1 = PreprocessOptions::new()
            .with_include(dir.clone())
            .with_include_expansion_cache(Arc::clone(&cache))
            .with_shared_macros(shared1)
            .with_accumulate_macros(true);
        preprocess_string(src, &dir.join("a.c"), &opts1);
        // The second run hits the cache; the replayed #define must reach the
        // shared table exactly as a live #define would.
        let shared2 = Arc::new(RwLock::new(MacroTable::new()));
        let opts2 = PreprocessOptions::new()
            .with_include(dir.clone())
            .with_include_expansion_cache(cache)
            .with_shared_macros(Arc::clone(&shared2))
            .with_accumulate_macros(true);
        preprocess_string(src, &dir.join("b.c"), &opts2);
        assert!(
            shared2.read().unwrap().contains_key("FROM_HDR"),
            "cache replay must accumulate macros into the shared table"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    /// Spacing-insensitive view of preprocessed output, so assertions
    /// don't depend on the emitter's whitespace choices.
    fn flat(output: &str) -> String {
        output.replace(['\n', ' '], "")
    }

    #[test]
    fn unnamed_variadic_empty_args_elide_comma() {
        let src = "#define LOG(fmt, ...) printf(fmt, ##__VA_ARGS__)\n\
                   void f(void) { LOG(\"plain\"); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            !flat(&result.output).contains(",)"),
            "GNU `, ##__VA_ARGS__` with no varargs must elide the comma: {}",
            result.output
        );
        assert!(result.output.contains("printf"), "{}", result.output);
    }

    #[test]
    fn unnamed_variadic_forwards_args_once() {
        let src = "#define LOG(fmt, ...) printf(fmt, ##__VA_ARGS__)\n\
                   void f(void) { LOG(\"num %d\", 1); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert_eq!(
            result.output.matches("num %d").count(),
            1,
            "__VA_ARGS__ must not re-substitute the named parameters: {}",
            result.output
        );
        assert_eq!(
            result.output.matches('1').count(),
            1,
            "varargs must be substituted exactly once: {}",
            result.output
        );
    }

    #[test]
    fn zero_named_param_variadic_expands() {
        let src = "#define TRACE(...) log_event(__VA_ARGS__)\n\
                   void f(void) { TRACE(); TRACE(1, 2); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            flat(&result.output).contains("log_event()"),
            "empty __VA_ARGS__ must expand to nothing: {}",
            result.output
        );
        assert!(
            flat(&result.output).contains("log_event(1,2)"),
            "{}",
            result.output
        );
    }

    #[test]
    fn variadic_string_vararg_survives() {
        let src = "#define LOG(fmt, ...) printf(fmt, ##__VA_ARGS__)\n\
                   void f(void) { LOG(\"%s\", \"reason\"); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("\"reason\""),
            "a string-literal vararg must not be destroyed by comma pasting: {}",
            result.output
        );
        assert!(!flat(&result.output).contains(",)"), "{}", result.output);
    }

    #[test]
    fn variadic_comma_stays_punct_for_nested_split() {
        let src = "#define INNER(a, b) use(a); use(b);\n\
                   #define WRAP(fmt, ...) INNER(fmt, ##__VA_ARGS__)\n\
                   void f(void) { WRAP(\"f\", x); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            flat(&result.output).contains("use(x)"),
            "the separator comma must stay a real token so nested macros split \
             their arguments: {}",
            result.output
        );
        assert!(!flat(&result.output).contains("use()"), "{}", result.output);
    }

    #[test]
    fn variadic_first_vararg_still_macro_expands() {
        let src = "#define COUNT 42\n\
                   #define LOG(fmt, ...) printf(fmt, ##__VA_ARGS__)\n\
                   void f(void) { LOG(\"%d\", COUNT); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("42"),
            "the first vararg must stay expandable on rescan: {}",
            result.output
        );
        assert!(!result.output.contains("COUNT"), "{}", result.output);
    }

    #[test]
    fn named_variadic_va_args_spelling_aliases_to_tail() {
        let src = "#define LOGE(fmt, args...) HiLogPrint(fmt, ##__VA_ARGS__)\n\
                   void f(void) { LOGE(\"oom\", n); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            !result.output.contains("__VA_ARGS__"),
            "__VA_ARGS__ in a named-variadic body must alias the tail param: {}",
            result.output
        );
        assert!(result.output.contains('n'), "{}", result.output);
    }

    #[test]
    fn param_list_line_continuation() {
        let src = "#define LOG(fmt, \\\n    ...) printf(fmt, ##__VA_ARGS__)\n\
                   void f(void) { LOG(\"x\"); }\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("printf") && result.output.contains("after"),
            "a continued parameter list must not abort the file: {}",
            result.output
        );
    }

    #[test]
    fn named_variadic_continuation_before_ellipsis() {
        let src = "#define F(x, args \\\n...) g(x, ##args)\n\
                   void h(void) { F(1); F(2, 3); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            flat(&result.output).contains("g(1)"),
            "a continuation between a named variadic and `...` must parse: {}",
            result.output
        );
        assert!(flat(&result.output).contains("g(2,3)"), "{}", result.output);
    }

    #[test]
    fn continuation_before_close_does_not_leak_paren() {
        let src = "#define VLOG(fmt, ... \\\n) printf(fmt)\n\
                   void f(void) { VLOG(\"x\"); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("printf(\"x\")") && !result.output.contains(") printf"),
            "tokens after `...` must not leak into the replacement list: {}",
            result.output
        );
    }

    #[test]
    fn explicitly_empty_vararg_keeps_comma() {
        // gcc/clang: `, ##__VA_ARGS__` deletes the comma only when the
        // varargs are OMITTED; an explicitly supplied empty argument keeps
        // it (F(1) -> g(1) but F(1,) -> g(1,); verified against both).
        let src = "#define F(x, ...) g(x, ##__VA_ARGS__)\n\
                   void h(void) { F(1); }\nvoid k(void) { F(2,); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            flat(&result.output).contains("g(1)"),
            "omitted varargs must elide the comma: {}",
            result.output
        );
        assert!(
            flat(&result.output).contains("2,)"),
            "an explicitly empty vararg must keep the comma like gcc: {}",
            result.output
        );
    }

    #[test]
    fn whitespace_only_explicit_vararg_keeps_comma() {
        let src = "#define LOG(fmt, ...) printf(fmt, ##__VA_ARGS__)\n\
                   void f(void) { LOG(\"x\",\n); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            flat(&result.output).contains(",)"),
            "a whitespace-only explicit vararg keeps the comma like gcc: {}",
            result.output
        );
    }

    #[test]
    fn lone_blank_argument_counts_as_omitted() {
        let src = "#define G(...) f(0, ##__VA_ARGS__)\n\
                   void h(void) { G(); G( ); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            !flat(&result.output).contains(",)"),
            "G() supplies zero arguments, so the comma is elided: {}",
            result.output
        );
    }

    #[test]
    fn non_variadic_hash_hash_empty_arg_keeps_comma() {
        let src = "#define M(a, b) f(a, ## b)\nvoid g(void) { M(x, ); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            flat(&result.output).contains("f(x,)"),
            "GNU comma deletion applies only to variadic tails; a non-variadic \
             empty ##-argument keeps the comma like gcc: {}",
            result.output
        );
    }

    #[test]
    fn truncated_param_list_in_expansion_does_not_panic() {
        // The expansion of DECL re-scans a `#define` whose parameter list is
        // truncated mid-`...`; this must degrade to a diagnostic, not panic.
        let src = "#define DECL(x) #define x(...\nDECL(FOO)\nint after(void);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            flat(&result.output).contains("intafter(void);"),
            "{}",
            result.output
        );
    }

    /// An unterminated parameter list ends at the newline like any other
    /// directive: warn, drop the definition, keep the following code (gcc
    /// errors and continues; it never lets the list run on to a `)` on a
    /// later line).
    #[test]
    fn unterminated_param_list_keeps_following_code() {
        let src =
            "#define PARTIAL(x, ...\nint before(void);\nint f(int a) { return (a); }\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        let out = flat(&result.output);
        assert!(out.contains("intbefore(void);"), "{}", result.output);
        assert!(out.contains("intf(inta){return(a);}"), "{}", result.output);
        assert!(out.contains("intafter;"), "{}", result.output);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("unterminated macro parameter list")),
            "{:?}",
            result.diagnostics
        );
        assert!(!result.output.contains("PARTIAL"), "{}", result.output);
    }

    #[test]
    fn unterminated_param_list_without_ellipsis_keeps_following_code() {
        let src = "#define PARTIAL(x,\nint f(int a) { return (a); }\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            flat(&result.output).contains("intf(inta){return(a);}"),
            "{}",
            result.output
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("unterminated macro parameter list")),
            "{:?}",
            result.diagnostics
        );
    }

    /// A malformed (but terminated) list is dropped the same way instead of
    /// stopping preprocessing of the whole file.
    #[test]
    fn malformed_param_list_keeps_following_code() {
        let src = "#define BAD(x y) x\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("after"), "{}", result.output);
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("preprocess stopped")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn variadic_logging_macro_chain_expands_cleanly() {
        let src = "#define HILOG_DEBUG(label, fmt, args...) printf(fmt, ##args)\n\
                   #define DECORATOR_HILOG(op, fmt, args...) op(\"L\", fmt, ##args)\n\
                   #define MEDIA_DEBUG_LOG(fmt, ...) DECORATOR_HILOG(HILOG_DEBUG, fmt, ##__VA_ARGS__)\n\
                   void f(void)\n{\n\
                   MEDIA_DEBUG_LOG(\"plain\");\n\
                   MEDIA_DEBUG_LOG(\"num %d\", 1);\n}\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            !flat(&result.output).contains(",)"),
            "empty varargs must elide the comma through the nested chain: {}",
            result.output
        );
        assert_eq!(
            result.output.matches("num %d").count(),
            1,
            "arguments must be forwarded exactly once through the chain: {}",
            result.output
        );
    }

    #[test]
    fn builtin_fallback_yields_to_source_definition() {
        let src = "#define __init KEEP_ME\nint __init x;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("KEEP_ME"),
            "a source #define must override the builtin fallback: {}",
            result.output
        );
    }

    #[test]
    fn self_referential_function_macro_terminates() {
        let src = "#define F(x) F(x)\nint y = F(1);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("F") && result.output.contains("1"),
            "{}",
            result.output
        );
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("expansion depth exceeded")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn self_ref_macro_fixture() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/self_ref_macro.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(
            result.output.contains("PRIVATE_MESSAGE_TYPE")
                && result.output.contains("ENGINE_UPLOAD_READY_MSG"),
            "{}",
            result.output
        );
        assert!(
            !result.output.contains("MIN"),
            "nested MIN leaked: {}",
            result.output
        );
    }

    #[test]
    fn include_macro_operand_expands() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/include_macro.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(
            result.output.contains("NESTED_VAL") || result.output.contains("42"),
            "macro #include must splice nested.h: {}",
            result.output
        );
        assert!(
            result
                .included_headers
                .iter()
                .any(|p| p.ends_with("include_macro_nested.h")),
            "included_headers must record the expanded include: {:?}",
            result.included_headers
        );
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("expected string or <...>")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn function_macro_inside_object_expansion_expands() {
        // C11 6.10.3.4: after an object-like macro is replaced, the result is
        // rescanned; a function-like macro invoked there must be expanded
        // too, not emitted verbatim.
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/nested_fn_macro.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(
            !result.output.contains("WRAP") && !result.output.contains("SHARED"),
            "macro invocations leaked verbatim: {}",
            result.output
        );
        assert!(result.output.contains("status_Node"), "{}", result.output);
        assert!(result.output.contains("done"), "{}", result.output);
    }

    #[test]
    fn object_macro_with_parenthesized_value() {
        let src = "#define START (-100)\nint x = START;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("(-100)")
                || result.output.contains("-100")
                || result.output.contains("- 100"),
            "{}",
            result.output
        );
        assert!(!result.output.contains("START"));
    }

    #[test]
    fn variadic_macro_empty_args_strips_hash_hash() {
        let src = "#define WRAP(fmt, arg...) BASE(fmt, ##arg)\nWRAP(\"x\");\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("BASE") && result.output.contains("\"x\""),
            "{}",
            result.output
        );
        assert!(
            !result.output.contains(", ,"),
            "should not leave dangling comma: {}",
            result.output
        );
    }

    #[test]
    fn enum_body_define_does_not_break_preproc() {
        let src = "typedef enum {\n    A = 1,\n#define OFF (-100)\n    B = OFF,\n} E;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result
            .diagnostics
            .iter()
            .any(|d| { d.message.contains("expected identifier in directive") }));
    }

    fn unique_tmp_dir(tag: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("trace_preproc_{}_{}", tag, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        // Canonicalize so fixture paths match cache keys, which are stored
        // canonicalized; on macOS temp_dir() is behind the /var symlink.
        dir.canonicalize().unwrap()
    }

    /// Regression: a nested include whose expansion is fully skipped by an
    /// already-defined guard must (a) warn and (b) NOT be claimed as
    /// content-bearing in the parent's cached `IncludeExpansion::files`.
    /// Claiming it made replaying translation units treat the header as
    /// already included while its content was silently absent.
    #[test]
    fn guard_skipped_include_not_claimed_and_warned() {
        let dir = unique_tmp_dir("guard_starve");
        let a = dir.join("a");
        let b = dir.join("b");
        fs::create_dir_all(&a).unwrap();
        fs::create_dir_all(&b).unwrap();

        // Same content, same guard, two paths: only one can be cached.
        let list_src = "#ifndef LIST_H\n#define LIST_H\nstruct Node { int v; };\n#endif\n";
        fs::write(a.join("list.h"), list_src).unwrap();
        fs::write(b.join("list.h"), list_src).unwrap();
        fs::write(
            dir.join("outer.h"),
            "#include \"list.h\"\nint outer_use(void);\n",
        )
        .unwrap();

        let shared = Arc::new(RwLock::new(MacroTable::new()));
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));

        // Warm-style pass over the first twin: defines LIST_H, caches text.
        let warm_opts = PreprocessOptions::new()
            .with_shared_macros(Arc::clone(&shared))
            .with_accumulate_macros(true)
            .with_include_expansion_cache(Arc::clone(&cache))
            .with_include(a.clone());
        let r1 = preprocess_file(&a.join("list.h"), &warm_opts).unwrap();
        assert!(r1.output.contains("Node"), "{}", r1.output);

        // Second pass reaching the OTHER twin through outer.h: the guard is
        // already defined in the shared table, so b/list.h expands to
        // nothing inline.
        let index_opts = PreprocessOptions::new()
            .with_shared_macros(Arc::clone(&shared))
            .with_accumulate_macros(true)
            .with_include_expansion_cache(Arc::clone(&cache))
            .with_include(b.clone());
        let r2 = preprocess_file(&dir.join("outer.h"), &index_opts).unwrap();
        // Documented consequence: content behind the leaked guard is absent.
        assert!(
            !r2.output.contains("Node"),
            "starved expansion expected here"
        );
        // (a) the starvation is visible as a diagnostic
        assert!(
            r2.diagnostics
                .iter()
                .any(|d| d.message.contains("expanded to nothing")),
            "{:?}",
            r2.diagnostics
        );
        // (b) outer.h's cached entry must not claim b/list.h as content-bearing
        let outer_entry = cache.read().unwrap().get(&dir.join("outer.h")).cloned();
        let claimed_b = outer_entry
            .as_ref()
            .map(|e| e.files.iter().any(|f| *f == b.join("list.h")))
            .unwrap_or(false);
        assert!(
            !claimed_b,
            "claimed starved file: {:?}",
            outer_entry.map(|e| e.files.clone())
        );

        let _ = fs::remove_dir_all(&dir);
    }

    /// Frozen (TU-phase) preprocessing must stay silent about guard-skipped
    /// includes: workers expand misses inline per TU and warnings there would
    /// repeat per translation unit.
    #[test]
    fn frozen_cache_does_not_warn_on_guard_skip() {
        let dir = unique_tmp_dir("frozen_quiet");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("g.h"),
            "#ifndef G_H\n#define G_H\nint g(void);\n#endif\n",
        )
        .unwrap();

        let shared = Arc::new(RwLock::new(MacroTable::new()));
        {
            let mut t = shared.write().unwrap();
            use crate::macros::MacroDef;
            use crate::{Lexer, TokenKind};
            let toks: Vec<_> = Lexer::new("1")
                .tokenize()
                .into_iter()
                .filter(|t| !matches!(t.kind, TokenKind::Eof))
                .collect();
            t.insert("G_H".to_string(), MacroDef::Object { replacement: toks });
        }
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let opts = PreprocessOptions::new()
            .with_shared_macros(Arc::clone(&shared))
            .with_include_expansion_cache(cache)
            .with_frozen_expansion_cache(true)
            .with_include(dir.clone());
        let src = "#include \"g.h\"\nint main(void){return 0;}\n";
        let r = preprocess_string(src, &dir.join("m.c"), &opts);
        assert!(
            !r.diagnostics
                .iter()
                .any(|d| d.message.contains("expanded to nothing")),
            "{:?}",
            r.diagnostics
        );
        let _ = fs::remove_dir_all(&dir);
    }

    /// Diamond includes must not copy a header's cached body into live
    /// output on every skip — that exponentiates. Live text stays unique;
    /// the second parent's *cache entry* still embeds the nested header
    /// so a later replay of only that parent keeps the nested declaration.
    #[allow(clippy::manual_range_contains)]
    #[test]
    fn diamond_include_does_not_blow_up_and_cache_stays_self_contained() {
        let dir = unique_tmp_dir("diamond_inc");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("common.h"),
            "#ifndef COMMON_H\n#define COMMON_H\nstruct NeedThis { int x; };\n#endif\n",
        )
        .unwrap();
        fs::write(
            dir.join("left.h"),
            "#ifndef LEFT_H\n#define LEFT_H\n#include \"common.h\"\nvoid left(void);\n#endif\n",
        )
        .unwrap();
        fs::write(
            dir.join("right.h"),
            "#ifndef RIGHT_H\n#define RIGHT_H\n#include \"common.h\"\nvoid right(struct NeedThis *p);\n#endif\n",
        )
        .unwrap();
        fs::write(
            dir.join("top.h"),
            "#ifndef TOP_H\n#define TOP_H\n#include \"left.h\"\n#include \"right.h\"\n#endif\n",
        )
        .unwrap();

        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let warm = PreprocessOptions::new()
            .with_include_expansion_cache(Arc::clone(&cache))
            .with_include(dir.clone());
        let top = preprocess_file(&dir.join("top.h"), &warm).unwrap();
        let need_count = top.output.matches("NeedThis").count();
        assert!(
            need_count >= 1 && need_count <= 2,
            "live output should mention NeedThis once (maybe twice), not explode: {need_count}\n{}",
            top.output
        );
        assert!(
            top.output.len() < 1024,
            "diamond live output too large: {}",
            top.output.len()
        );

        let right = cache
            .read()
            .unwrap()
            .get(&dir.join("right.h"))
            .cloned()
            .expect("right.h cached");
        assert!(
            right.text.contains("NeedThis"),
            "right.h cache must be self-contained, got {}",
            right.text
        );

        // Frozen consumer that only includes right.h still sees NeedThis.
        let frozen = PreprocessOptions::new()
            .with_include_expansion_cache(Arc::clone(&cache))
            .with_frozen_expansion_cache(true)
            .with_include(dir.clone());
        let c = preprocess_file(&dir.join("right.h"), &frozen).unwrap();
        assert!(
            c.output.contains("NeedThis"),
            "frozen replay of right.h lost nested common.h: {}",
            c.output
        );
        let _ = fs::remove_dir_all(&dir);
    }

    /// n headers each including all previous ones: live output is O(n), not 2^n.
    #[test]
    fn chained_includes_live_output_is_linear() {
        let dir = unique_tmp_dir("chain_inc");
        fs::create_dir_all(&dir).unwrap();
        const N: usize = 24;
        for i in 0..N {
            let mut src = format!("#ifndef H{i}\n#define H{i}\n");
            for j in 0..i {
                src.push_str(&format!("#include \"h{j}.h\"\n"));
            }
            src.push_str(&format!("int v{i};\n#endif\n"));
            fs::write(dir.join(format!("h{i}.h")), src).unwrap();
        }
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let opts = PreprocessOptions::new()
            .with_include_expansion_cache(Arc::clone(&cache))
            .with_include(dir.clone());
        let r = preprocess_file(&dir.join(format!("h{}.h", N - 1)), &opts).unwrap();
        for i in 0..N {
            assert!(
                r.output.contains(&format!("v{i}")),
                "missing v{i} in {}",
                r.output
            );
        }
        assert!(
            r.output.len() < 8 * 1024,
            "chained-include live output exploded: {}",
            r.output.len()
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn include_depth_cap_skips_deeper_nests() {
        let dir = unique_tmp_dir("inc_depth");
        fs::create_dir_all(&dir).unwrap();
        const N: usize = 12;
        for i in 0..N {
            let src = if i + 1 < N {
                format!("int v{i};\n#include \"n{}.h\"\n", i + 1)
            } else {
                format!("int v{i};\n")
            };
            fs::write(dir.join(format!("n{i}.h")), src).unwrap();
        }
        let opts = PreprocessOptions::new()
            .with_include(dir.clone())
            .with_max_include_depth(6);
        let r = preprocess_file(&dir.join("n0.h"), &opts).unwrap();
        assert!(
            r.diagnostics
                .iter()
                .any(|d| d.message.contains("include depth exceeded")),
            "expected depth warning: {:?}",
            r.diagnostics
        );
        assert!(r.output.contains("v0"), "{}", r.output);
        assert!(
            !r.output.contains("v11"),
            "depth cap should not expand the whole chain: {}",
            r.output
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn token_budget_stops_explosive_macro_expansion() {
        let src = "\
#define A B B B B B B B B
#define B C C C C C C C C
#define C D D D D D D D D
#define D E E E E E E E E
#define E 1
int x = A;
";
        let opts = PreprocessOptions::new().with_max_expanded_tokens(2_000);
        let result = preprocess_string(src, Path::new("t.c"), &opts);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("token budget exceeded")),
            "expected token-budget diagnostic: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn inline_false_keeps_parent_output_file_local() {
        let dir = unique_tmp_dir("no_inline");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("common.h"),
            "#ifndef COMMON_H\n#define COMMON_H\nstruct NeedThis { int x; };\n#endif\n",
        )
        .unwrap();
        fs::write(
            dir.join("top.h"),
            "#ifndef TOP_H\n#define TOP_H\n#include \"common.h\"\nint from_top;\n#endif\n",
        )
        .unwrap();
        let cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let opts = PreprocessOptions::new()
            .with_include_expansion_cache(Arc::clone(&cache))
            .with_include(dir.clone())
            .with_inline_include_bodies(false);
        let top = preprocess_file(&dir.join("top.h"), &opts).unwrap();
        assert!(
            top.output.contains("from_top"),
            "parent tokens must remain: {}",
            top.output
        );
        assert!(
            !top.output.contains("NeedThis"),
            "nested header body must not be copied into parent live output: {}",
            top.output
        );
        let common = cache
            .read()
            .unwrap()
            .get(&dir.join("common.h"))
            .cloned()
            .expect("common.h cached");
        assert!(
            common.text.contains("NeedThis"),
            "child cache still holds its own text: {}",
            common.text
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn if_defined_true_when_macro_defined() {
        let src = "#define FEATURE 1\n#if defined(FEATURE)\nint kept;\n#endif\n#if defined FEATURE\nint kept_noparen;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
        assert!(result.output.contains("kept_noparen"), "{}", result.output);
    }

    #[test]
    fn if_defined_false_when_macro_undefined() {
        let src = "#if defined(MISSING)\nint dropped;\n#endif\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("dropped"), "{}", result.output);
        assert!(result.output.contains("after"), "{}", result.output);
    }

    #[test]
    fn if_not_defined_false_when_macro_defined() {
        let src = "#define FEATURE 1\n#if !defined(FEATURE)\nint dropped;\n#endif\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("dropped"), "{}", result.output);
        assert!(result.output.contains("after"), "{}", result.output);
    }

    #[test]
    fn if_defined_conjunction_requires_both() {
        let both = "#define A 1\n#define B 1\n#if defined(A) && defined(B)\nint kept;\n#endif\n";
        let result = preprocess_string(both, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
        let one = "#define A 1\n#if defined(A) && defined(B)\nint dropped;\n#endif\n";
        let result = preprocess_string(one, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("dropped"), "{}", result.output);
    }

    #[test]
    fn if_not_defined_binds_tighter_than_and() {
        // (!defined A) && (defined B): A defined, B defined -> false && true = false.
        let src = "#define A 1\n#define B 1\n#if !defined(A) && defined(B)\nint dropped;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("dropped"), "{}", result.output);
        // A undefined, B defined -> true && true = true.
        let src = "#define B 1\n#if !defined(A) && defined(B)\nint kept;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
    }

    #[test]
    fn elif_not_taken_after_taken_if() {
        let src = "#if 1\nint first;\n#elif 1\nint second;\n#else\nint third;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("first"), "{}", result.output);
        assert!(!result.output.contains("second"), "{}", result.output);
        assert!(!result.output.contains("third"), "{}", result.output);
    }

    #[test]
    fn else_not_taken_after_taken_elif() {
        let src = "#if 0\nint first;\n#elif 1\nint second;\n#elif 1\nint third;\n#else\nint fourth;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("first"), "{}", result.output);
        assert!(result.output.contains("second"), "{}", result.output);
        assert!(!result.output.contains("third"), "{}", result.output);
        assert!(!result.output.contains("fourth"), "{}", result.output);
    }

    #[test]
    fn else_taken_when_no_branch_matched() {
        let src = "#if 0\nint first;\n#elif 0\nint second;\n#else\nint third;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("first"), "{}", result.output);
        assert!(!result.output.contains("second"), "{}", result.output);
        assert!(result.output.contains("third"), "{}", result.output);
    }

    #[test]
    fn if_comparisons_and_parens() {
        let src = "#define VERSION 3\n#if (VERSION >= 2) && !defined(MISSING)\nint kept;\n#endif\n#if VERSION == 2\nint dropped;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
        assert!(!result.output.contains("dropped"), "{}", result.output);
    }

    #[test]
    fn if_or_precedence_over_and() {
        // C precedence: 1 || 0 && 0 == 1 || (0 && 0) -> true.
        let src = "#if 1 || 0 && 0\nint kept;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
    }

    #[test]
    fn if_unknown_identifier_evaluates_to_zero() {
        let src = "#if TOTALLY_UNDEFINED_NAME\nint dropped;\n#endif\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("dropped"), "{}", result.output);
        assert!(result.output.contains("after"), "{}", result.output);
    }

    #[test]
    fn if_defined_chained_macro_definition() {
        // Operand of defined() must not be macro-expanded (C11 6.10.1p4).
        let src = "#define ON 1\n#define FEATURE ON\n#if defined(FEATURE)\nint kept;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
    }

    #[test]
    fn if_function_like_macro_expands_in_condition() {
        let src =
            "#define GE(a, b) ((a) >= (b))\n#if GE(3, 2)\nint kept;\n#else\nint dropped;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
        assert!(!result.output.contains("dropped"), "{}", result.output);
    }

    #[test]
    fn if_function_like_macro_uses_object_macro_args() {
        let src = "#define V 3\n#define ATLEAST(x) (V >= (x))\n#if ATLEAST(2)\nint kept;\n#else\nint dropped;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
        assert!(!result.output.contains("dropped"), "{}", result.output);
    }

    #[test]
    fn if_trailing_garbage_is_false() {
        let src = "#if 1 garbage\nint dropped;\n#endif\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("dropped"), "{}", result.output);
        assert!(result.output.contains("after"), "{}", result.output);
    }

    #[test]
    fn if_unbalanced_paren_is_false() {
        let src = "#if (0 || 1\nint dropped;\n#endif\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("dropped"), "{}", result.output);
        assert!(result.output.contains("after"), "{}", result.output);
    }

    #[test]
    fn if_condition_spans_line_continuation() {
        let src = "#if 0 || \\\n    1\nint kept;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
        assert!(
            !result.output.contains("\n1"),
            "continuation line must not leak: {}",
            result.output
        );
    }

    #[test]
    fn if_unsigned_64bit_literal_is_positive() {
        let src = "#if 0xffffffffffffffffULL > 0\nint big_ok;\n#endif\n#if 0xFFFFFFFF & 0x80000000\nint mask_ok;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("big_ok"), "{}", result.output);
        assert!(result.output.contains("mask_ok"), "{}", result.output);
    }

    #[test]
    fn if_defined_elif_fixture() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/if_defined_elif.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(result.output.contains("feature_on"), "{}", result.output);
        assert!(!result.output.contains("feature_off"), "{}", result.output);
        assert!(!result.output.contains("b1"), "{}", result.output);
        assert!(result.output.contains("b2"), "{}", result.output);
        assert!(!result.output.contains("b3"), "{}", result.output);
        assert!(!result.output.contains("b4"), "{}", result.output);
        assert!(result.output.contains("compound_ok"), "{}", result.output);
        assert!(result.output.contains("fnlike_ok"), "{}", result.output);
    }

    #[test]
    fn if_unsigned_conversion_semantics() {
        // Usual arithmetic conversions at uintmax width (C11 6.10.1p4):
        // a signed operand converts to unsigned when the other is unsigned.
        let src = "#if -1 < 1U\nint sc_dropped;\n#endif\n\
#if ~0U > 65535\nint probe_ok;\n#endif\n\
#if -1 > 0U\nint wrap_ok;\n#endif\n\
#if (0x8000000000000000 >> 63) == 1\nint shr_u_ok;\n#endif\n\
#if (-2 >> 1) == -1\nint shr_s_ok;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("sc_dropped"), "{}", result.output);
        assert!(result.output.contains("probe_ok"), "{}", result.output);
        assert!(result.output.contains("wrap_ok"), "{}", result.output);
        assert!(result.output.contains("shr_u_ok"), "{}", result.output);
        assert!(result.output.contains("shr_s_ok"), "{}", result.output);
    }

    #[test]
    fn if_true_false_keywords() {
        let src = "#if true\nint t_kept;\n#endif\n#if false\nint f_dropped;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.cpp"), &PreprocessOptions::new());
        assert!(result.output.contains("t_kept"), "{}", result.output);
        assert!(!result.output.contains("f_dropped"), "{}", result.output);
    }

    #[test]
    fn if_ternary_combines_branch_types() {
        // The ternary result type is the common type of BOTH arms (int +
        // unsigned -> unsigned), even for the untaken arm.
        let src = "#if (1 ? -1 : 1U) < 0\nint uns_dropped;\n#endif\n#if (1 ? -1 : 1) < 0\nint sgn_kept;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("uns_dropped"), "{}", result.output);
        assert!(result.output.contains("sgn_kept"), "{}", result.output);
    }

    #[test]
    fn if_object_macro_aliasing_function_like_rescans() {
        let src = "#define GE(a, b) ((a) >= (b))\n#define CALL GE\n#if CALL(3, 2)\nint kept;\n#else\nint dropped;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
        assert!(!result.output.contains("dropped"), "{}", result.output);
    }

    #[test]
    fn skipped_group_tolerates_malformed_ifdef() {
        let src = "#if 0\n#ifdef 123\nint x;\n#endif\n#endif\nint after;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("after"), "{}", result.output);
        assert!(!result.output.contains("int x"), "{}", result.output);
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("expected identifier")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn skipped_elif_condition_consumes_continuation() {
        let src = "#if 1\nint a;\n#elif 0 && \\\n#endif\nint x;\n#endif\nint tail;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("int a"), "{}", result.output);
        assert!(!result.output.contains("int x"), "{}", result.output);
        assert!(result.output.contains("tail"), "{}", result.output);
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("without #if")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn if_char_escapes() {
        let src = "#if '\\x41' == 65\nint hex_ok;\n#endif\n#if '\\101' == 65\nint oct_ok;\n#endif\n#if '\\012' == 10\nint oct2_ok;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("hex_ok"), "{}", result.output);
        assert!(result.output.contains("oct_ok"), "{}", result.output);
        assert!(result.output.contains("oct2_ok"), "{}", result.output);
    }

    #[test]
    fn if_line_builtin_positive() {
        let src = "#if __LINE__ > 0\nint kept;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
    }

    #[test]
    fn variadic_macro_definition_expands_in_condition() {
        let src = "#define ANY(...) 1\n#if ANY(x)\nint kept;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
    }

    #[test]
    fn cpp_digit_separators_in_condition() {
        let src = "#if 1'000'000 > 999999\nint kept;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.cpp"), &PreprocessOptions::new());
        assert!(result.output.contains("kept"), "{}", result.output);
    }

    #[test]
    fn elif_after_else_is_diagnosed() {
        let src = "#if 0\n#elif 0\n#else\nint a;\n#elif 1\nint b;\n#endif\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("int a"), "{}", result.output);
        assert!(!result.output.contains("int b"), "{}", result.output);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("#elif after #else")),
            "{:?}",
            result.diagnostics
        );
    }

    #[test]
    fn condition_expansion_bomb_hits_budget() {
        // 2^27 tokens if fully expanded; the budget must stop it quickly,
        // warn, and conservatively skip the branch. (Regression guard for
        // the budget itself: the unguarded expander OOMed on this input.)
        let mut src = String::from("#define Z0 z z\n");
        for n in 1..=27 {
            src.push_str(&format!("#define Z{n} Z{} Z{}\n", n - 1, n - 1));
        }
        src.push_str("#if Z27\nint dropped;\n#endif\nint after;\n");
        let result = preprocess_string(&src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(!result.output.contains("dropped"), "{}", result.output);
        assert!(result.output.contains("after"), "{}", result.output);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("expansion budget exceeded in #if")),
            "{:?}",
            result.diagnostics
        );
    }

    /// Issue #8: a header ending inside an open `#if` must not leave its
    /// frame on the stack. The includer continues in its own (active)
    /// state, its own `#endif` still matches its own `#if`, and the
    /// truncated header is diagnosed at the offending directive.
    #[test]
    fn unterminated_if_in_header_keeps_includer_active() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/unterminated_if_include.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(
            result.output.contains("int main"),
            "includer content after the #include vanished: {}",
            result.output
        );
        assert!(
            result.output.contains("int after"),
            "includer content after its own #endif vanished: {}",
            result.output
        );
        assert!(
            !result.output.contains("int dead"),
            "skipped group leaked: {}",
            result.output
        );
        let unterminated: Vec<_> = result
            .diagnostics
            .iter()
            .filter(|d| d.message.contains("unterminated #if"))
            .collect();
        assert_eq!(unterminated.len(), 1, "{:?}", result.diagnostics);
        let d = unterminated[0];
        assert_eq!(d.severity, DiagnosticSeverity::Error);
        assert_eq!(d.line, 1, "diagnostic must point at the open #if");
        assert!(
            d.file
                .as_deref()
                .is_some_and(|f| f.ends_with("unterminated_if_header.h")),
            "diagnostic must name the truncated header: {d:?}"
        );
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("#endif without #if")),
            "includer's #endif must match its own #if: {:?}",
            result.diagnostics
        );
    }

    /// The root file gets the same EOF check, and every open frame is
    /// reported exactly once at the line of its own directive.
    #[test]
    fn unterminated_if_at_root_eof_reports_each_open_frame() {
        let src = "int before;\n#if 1\n#ifdef NOT_SET\nint hidden;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("int before"), "{}", result.output);
        assert!(!result.output.contains("int hidden"), "{}", result.output);
        let mut lines: Vec<u32> = result
            .diagnostics
            .iter()
            .filter(|d| d.message.contains("unterminated #if"))
            .map(|d| d.line)
            .collect();
        lines.sort_unstable();
        assert_eq!(lines, vec![2, 3], "{:?}", result.diagnostics);
    }

    /// Balanced (nested) conditionals must not trip the EOF check.
    #[test]
    fn balanced_nested_conditionals_emit_no_eof_diagnostic() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/nested_if.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("unterminated #if")),
            "{:?}",
            result.diagnostics
        );
    }

    /// A closing or branch directive can only act on a frame opened in the
    /// same file. A header that starts with a stray `#endif` / `#else` /
    /// `#elif` must be diagnosed and must not pop or mutate the includer's
    /// frame, otherwise the includer's own `#endif` fails to match and the
    /// rest of the translation unit is lost.
    #[test]
    fn stray_closing_directives_in_header_do_not_touch_includer_frame() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/stray_closer_include.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        assert!(result.output.contains("survived"), "{}", result.output);
        assert!(result.output.contains("tail"), "{}", result.output);
        for (header, directive) in [
            ("stray_endif.h", "#endif"),
            ("stray_else.h", "#else"),
            ("stray_elif.h", "#elif"),
        ] {
            let expected = format!("{directive} without #if");
            assert!(
                result.diagnostics.iter().any(|d| {
                    d.message == expected
                        && d.line == 1
                        && d.file.as_deref().is_some_and(|f| f.ends_with(header))
                }),
                "missing `{expected}` attributed to {header}: {:?}",
                result.diagnostics
            );
        }
        assert!(
            !result.diagnostics.iter().any(|d| {
                d.message.contains("without #if")
                    && d.file
                        .as_deref()
                        .is_some_and(|f| f.ends_with("stray_closer_include.c"))
            }),
            "includer's #endif must still match its own #if: {:?}",
            result.diagnostics
        );
    }

    /// C11 6.10.3.2: `#param` becomes a string literal spelling the
    /// argument's tokens, with the original spacing collapsed to single
    /// spaces (#13). Before this the `#` was dropped together with its
    /// argument.
    #[test]
    fn stringize_quotes_argument_spelling() {
        let src = "#define STR(x) #x\n\
                   const char *a = STR(hello);\n\
                   const char *b = STR(a + b);\n\
                   const char *c = STR(a+b);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("\"hello\""), "{}", result.output);
        assert!(result.output.contains("\"a + b\""), "{}", result.output);
        assert!(result.output.contains("\"a+b\""), "{}", result.output);
        assert!(!result.output.contains('#'), "{}", result.output);
    }

    /// The operand of `#` is the argument as written, never expanded.
    /// (The two-level `XSTR(x) STR(x)` idiom that expands first needs C11
    /// argument prescan, which docs/PREPROCESSOR.md lists as unsupported.)
    #[test]
    fn stringize_uses_unexpanded_argument() {
        let src = "#define STR(x) #x\n\
                   #define VALUE 42\n\
                   const char *d = STR(VALUE);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("\"VALUE\""), "{}", result.output);
        assert!(!result.output.contains("42"), "{}", result.output);
    }

    /// `"` and `\` inside string and character literals in the argument are
    /// escaped so the result is one valid string literal.
    #[test]
    fn stringize_escapes_literals_in_argument() {
        let src = "#define STR(x) #x\n\
                   const char *c = STR(\"quoted \\\"inner\\\"\");\n\
                   const char *q = STR('\\'');\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains(r#""\"quoted \\\"inner\\\"\"""#),
            "{}",
            result.output
        );
        assert!(result.output.contains(r#""'\\''""#), "{}", result.output);
    }

    /// Newlines inside an argument count as whitespace: one space.
    #[test]
    fn stringize_collapses_newlines_to_single_space() {
        let src = "#define STR(x) #x\nconst char *m = STR(first\n    second);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("\"first second\""),
            "{}",
            result.output
        );
    }

    /// `#__VA_ARGS__` spells every variadic argument, comma-separated.
    #[test]
    fn stringize_variadic_collector_joins_arguments() {
        let src = "#define ALL(...) #__VA_ARGS__\nconst char *g = ALL(p, q);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("\"p, q\""), "{}", result.output);
    }

    /// The log/assert shapes that dominate docs/PARSE_FAILURES.md: `#cond`
    /// inside a larger body, next to `__VA_ARGS__` and `__LINE__`.
    #[test]
    fn stringize_fixture_log_and_assert_macros() {
        use std::path::PathBuf;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/preproc/stringize.c");
        let result = preprocess_file(&path, &PreprocessOptions::new()).unwrap();
        // `flat` strips every space, string-literal contents included.
        let out = flat(&result.output);
        assert!(
            out.contains("log_write(\"hello%d\",1)"),
            "{}",
            result.output
        );
        assert!(
            out.contains("fail(\"x>0&&ptr!=NULL\","),
            "{}",
            result.output
        );
        assert!(!result.output.contains('#'), "{}", result.output);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    }

    /// The generated string literal is macro-expanded code, so it must map
    /// to the expansion site (the invoking macro name), not to the `#` in
    /// the definition (AGENTS.md LineMap invariant).
    #[test]
    fn stringize_maps_to_expansion_site() {
        let src = "#define STR(x) #x\n\n\nconst char *s = STR(foo);\n";
        let mut opts = PreprocessOptions::new();
        opts.track_line_map = true;
        let result = preprocess_string(src, Path::new("t.c"), &opts);
        let off = result.output.find("\"foo\"").expect("stringized literal");
        let entry = result.line_map.lookup(off).expect("line map entry");
        let invocation_col = src.lines().nth(3).unwrap().find("STR(").unwrap() as u32 + 1;
        assert_eq!(
            (entry.line, entry.col),
            (4, invocation_col),
            "{:?}",
            result.line_map
        );
    }

    /// `#__VA_ARGS__` spells the variadic arguments with the commas as
    /// written: no invented space after `,`, and a space before it only
    /// when the source had one.
    #[test]
    fn stringize_variadic_preserves_comma_spacing() {
        let src = "#define ALL(...) #__VA_ARGS__\n\
                   const char *a = ALL(p,q);\n\
                   const char *b = ALL(p, q);\n\
                   const char *c = ALL(p , q);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("\"p,q\""), "{}", result.output);
        assert!(result.output.contains("\"p, q\""), "{}", result.output);
        assert!(result.output.contains("\"p , q\""), "{}", result.output);
    }

    fn lookup_at<'a>(result: &'a PreprocessResult, needle: &str) -> &'a crate::LineMapEntry {
        let off = result
            .output
            .find(needle)
            .unwrap_or_else(|| panic!("{needle:?} in {}", result.output));
        result.line_map.lookup(off).expect("line map entry")
    }

    fn col_of(src: &str, line: usize, needle: &str) -> u32 {
        src.lines().nth(line - 1).unwrap().find(needle).unwrap() as u32 + 1
    }

    /// A stringized literal produced through a forwarding macro maps to the
    /// outermost invocation, not to the `STR` token inside `WRAP`'s body.
    #[test]
    fn stringize_maps_to_expansion_site_through_wrapper() {
        let src = "#define STR(x) #x\n#define WRAP(x) STR(x)\n\nconst char *s = WRAP(foo);\n";
        let mut opts = PreprocessOptions::new();
        opts.track_line_map = true;
        let result = preprocess_string(src, Path::new("t.c"), &opts);
        let e = lookup_at(&result, "\"foo\"");
        assert_eq!(
            (e.line, e.col),
            (4, col_of(src, 4, "WRAP(")),
            "{:?}",
            result.line_map
        );
    }

    /// Every replacement-list token maps to the invocation (AGENTS.md
    /// LineMap invariant); an argument token keeps its own source position.
    #[test]
    fn replacement_tokens_map_to_expansion_site() {
        let src = "#define ADD(x) x + 1\n#define TWICE(x) ADD(x) * 2\n\nint a = TWICE(v);\n";
        let mut opts = PreprocessOptions::new();
        opts.track_line_map = true;
        let result = preprocess_string(src, Path::new("t.c"), &opts);
        let site = (4, col_of(src, 4, "TWICE("));
        let plus = lookup_at(&result, "+");
        assert_eq!((plus.line, plus.col), site, "{:?}", result.line_map);
        let star = lookup_at(&result, "*");
        assert_eq!((star.line, star.col), site, "{:?}", result.line_map);
        let v = lookup_at(&result, "v");
        assert_eq!(
            (v.line, v.col),
            (4, col_of(src, 4, "v)")),
            "{:?}",
            result.line_map
        );
    }

    /// `__LINE__` in a replacement list is the line of the invocation
    /// (C11 6.10.8.1), not of the `#define`.
    #[test]
    fn line_macro_in_body_expands_to_invocation_line() {
        let src = "#define HERE __LINE__\n#define VIA HERE\n\n\nint a = HERE;\nint b = VIA;\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        let out = flat(&result.output);
        assert!(out.contains("inta=5;"), "{}", result.output);
        assert!(out.contains("intb=6;"), "{}", result.output);
    }

    /// Translation phase 2 (C11 5.1.1.2p1) deletes `\`-newline before
    /// tokenizing, so a tight splice inside a stringized argument is
    /// zero-width and the tokens around it are adjacent: gcc and clang both
    /// spell `STR(a\<newline>b)` as "ab", not "a b".
    #[test]
    fn stringize_tight_line_splice_is_zero_width() {
        let src = "#define STR(x) #x\nconst char *s = STR(a\\\nb);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("\"ab\""),
            "tight splice must not introduce a space: {}",
            result.output
        );
    }

    /// Only the splice itself is zero-width. Whitespace before the `\` or
    /// after the newline is real and still separates the tokens.
    #[test]
    fn stringize_keeps_whitespace_around_a_splice() {
        for src in [
            "#define STR(x) #x\nconst char *s = STR(a \\\nb);\n",
            "#define STR(x) #x\nconst char *s = STR(a\\\n b);\n",
        ] {
            let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
            assert!(
                result.output.contains("\"a b\""),
                "whitespace around a splice must survive: {}",
                result.output
            );
        }
    }

    /// A run of splices is still zero-width as long as every one of them is
    /// tight, and a splice between two punctuators behaves the same way.
    #[test]
    fn stringize_chained_and_punct_splices() {
        let chained = "#define STR(x) #x\nconst char *s = STR(a\\\n\\\nb);\n";
        let result = preprocess_string(chained, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("\"ab\""), "{}", result.output);

        let punct = "#define STR(x) #x\nconst char *s = STR(p->\\\nq);\n";
        let result = preprocess_string(punct, Path::new("t.c"), &PreprocessOptions::new());
        assert!(result.output.contains("\"p->q\""), "{}", result.output);
    }

    /// A tight splice right after a top-level `,` is still zero-width even
    /// though the separator starts a fresh argument: adjacency is measured
    /// from the last token consumed, not the last token of the argument.
    #[test]
    fn stringize_tight_splice_after_argument_separator() {
        let src = "#define ALL(...) #__VA_ARGS__\n\
                   const char *s = ALL(p,\\\nq);\n\
                   const char *t = ALL(p\\\n,q);\n\
                   const char *u = ALL(p, \\\nq);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        let out = &result.output;
        assert!(out.contains("s= \"p,q\" ;"), "{out}");
        assert!(out.contains("t= \"p,q\" ;"), "{out}");
        assert!(out.contains("u= \"p, q\" ;"), "{out}");
    }

    /// The splice flag rides on the token through substitution, so an
    /// argument forwarded to another macro and stringized there still
    /// spells the tight splice as zero-width.
    #[test]
    fn stringize_tight_splice_survives_forwarding() {
        let src = "#define INNER(x) #x\n\
                   #define OUTER(x) INNER(x)\n\
                   const char *s = OUTER(a\\\nb);\n";
        let result = preprocess_string(src, Path::new("t.c"), &PreprocessOptions::new());
        assert!(
            result.output.contains("\"ab\""),
            "splice flag must survive nested argument parsing: {}",
            result.output
        );
    }
}
