use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn discover_c_files(root: &Path) -> Vec<PathBuf> {
    discover_source_files(root).0
}

/// `.h` files under the analyzed tree (for struct layouts), not external include dirs.
pub fn discover_header_files(root: &Path) -> Vec<PathBuf> {
    discover_source_files(root).1
}

/// Translation-unit source extensions: C plus the common C++ spellings.
pub const TU_EXTENSIONS: &[&str] = &["c", "cpp", "cc", "cxx", "c++"];
/// Header extensions pulled in via `#include` (and macro-warmed).
pub const HEADER_EXTENSIONS: &[&str] = &["h", "hpp", "hh", "hxx", "H", "inl", "ipp"];

/// C++ translation-unit extensions (not headers).
pub fn is_cpp_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("cpp" | "cc" | "cxx" | "c++")
    )
}

/// C++ header extensions. `.h` is ambiguous and is *not* included: a `.h`
/// is parsed as C++ only when the include graph reaches it from a C++ TU.
pub fn is_cpp_header_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("hpp" | "hh" | "hxx" | "inl" | "ipp")
    )
}

/// Single directory walk collecting C/C++ TU paths and header paths.
pub fn discover_source_files(root: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    if root.is_file() {
        let ext = root.extension().and_then(|e| e.to_str());
        return match ext {
            Some(e) if TU_EXTENSIONS.contains(&e) => (vec![root.to_path_buf()], Vec::new()),
            Some(e) if HEADER_EXTENSIONS.contains(&e) => (Vec::new(), vec![root.to_path_buf()]),
            _ => (Vec::new(), Vec::new()),
        };
    }
    let mut c_files = Vec::new();
    let mut h_files = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        match entry.path().extension().and_then(|x| x.to_str()) {
            Some(e) if TU_EXTENSIONS.contains(&e) => c_files.push(entry.path().to_path_buf()),
            Some(e) if HEADER_EXTENSIONS.contains(&e) => h_files.push(entry.path().to_path_buf()),
            _ => {}
        }
    }
    c_files.sort();
    h_files.sort();
    (c_files, h_files)
}

#[allow(dead_code)]
fn discover_by_extension(root: &Path, ext: &str) -> Vec<PathBuf> {
    if root.is_file() {
        return root
            .extension()
            .and_then(|e| e.to_str())
            .filter(|e| *e == ext)
            .map(|_| vec![root.to_path_buf()])
            .unwrap_or_default();
    }
    let mut paths: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .is_some_and(|x| x == ext)
        })
        .map(|e| e.path().to_path_buf())
        .collect();
    paths.sort();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn cpp_header_extensions_are_cpp() {
        assert!(is_cpp_header_path(Path::new("util.hpp")));
        assert!(is_cpp_header_path(Path::new("util.hh")));
        assert!(!is_cpp_header_path(Path::new("plugin.h")));
        assert!(!is_cpp_path(Path::new("plugin.h")));
        assert!(is_cpp_path(Path::new("plugin.cpp")));
    }
}
