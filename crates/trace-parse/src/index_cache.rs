use crate::deps::IncludeGraph;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use trace_preproc::{preprocess_file, Diagnostic, LineMap, PreprocessOptions};

/// Preprocessed text plus its origin map for one canonical file path.
#[derive(Debug, Clone)]
pub struct PreprocessedSource {
    pub text: Arc<str>,
    /// Maps preprocessed offsets back to original `(file, line, col)`.
    /// Empty when the file was not preprocessed (raw source: tree-sitter
    /// positions already refer to original locations).
    pub line_map: Arc<LineMap>,
    /// Canonical `#include` closure from this preprocess run.
    pub included_headers: Arc<Vec<PathBuf>>,
    /// Everything the preprocessor reported while producing `text`, in
    /// emission order, attributed to the file it happened in (nested
    /// includes included). Empty for raw sources.
    pub diagnostics: Vec<Diagnostic>,
}

/// Preprocessed source text for indexing (one entry per canonical file path).
#[derive(Debug, Clone, Default)]
pub struct IndexSourceCache {
    inner: Arc<RwLock<HashMap<PathBuf, Arc<PreprocessedSource>>>>,
}

impl IndexSourceCache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_or_preprocess(
        &self,
        path: &Path,
        graph: &IncludeGraph,
        eff_opts: &PreprocessOptions,
    ) -> Result<Arc<PreprocessedSource>, String> {
        let canonical = graph.intern_path(path);
        if let Ok(guard) = self.inner.read() {
            if let Some(src) = guard.get(&canonical) {
                return Ok(Arc::clone(src));
            }
        }

        let src = Arc::new(read_index_source(path, graph, eff_opts)?);
        if let Ok(mut guard) = self.inner.write() {
            guard.entry(canonical).or_insert_with(|| Arc::clone(&src));
        }
        Ok(src)
    }

    /// Preprocess `path` without storing the result here, for the side
    /// effects carried by `eff_opts` (include-expansion cache, shared macro
    /// table) and for what the run reported. The warm pass uses it for the
    /// second language of a header reached from both C and C++ units: this
    /// cache keeps the text in the language the header is parsed as, but
    /// that language's lexer may not see what this one reports (a `#` line
    /// inside a C++ raw string is a directive in C), so the caller forwards
    /// the returned `diagnostics`.
    pub fn preprocess_uncached(
        &self,
        path: &Path,
        graph: &IncludeGraph,
        eff_opts: &PreprocessOptions,
    ) -> Result<PreprocessedSource, String> {
        read_index_source(path, graph, eff_opts)
    }

    /// Drop `path` so the next `get_or_preprocess` runs the preprocessor
    /// again. The warm pass uses it for a header whose language changed
    /// after a macro-spelled include made it reachable from the other
    /// language's units: the text cached so far was lexed the old way.
    pub fn evict(&self, path: &Path, graph: &IncludeGraph) {
        let canonical = graph.intern_path(path);
        if let Ok(mut guard) = self.inner.write() {
            guard.remove(&canonical);
        }
    }

    /// Canonical file → project headers it `#include`d during preprocess.
    pub fn included_by_file(&self) -> Vec<(PathBuf, Vec<PathBuf>)> {
        let Ok(guard) = self.inner.read() else {
            return Vec::new();
        };
        guard
            .iter()
            .map(|(path, src)| (path.clone(), src.included_headers.as_ref().clone()))
            .collect()
    }
}

impl PreprocessedSource {
    /// A source indexed as-is: tree-sitter positions already refer to
    /// original locations, and nothing was preprocessed that could report.
    fn raw(text: Arc<str>) -> Self {
        Self {
            text,
            line_map: Arc::new(LineMap::new()),
            included_headers: Arc::new(Vec::new()),
            diagnostics: Vec::new(),
        }
    }
}

fn read_index_source(
    path: &Path,
    graph: &IncludeGraph,
    eff_opts: &PreprocessOptions,
) -> Result<PreprocessedSource, String> {
    let canonical = graph.intern_path(path);
    if !should_preprocess(path, eff_opts, graph) {
        if let Some(s) = graph.source_cache.get(&canonical) {
            return Ok(PreprocessedSource::raw(Arc::clone(s)));
        }
        return std::fs::read_to_string(path)
            .map(|s| PreprocessedSource::raw(Arc::from(s)))
            .map_err(|e| e.to_string());
    }
    let preproc_result = preprocess_file(&canonical, eff_opts).map_err(|e| e.to_string())?;
    // Keep partial output even when preprocessing stopped mid-file. A stop
    // usually happens inside ONE nested header; discarding everything and
    // parsing raw source instead silently drops every `#include`d declaration
    // from the unit (328/440 TUs on a real HDF tree) and feeds the parser
    // unexpanded function-like macros, which is strictly less sound than a
    // truncated-but-consistent prefix (spans stay LineMap-mappable).
    Ok(PreprocessedSource {
        text: Arc::from(preproc_result.output),
        line_map: Arc::new(preproc_result.line_map),
        included_headers: Arc::new(preproc_result.included_headers),
        diagnostics: preproc_result.diagnostics,
    })
}

fn should_preprocess(path: &Path, opts: &PreprocessOptions, graph: &IncludeGraph) -> bool {
    if !opts.defines.is_empty() || !opts.include_paths.is_empty() {
        return true;
    }
    graph.needs_preprocess.contains(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn should_preprocess_uses_effective_include_paths() {
        let path = PathBuf::from("/proj/main.c");
        let mut graph = IncludeGraph {
            root: PathBuf::from("/proj"),
            ..Default::default()
        };
        graph.needs_preprocess.insert(path.clone());

        let empty = PreprocessOptions::default();
        assert!(should_preprocess(&path, &empty, &graph));

        let with_include =
            PreprocessOptions::default().with_include(PathBuf::from("/proj/include"));
        assert!(should_preprocess(&path, &with_include, &graph));
    }

    #[test]
    fn get_or_preprocess_falls_back_when_file_missing() {
        let cache = IndexSourceCache::new();
        let graph = IncludeGraph {
            root: PathBuf::from("/nonexistent"),
            ..Default::default()
        };
        let opts = PreprocessOptions::default();
        let missing = PathBuf::from("/nonexistent/definitely_missing_trace_file.c");
        assert!(cache.get_or_preprocess(&missing, &graph, &opts).is_err());
    }
}
