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

    fn push_cond(&mut self, cond: bool) {
        let parent_active = self.is_active();
        let active = parent_active && cond;
        self.conditional_stack.push(CondFrame {
            parent_active,
            active,
            taken: active,
            else_seen: false,
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
            self.line_map.push(offset, fid, tok.line, tok.col);
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

    fn warn(&mut self, line: u32, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Warning,
            file: Some(self.current_file.clone()),
            line,
            message: message.into(),
        });
    }

    fn error(&mut self, line: u32, message: impl Into<String>) -> PreprocessError {
        let msg = message.into();
        self.diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Error,
            file: Some(self.current_file.clone()),
            line,
            message: msg.clone(),
        });
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
        if let Err(e) = self.process_tokens(&tokens) {
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

    fn process_tokens(&mut self, tokens: &[Token]) -> Result<(), PreprocessError> {
        let mut i = 0;
        while i < tokens.len() {
            self.check_resource_limits(tokens[i].line)?;
            let tok = &tokens[i];
            if matches!(tok.kind, TokenKind::Eof) {
                break;
            }

            if matches!(tok.kind, TokenKind::Hash) {
                if at_beginning_of_line(tokens, i) {
                    i = self.handle_directive(tokens, i)?;
                    continue;
                }
                if let Some(TokenKind::Identifier(name)) = tokens.get(i + 1).map(|t| &t.kind) {
                    self.emit_str(
                        &format!("\"{name}\""),
                        tokens[i + 1].line,
                        tokens[i + 1].col,
                    );
                    i += 2;
                    continue;
                }
            }

            if self.is_active() {
                if let TokenKind::Identifier(name) = &tok.kind {
                    if name == "__FILE__" {
                        self.emit_str(
                            &format!("\"{}\"", self.current_file.display()),
                            tok.line,
                            tok.col,
                        );
                        i += 1;
                        continue;
                    }
                    if name == "__LINE__" {
                        self.emit_str(&tok.line.to_string(), tok.line, tok.col);
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

    /// Expand macro replacement tokens: no `#` directives; `#x` stringizes; recurse into object macros.
    fn expand_tokens_no_directives(&mut self, tokens: &[Token]) -> Result<(), PreprocessError> {
        let mut i = 0;
        while i < tokens.len() {
            self.check_resource_limits(tokens[i].line)?;
            let tok = &tokens[i];
            if matches!(tok.kind, TokenKind::Eof) {
                break;
            }
            if matches!(tok.kind, TokenKind::Hash) {
                if let Some(TokenKind::Identifier(name)) = tokens.get(i + 1).map(|t| &t.kind) {
                    self.emit_str(
                        &format!("\"{name}\""),
                        tokens[i + 1].line,
                        tokens[i + 1].col,
                    );
                    i += 2;
                    continue;
                }
                self.emit_token(tok);
                i += 1;
                continue;
            }
            if self.is_active() {
                if let TokenKind::Identifier(name) = &tok.kind {
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
                    self.push_cond(defined);
                } else {
                    self.push_cond(false);
                }
            }
            "ifndef" => {
                if self.is_active() {
                    let name = self.read_directive_ident(tokens, &mut i)?;
                    let defined = self.is_defined_for_conditionals(&name);
                    self.push_cond(!defined);
                } else {
                    self.push_cond(false);
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
                self.push_cond(cond);
            }
            "elif" => {
                if self.conditional_stack.is_empty() {
                    return Err(self.error(tokens[i.saturating_sub(1)].line, "#elif without #if"));
                }
                let frame = *self.conditional_stack.last().unwrap();
                let cond = if frame.parent_active && !frame.taken && !frame.else_seen {
                    self.expand_and_eval_condition(tokens, &mut i)
                } else {
                    self.skip_condition_tokens(tokens, &mut i);
                    false
                };
                if frame.else_seen {
                    self.warn(
                        tokens[i.saturating_sub(1)].line,
                        "#elif after #else; branch ignored".to_string(),
                    );
                }
                let f = self.conditional_stack.last_mut().unwrap();
                f.active = f.parent_active && !f.taken && cond;
                f.taken |= f.active;
            }
            "else" => {
                if self.conditional_stack.is_empty() {
                    return Err(self.error(tokens[i.saturating_sub(1)].line, "#else without #if"));
                }
                let f = self.conditional_stack.last_mut().unwrap();
                f.active = f.parent_active && !f.taken;
                f.taken = true;
                f.else_seen = true;
            }
            "endif" => {
                if self.conditional_stack.is_empty() {
                    return Err(self.error(tokens[i.saturating_sub(1)].line, "#endif without #if"));
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
        while i < tokens.len() && matches!(tokens[i].kind, TokenKind::Newline) {
            i += 1;
        }
        let name = self.read_directive_ident(tokens, &mut i)?;
        let paren_start = i;
        if matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == "(")
            && self.looks_like_function_macro_params(tokens, i + 1)
        {
            i += 1;
            let (params, variadic) = self.parse_macro_param_list(tokens, &mut i)?;
            let mut replacement = Vec::new();
            while i < tokens.len() && !matches!(tokens[i].kind, TokenKind::Newline) {
                if matches!(&tokens[i].kind, TokenKind::Punct(s) if s == "\\")
                    && i + 1 < tokens.len()
                    && matches!(tokens[i + 1].kind, TokenKind::Newline)
                {
                    i += 2;
                    continue;
                }
                replacement.push(tokens[i].clone());
                i += 1;
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

        if matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == "(") {
            // `#define NAME (...)` — object macro, replacement starts with `(`.
            i = paren_start;
        }

        let mut replacement = Vec::new();
        while i < tokens.len() && !matches!(tokens[i].kind, TokenKind::Newline) {
            if matches!(&tokens[i].kind, TokenKind::Punct(s) if s == "\\")
                && i + 1 < tokens.len()
                && matches!(tokens[i + 1].kind, TokenKind::Newline)
            {
                i += 2;
                continue;
            }
            replacement.push(tokens[i].clone());
            i += 1;
        }
        self.insert_macro(name, MacroDef::Object { replacement });
        Ok(i)
    }

    fn looks_like_function_macro_params(&self, tokens: &[Token], mut i: usize) -> bool {
        while i < tokens.len() && matches!(tokens[i].kind, TokenKind::Newline) {
            i += 1;
        }
        match tokens.get(i).map(|t| &t.kind) {
            Some(TokenKind::Identifier(_)) => true,
            Some(TokenKind::Punct(s)) if s == ")" => true,
            Some(TokenKind::Punct(s)) if s == "..." => true,
            // The lexer emits `...` as three `.` tokens (`#define ANY(...)`).
            Some(TokenKind::Punct(s)) if s == "." => true,
            _ => false,
        }
    }

    fn parse_macro_param_list(
        &mut self,
        tokens: &[Token],
        i: &mut usize,
    ) -> Result<(Vec<String>, bool), PreprocessError> {
        let mut params = Vec::new();
        let mut variadic = false;
        loop {
            while *i < tokens.len() && matches!(tokens[*i].kind, TokenKind::Newline) {
                *i += 1;
            }
            if *i >= tokens.len() {
                return Err(self.error(1, "unterminated macro parameter list"));
            }
            if matches!(&tokens[*i].kind, TokenKind::Punct(s) if s == ")") {
                *i += 1;
                break;
            }
            if self.token_is_ellipsis(tokens, *i) {
                variadic = true;
                *i = self.skip_ellipsis(tokens, *i);
                while *i < tokens.len() && matches!(tokens[*i].kind, TokenKind::Newline) {
                    *i += 1;
                }
                if matches!(&tokens[*i].kind, TokenKind::Punct(s) if s == ")") {
                    *i += 1;
                }
                break;
            }
            let param = self.read_directive_ident(tokens, i)?;
            params.push(param);
            if self.token_is_ellipsis(tokens, *i) {
                variadic = true;
                *i = self.skip_ellipsis(tokens, *i);
                while *i < tokens.len() && matches!(tokens[*i].kind, TokenKind::Newline) {
                    *i += 1;
                }
                if matches!(&tokens[*i].kind, TokenKind::Punct(s) if s == ")") {
                    *i += 1;
                }
                break;
            }
            while *i < tokens.len() && matches!(tokens[*i].kind, TokenKind::Newline) {
                *i += 1;
            }
            if matches!(&tokens[*i].kind, TokenKind::Punct(s) if s == ")") {
                *i += 1;
                break;
            }
            if matches!(&tokens[*i].kind, TokenKind::Punct(s) if s == ",") {
                *i += 1;
                continue;
            }
            return Err(self.error(tokens[*i].line, "expected , or ) in macro parameters"));
        }
        Ok((params, variadic))
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
    ) -> Result<Vec<Vec<Token>>, PreprocessError> {
        while *i < tokens.len() && matches!(tokens[*i].kind, TokenKind::Newline) {
            *i += 1;
        }
        if !matches!(tokens.get(*i).map(|t| &t.kind), Some(TokenKind::Punct(s)) if s == "(") {
            return Ok(Vec::new());
        }
        *i += 1;
        let mut args: Vec<Vec<Token>> = Vec::new();
        let mut current: Vec<Token> = Vec::new();
        let mut depth = 0u32;
        while *i < tokens.len() {
            if matches!(&tokens[*i].kind, TokenKind::Punct(s) if s == "\\")
                && *i + 1 < tokens.len()
                && matches!(tokens[*i + 1].kind, TokenKind::Newline)
            {
                *i += 2;
                continue;
            }
            match &tokens[*i].kind {
                TokenKind::Punct(s) if s == "(" => {
                    depth += 1;
                    current.push(tokens[*i].clone());
                    *i += 1;
                }
                TokenKind::Punct(s) if s == ")" && depth == 0 => {
                    args.push(current);
                    *i += 1;
                    break;
                }
                TokenKind::Punct(s) if s == ")" => {
                    depth -= 1;
                    current.push(tokens[*i].clone());
                    *i += 1;
                }
                TokenKind::Punct(s) if s == "," && depth == 0 => {
                    args.push(current);
                    current = Vec::new();
                    *i += 1;
                }
                TokenKind::Eof => {
                    return Err(self.error(tokens[*i].line, "unterminated macro argument list"));
                }
                _ => {
                    current.push(tokens[*i].clone());
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
                        TokenKind::Number(tok.line.to_string()),
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

fn substitute_macro(
    macro_name: &str,
    origin: &Token,
    body: &[Token],
    params: &[String],
    args: &[Vec<Token>],
    variadic: bool,
) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    let mut i = 0;
    while i < body.len() {
        let concat_width = concat_width_at(body, i);
        if concat_width > 0 && i + concat_width < body.len() {
            if let TokenKind::Identifier(name) = &body[i + concat_width].kind {
                if let Some(idx) = params.iter().position(|p| p == name) {
                    let arg = if variadic && idx + 1 == params.len() && idx < args.len() {
                        args[idx..].concat()
                    } else {
                        args.get(idx).cloned().unwrap_or_default()
                    };
                    if arg.is_empty() {
                        if let Some(last) = out.last() {
                            if matches!(&last.kind, TokenKind::Punct(s) if s == ",") {
                                out.pop();
                            }
                        }
                        i += concat_width + 1;
                        continue;
                    }
                }
            }
        }
        if let TokenKind::Identifier(name) = &body[i].kind {
            if name == "__VA_ARGS__" && variadic {
                let start = params.len().saturating_sub(1);
                for (ai, arg) in args.iter().enumerate().skip(start) {
                    if ai > start {
                        out.push(
                            Token::new(TokenKind::Punct(",".into()), body[i].line, body[i].col)
                                .with_macro_hide(origin, macro_name),
                        );
                    }
                    out.extend(arg.iter().cloned());
                }
                i += 1;
                continue;
            }
            if let Some(idx) = params.iter().position(|p| p == name) {
                if variadic && idx + 1 == params.len() {
                    for (ai, arg) in args.iter().enumerate().skip(idx) {
                        if ai > idx {
                            out.push(
                                Token::new(TokenKind::Punct(",".into()), body[i].line, body[i].col)
                                    .with_macro_hide(origin, macro_name),
                            );
                        }
                        out.extend(arg.iter().cloned());
                    }
                } else if let Some(arg) = args.get(idx) {
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
fn parse_cond_macro_args(toks: &[Token], mut i: usize) -> Option<(Vec<Vec<Token>>, usize)> {
    if !matches!(toks.get(i).map(|t| &t.kind), Some(TokenKind::Punct(p)) if p == "(") {
        return None;
    }
    i += 1;
    let mut args: Vec<Vec<Token>> = Vec::new();
    let mut current: Vec<Token> = Vec::new();
    let mut depth = 0u32;
    while i < toks.len() {
        match &toks[i].kind {
            TokenKind::Punct(p) if p == "(" => {
                depth += 1;
                current.push(toks[i].clone());
            }
            TokenKind::Punct(p) if p == ")" && depth == 0 => {
                args.push(current);
                return Some((args, i + 1));
            }
            TokenKind::Punct(p) if p == ")" => {
                depth -= 1;
                current.push(toks[i].clone());
            }
            TokenKind::Punct(p) if p == "," && depth == 0 => {
                args.push(current);
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
    if let Err(e) = state.process_tokens(&tokens) {
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
}
