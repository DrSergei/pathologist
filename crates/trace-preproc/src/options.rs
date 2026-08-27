use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Cached preprocessed body for a `#include`d file (shared across translation units).
#[derive(Debug, Clone)]
pub struct IncludeExpansion {
    pub text: Arc<str>,
    pub files: Arc<HashSet<PathBuf>>,
    /// Origin map for `text`: offsets are relative to the start of the
    /// expansion. Empty when line-map tracking is disabled.
    pub line_map: Arc<crate::LineMap>,
    /// Macro definitions this header's processing added relative to its
    /// starting table (new names only). Cached expansions are replayed
    /// WITHOUT executing their `#define` directives, so a consumer that
    /// splices an entry must re-apply these — otherwise a later header
    /// whose body invokes one of those macros starves during its own
    /// warm pass and freezes invocation residue into its cached text.
    /// `#undef` side effects and redefinitions of pre-existing macros
    /// are not captured (rare in practice).
    pub macros: Arc<Vec<(String, crate::MacroDef)>>,
}

#[derive(Debug, Clone)]
pub struct PreprocessOptions {
    pub include_paths: Vec<PathBuf>,
    pub defines: indexmap::IndexMap<String, String>,
    /// Canonical path → raw file contents (skips disk reads during `#include` expansion).
    pub source_cache: Option<std::sync::Arc<HashMap<PathBuf, String>>>,
    /// Shared cache of expanded `#include` bodies keyed by canonical path.
    pub include_expansion_cache: Option<Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>>>,
    /// Basename → project paths for fast include resolution.
    pub basename_index: Option<Arc<HashMap<String, Vec<PathBuf>>>>,
    /// Shared macro table populated during header warm-up; inherited by translation units.
    pub shared_macros: Option<crate::SharedMacroTable>,
    /// When true, `#define` / `#undef` update [`Self::shared_macros`].
    pub accumulate_macros: bool,
    /// When true, `include_expansion_cache` is read-only: hits are replayed,
    /// misses are expanded inline but never inserted. Parallel workers must
    /// set this so first-writer-wins races cannot make output scheduling-
    /// dependent.
    pub frozen_expansion_cache: bool,
    /// When false, skip `LineMap` updates (faster indexing; spans are not remapped yet).
    pub track_line_map: bool,
    /// Stop expanding a file once live output exceeds this many bytes.
    pub max_output_bytes: usize,
    /// Nested `#include` stack cap (`include_stack.len()` at `process_file`).
    pub max_include_depth: usize,
    /// Token-loop iterations (including macro rescan) per preprocess run.
    pub max_expanded_tokens: u64,
    /// When false, `#include` of a cacheable header replays macros/guards
    /// but does not copy the header body into live output. Indexing uses
    /// this so each file's preprocessed text stays file-local (PCH-style
    /// header IR is merged later). Default true keeps standalone
    /// `preprocess_file` self-contained.
    pub inline_include_bodies: bool,
}

impl Default for PreprocessOptions {
    fn default() -> Self {
        Self {
            include_paths: Vec::new(),
            defines: indexmap::IndexMap::new(),
            source_cache: None,
            include_expansion_cache: None,
            basename_index: None,
            shared_macros: None,
            accumulate_macros: false,
            frozen_expansion_cache: false,
            track_line_map: false,
            max_output_bytes: 32 * 1024 * 1024,
            max_include_depth: 64,
            max_expanded_tokens: 8_000_000,
            inline_include_bodies: true,
        }
    }
}

impl PreprocessOptions {
    pub fn new() -> Self {
        Self {
            track_line_map: true,
            ..Self::default()
        }
    }

    /// Options used for indexing: line-map tracking stays on so lowered
    /// entities can be attributed to their original `#include`d file.
    pub fn for_indexing(mut self) -> Self {
        self.track_line_map = true;
        self
    }

    pub fn with_include_expansion_cache(
        mut self,
        cache: Arc<RwLock<HashMap<PathBuf, IncludeExpansion>>>,
    ) -> Self {
        self.include_expansion_cache = Some(cache);
        self
    }

    pub fn with_basename_index(mut self, index: Arc<HashMap<String, Vec<PathBuf>>>) -> Self {
        self.basename_index = Some(index);
        self
    }

    pub fn with_shared_macros(mut self, table: crate::SharedMacroTable) -> Self {
        self.shared_macros = Some(table);
        self
    }

    pub fn with_accumulate_macros(mut self, accumulate: bool) -> Self {
        self.accumulate_macros = accumulate;
        self
    }

    pub fn with_frozen_expansion_cache(mut self, frozen: bool) -> Self {
        self.frozen_expansion_cache = frozen;
        self
    }

    pub fn with_include(mut self, path: PathBuf) -> Self {
        self.include_paths.push(path);
        self
    }

    pub fn with_define(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.defines.insert(name.into(), value.into());
        self
    }

    pub fn with_max_output_bytes(mut self, n: usize) -> Self {
        self.max_output_bytes = n;
        self
    }

    pub fn with_max_include_depth(mut self, n: usize) -> Self {
        self.max_include_depth = n;
        self
    }

    pub fn with_max_expanded_tokens(mut self, n: u64) -> Self {
        self.max_expanded_tokens = n;
        self
    }

    pub fn with_inline_include_bodies(mut self, inline_bodies: bool) -> Self {
        self.inline_include_bodies = inline_bodies;
        self
    }
}
