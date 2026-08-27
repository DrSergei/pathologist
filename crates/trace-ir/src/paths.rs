//! Filesystem path helpers shared by all pipeline stages.

use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

std::thread_local! {
    static CANON_CACHE: RefCell<FxHashMap<PathBuf, PathBuf>> =
        RefCell::new(FxHashMap::default());
}

/// Canonicalize `path`, falling back to the original path on error, and strip
/// the Windows extended-length prefix (`\\?\C:\...`,
/// `\\?\UNC\server\share\...`) that `std::fs::canonicalize` adds. Without the
/// strip, every interned file name in the IR and the exported database would
/// carry the prefix on Windows, breaking display and substring matching.
///
/// Repeat lookups of the same path skip `std::fs::canonicalize` (indexing
/// recanonicalizes include-graph keys per TU).
pub fn canonicalize(path: &Path) -> PathBuf {
    CANON_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(existing) = cache.get(path) {
            return existing.clone();
        }
        let canonical = canonicalize_uncached(path);
        cache.insert(path.to_path_buf(), canonical.clone());
        if canonical.as_path() != path {
            cache
                .entry(canonical.clone())
                .or_insert_with(|| canonical.clone());
        }
        canonical
    })
}

fn canonicalize_uncached(path: &Path) -> PathBuf {
    let canonical = match std::fs::canonicalize(path) {
        Ok(p) => p,
        Err(_) => return path.to_path_buf(),
    };
    #[cfg(windows)]
    {
        let text = canonical.as_os_str().to_string_lossy();
        if let Some(rest) = text.strip_prefix(r"\\?\UNC\") {
            return PathBuf::from(format!(r"\\{rest}"));
        }
        if let Some(rest) = text.strip_prefix(r"\\?\") {
            return PathBuf::from(rest);
        }
    }
    canonical
}
