use crate::macros::{MacroDef, MacroTable};
use crate::{Diagnostic, DiagnosticSeverity, Lexer, LineMap, PreprocessOptions, Token, TokenKind};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
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
    include_stack: Vec<PathBuf>,
    included_guard: HashSet<PathBuf>,
    conditional_stack: Vec<bool>,
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
            include_stack: vec![file.clone()],
            included_guard: HashSet::new(),
            conditional_stack: vec![true],
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
        };
        if let Some(shared) = &state.opts.shared_macros {
            if let Ok(guard) = shared.read() {
                state.macros = guard.clone();
            }
        } else {
            state.init_cli_defines();
        }
        // Builtins are local to each preprocess so they apply even when
        // the shared warm table is cloned (hiview `__UNUSED` lives in .cpp
        // files, not in the header that `#ifndef`s it).
        state.install_builtin_macros();
        state
    }

    /// GNU/MSVC unused-parameter markers. Without this, an undefined
    /// `__UNUSED` after a reference declarator (`T &event __UNUSED`) is
    /// parsed as a broken `declaration` and the function body is dropped.
    fn install_builtin_macros(&mut self) {
        if !self.macros.contains_key("__UNUSED") {
            self.macros.insert(
                "__UNUSED".to_string(),
                MacroDef::Object {
                    replacement: Vec::new(),
                },
            );
        }
    }

    fn init_cli_defines(&mut self) {
        let defines: Vec<_> = self
            .opts
            .defines
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (name, val) in defines {
            let tokens = Lexer::new(&val).tokenize();
            let filtered: Vec<Token> = tokens
                .into_iter()
                .filter(|t| !matches!(t.kind, TokenKind::Eof))
                .collect();
            self.insert_macro(
                name,
                MacroDef::Object {
                    replacement: filtered,
                },
            );
        }
    }

    fn insert_macro(&mut self, name: String, def: MacroDef) {
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
        self.conditional_stack.iter().all(|&b| b)
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
            for (name, def) in entry.macros.iter() {
                self.macros
                    .entry(name.clone())
                    .or_insert_with(|| def.clone());
            }
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
        // entries. First-wins: a table that already defines the name
        // (e.g. TUs seeded from the union table) keeps its definition.
        for (name, def) in entry.macros.iter() {
            self.macros
                .entry(name.clone())
                .or_insert_with(|| def.clone());
        }
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
        // Snapshot for the entry's macro delta: everything this header's
        // processing adds relative to its starting table is replayed by
        // `splice_cached` (see `IncludeExpansion::macros`).
        let macros_snapshot = if cache_header && !self.opts.frozen_expansion_cache {
            Some(self.macros.clone())
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
                let macro_defs: Arc<Vec<(String, crate::MacroDef)>> = match &macros_snapshot {
                    Some(snap) => {
                        let mut v: Vec<(String, crate::MacroDef)> = self
                            .macros
                            .iter()
                            .filter(|(k, _)| !snap.contains_key(k.as_str()))
                            .map(|(k, val)| (k.clone(), val.clone()))
                            .collect();
                        v.shrink_to_fit();
                        Arc::new(v)
                    }
                    None => Arc::default(),
                };
                if !composed.is_empty() || !macro_defs.is_empty() || !new_files.is_empty() {
                    if let Ok(mut guard) = cache.write() {
                        guard.entry(canonical).or_insert(crate::IncludeExpansion {
                            text: composed.into(),
                            files: Arc::new(new_files),
                            line_map: Arc::new(composed_map),
                            macros: macro_defs,
                        });
                    }
                }
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
            "ifdef" => {
                let name = self.read_directive_ident(tokens, &mut i)?;
                let defined = self.macros.contains_key(&name);
                self.conditional_stack.push(self.is_active() && defined);
            }
            "ifndef" => {
                let name = self.read_directive_ident(tokens, &mut i)?;
                let defined = self.macros.contains_key(&name);
                self.conditional_stack.push(self.is_active() && !defined);
            }
            "if" => {
                let cond = self.expand_and_eval_condition(tokens, &mut i);
                self.conditional_stack.push(self.is_active() && cond);
            }
            "elif" => {
                if self.conditional_stack.len() <= 1 {
                    return Err(self.error(tokens[i.saturating_sub(1)].line, "#elif without #if"));
                }
                let parent_active = self.conditional_stack[..self.conditional_stack.len() - 1]
                    .iter()
                    .all(|&b| b);
                let current = self.conditional_stack.pop().unwrap();
                if !parent_active || current {
                    self.conditional_stack.push(false);
                } else {
                    let cond = self.expand_and_eval_condition(tokens, &mut i);
                    self.conditional_stack.push(parent_active && cond);
                }
            }
            "else" => {
                if self.conditional_stack.len() <= 1 {
                    return Err(self.error(tokens[i.saturating_sub(1)].line, "#else without #if"));
                }
                let parent_active = self.conditional_stack[..self.conditional_stack.len() - 1]
                    .iter()
                    .all(|&b| b);
                let current = self.conditional_stack.pop().unwrap();
                self.conditional_stack.push(parent_active && !current);
            }
            "endif" => {
                if self.conditional_stack.len() <= 1 {
                    return Err(self.error(tokens[i.saturating_sub(1)].line, "#endif without #if"));
                }
                self.conditional_stack.pop();
            }
            "line" => {
                // #line N "file" — update location tracking
                i = self.skip_to_newline(tokens, i);
            }
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
                i = self.skip_to_newline(tokens, i);
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

    fn expand_and_eval_condition(&self, tokens: &[Token], i: &mut usize) -> bool {
        let mut expanded = String::new();
        while *i < tokens.len() && !matches!(tokens[*i].kind, TokenKind::Newline) {
            if !expanded.is_empty() {
                expanded.push(' ');
            }
            match &tokens[*i].kind {
                TokenKind::Identifier(name) => {
                    if let Some(MacroDef::Object { replacement }) = self.macros.get(name) {
                        for rt in replacement {
                            if !expanded.is_empty()
                                && !expanded.ends_with(' ')
                                && !matches!(rt.kind, TokenKind::Newline)
                            {
                                expanded.push(' ');
                            }
                            expanded.push_str(&token_to_string(&rt.kind));
                        }
                    } else {
                        expanded.push_str(name);
                    }
                }
                other => expanded.push_str(&token_to_string(other)),
            }
            *i += 1;
        }
        eval_pp_condition(&expanded, &self.macros)
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
        if i + 1 < body.len() && matches!(&body[i].kind, TokenKind::Punct(s) if s == "##") {
            if let TokenKind::Identifier(name) = &body[i + 1].kind {
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
                        i += 2;
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
        let mut next = Vec::new();
        let mut changed = false;
        let mut i = 0;
        while i < tokens.len() {
            if i + 2 < tokens.len() && concat_width_at(&tokens, i + 1) > 0 {
                next.push(paste_two_tokens(&tokens[i], &tokens[i + 2]));
                i += 3;
                changed = true;
            } else if concat_width_at(&tokens, i) > 0 {
                i += concat_width_at(&tokens, i);
                changed = true;
            } else {
                next.push(tokens[i].clone());
                i += 1;
            }
        }
        if !changed {
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

fn eval_pp_condition(cond: &str, macros: &MacroTable) -> bool {
    let cond = cond.trim();
    if cond.is_empty() {
        return false;
    }
    if let Some(rest) = cond.strip_prefix('!') {
        return !eval_pp_condition(rest.trim(), macros);
    }
    if cond.starts_with("defined(") || cond.starts_with("defined (") {
        let inner = cond
            .trim_start_matches("defined")
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .trim();
        return macros.contains_key(inner);
    }
    if let Some((lhs, rhs)) = cond.split_once("&&") {
        return eval_pp_condition(lhs, macros) && eval_pp_condition(rhs, macros);
    }
    if let Some((lhs, rhs)) = cond.split_once("||") {
        return eval_pp_condition(lhs, macros) || eval_pp_condition(rhs, macros);
    }
    if let Some((lhs, rhs)) = cond.split_once("==") {
        return eval_pp_atom(lhs) == eval_pp_atom(rhs);
    }
    if let Some((lhs, rhs)) = cond.split_once("!=") {
        return eval_pp_atom(lhs) != eval_pp_atom(rhs);
    }
    eval_pp_atom(cond) != 0
}

fn eval_pp_atom(atom: &str) -> i64 {
    let atom = atom.trim();
    if atom == "0" || atom.eq_ignore_ascii_case("false") {
        return 0;
    }
    if atom == "1" || atom.eq_ignore_ascii_case("true") {
        return 1;
    }
    if let Ok(v) = atom.parse::<i64>() {
        return v;
    }
    if atom.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') && !atom.is_empty() {
        return 1;
    }
    0
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
        dir
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
}
