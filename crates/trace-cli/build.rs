mod build_support;

use build_support::{civil_from_days, parse_dirty, BuildMetadata};
use std::env;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    for name in [
        "TRACE_BUILD_GIT_SHA",
        "TRACE_BUILD_GIT_DIRTY",
        "TRACE_BUILD_DATE",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }
    println!("cargo:rerun-if-changed=.");

    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    if let Some(git_dir) = build_support::git_dir(&workspace_root) {
        println!("cargo:rerun-if-changed={}", git_dir.join("HEAD").display());
        println!("cargo:rerun-if-changed={}", git_dir.join("index").display());
    }

    let version = build_support::format_version(env!("CARGO_PKG_VERSION"), &collect());
    println!("cargo:rustc-env=TRACE_BUILD_VERSION={version}");
}

/// Collects build metadata, preferring the `TRACE_BUILD_*` overrides and
/// falling back to Git and the current UTC date.
///
/// The `dirty` flag is only as fresh as the last build-script run. Cargo
/// re-runs this script when `.git/HEAD`, `.git/index`, or a file under
/// `crates/trace-cli/` changes, so edits elsewhere in the workspace can leave
/// a previously clean flag in place. Release builds are unaffected: CI sets
/// `TRACE_BUILD_GIT_DIRTY` explicitly, so the flag is authoritative wherever
/// provenance matters and advisory for local development builds.
fn collect() -> BuildMetadata {
    let commit = env::var("TRACE_BUILD_GIT_SHA")
        .ok()
        .and_then(nonempty)
        .or_else(|| git_output(&["rev-parse", "--short=7", "HEAD"]))
        .unwrap_or_else(|| "unknown".to_owned());
    let dirty = env::var("TRACE_BUILD_GIT_DIRTY")
        .ok()
        .map(|value| parse_dirty(&value))
        .unwrap_or_else(|| {
            git_output(&["status", "--porcelain"]).is_some_and(|output| !output.is_empty())
        });
    let date = env::var("TRACE_BUILD_DATE")
        .ok()
        .and_then(nonempty)
        .unwrap_or_else(current_utc_date);

    BuildMetadata {
        commit,
        dirty,
        date,
    }
}

fn nonempty(value: String) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn current_utc_date() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (year, month, day) = civil_from_days((seconds / 86_400) as i64);
    format!("{year:04}-{month:02}-{day:02}")
}
