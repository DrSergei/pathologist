use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// The language a translation unit is lexed as. Lexing is not identical
/// across the two: C++11 raw string literals (`R"(…)"`) and user-defined
/// literal suffixes (`"x"_s`, `'c'_w`) are single tokens in C++ but two
/// tokens in C, where `R` and the suffix are identifiers that may well be
/// macros (`#define R …` / `'a'C`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    C,
    Cpp,
}

impl Language {
    /// Language implied by a file's extension: the C++ TU and header
    /// spellings GCC recognizes (`.cpp`, `.cc`, `.cxx`, `.c++`, `.C`,
    /// `.hpp`, `.hh`, `.hxx`, `.h++`, `.H`, `.inl`, `.ipp`) are C++;
    /// everything else, including the language-ambiguous `.h`, is C. This
    /// is the one place that decides; the indexer's discovery and grammar
    /// choice derive from it.
    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|e| e.to_str()) {
            Some(
                "cpp" | "cc" | "cxx" | "c++" | "C" | "hpp" | "hh" | "hxx" | "h++" | "H" | "inl"
                | "ipp",
            ) => Language::Cpp,
            _ => Language::C,
        }
    }
}

/// Key of the include-expansion cache: a header's canonical path and the
/// [`Language`] it was lexed as. A header has no language of its own — it
/// is lexed as the translation unit including it — and the two lexers
/// disagree on raw strings and ud-suffixes, so a header reached from both
/// C and C++ units gets one entry per language rather than the first
/// unit's tokenization replayed into the other.
pub type ExpansionKey = (PathBuf, Language);

/// Cached preprocessed body for a `#include`d file (shared across translation units).
#[derive(Debug, Clone)]
pub struct IncludeExpansion {
    pub text: Arc<str>,
    pub files: Arc<HashSet<PathBuf>>,
    /// Origin map for `text`: offsets are relative to the start of the
    /// expansion. Empty when line-map tracking is disabled.
    pub line_map: Arc<crate::LineMap>,
    /// The `#define` / `#undef` directives this header's processing executed
    /// (nested replays included), in order. Cached expansions are spliced
    /// WITHOUT executing their directives, so a consumer must re-apply these
    /// — otherwise a later header whose body invokes one of those macros
    /// starves during its own warm pass and freezes invocation residue into
    /// its cached text. An ordered log rather than a table diff: a diff
    /// cannot represent a no-op `#undef` or an undef-then-redefine of a
    /// name that existed at both capture boundaries.
    pub ops: Arc<Vec<crate::MacroOp>>,
}

#[derive(Debug, Clone)]
pub struct PreprocessOptions {
    pub include_paths: Vec<PathBuf>,
    pub defines: indexmap::IndexMap<String, String>,
    /// Canonical path → raw file contents (skips disk reads during `#include` expansion).
    pub source_cache: Option<std::sync::Arc<HashMap<PathBuf, std::sync::Arc<str>>>>,
    /// Shared cache of expanded `#include` bodies keyed by canonical path
    /// and lexing language (see [`ExpansionKey`]).
    pub include_expansion_cache: Option<Arc<RwLock<HashMap<ExpansionKey, IncludeExpansion>>>>,
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
    /// Language the translation unit (and every header it includes) is
    /// lexed as. `None` derives it from the TU path via
    /// [`Language::from_path`].
    pub language: Option<Language>,
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
            language: None,
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
        cache: Arc<RwLock<HashMap<ExpansionKey, IncludeExpansion>>>,
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

    pub fn with_language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
}
