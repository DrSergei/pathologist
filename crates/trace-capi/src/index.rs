//! `trace_index`: run the whole analyze pipeline against a project directory
//! and write the result to a SQLite database.

use crate::types::{TraceIndexOptions, TraceIndexResult, TraceStatus};
use crate::util::{guard, set_error};
use std::ffi::{c_char, c_int};
use std::path::PathBuf;
use std::sync::Arc;
use trace_analysis::{analyze_with_options, AnalyzeOptions, FnModelSet};
use trace_db::{export_to_sqlite, ExportOptions};
use trace_parse::build_program_with_jobs;
use trace_preproc::PreprocessOptions;

/// Rust-owned copy of the C options, so no borrowed C pointer outlives the
/// call.
struct IndexConfig {
    root: PathBuf,
    output: PathBuf,
    includes: Vec<PathBuf>,
    defines: Vec<(String, String)>,
    jobs: usize,
    full_export: bool,
    debug_points_to: bool,
    models: Vec<PathBuf>,
}

unsafe fn read_config(opts: &TraceIndexOptions) -> Result<IndexConfig, String> {
    let expected = std::mem::size_of::<TraceIndexOptions>();
    if opts.size != 0 && opts.size < expected {
        return Err(format!(
            "trace_index_options.size too small ({} < {expected}); \
             consumer and library were built against different ABI versions",
            opts.size
        ));
    }

    let root = crate::util::cstr(opts.root)?.to_owned();
    let output = crate::util::cstr(opts.output_db)?.to_owned();
    let includes = crate::util::str_array(opts.includes, opts.n_includes)?
        .into_iter()
        .map(PathBuf::from)
        .collect();
    let defines_raw = crate::util::str_array(opts.defines, opts.n_defines)?;
    let mut defines = Vec::with_capacity(defines_raw.len());
    for def in defines_raw {
        if let Some((name, value)) = def.split_once('=') {
            defines.push((name.to_owned(), value.to_owned()));
        } else {
            defines.push((def.clone(), "1".to_owned()));
        }
    }
    let models = crate::util::str_array(opts.models, opts.n_models)?
        .into_iter()
        .map(PathBuf::from)
        .collect();
    let jobs = if opts.jobs <= 0 {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .max(1)
    } else {
        opts.jobs as usize
    };

    Ok(IndexConfig {
        root: PathBuf::from(root),
        output: PathBuf::from(output),
        includes,
        defines,
        jobs,
        full_export: opts.full_export != 0,
        debug_points_to: opts.debug_points_to != 0,
        models,
    })
}

fn too_small(msg: &str, out_err: *mut *mut c_char) -> i32 {
    unsafe { set_error(out_err, msg) };
    TraceStatus::TraceErrInvalidArg as i32
}

/// Fail fast with a clear `TRACE_ERR_IO` (the `i/o error:` prefix is mapped by
/// `status_for`) when the output database path cannot be used, instead of
/// running the whole pipeline first. Detects a missing parent directory,
/// unwritable/read-only destinations and unwritable destinations up front.
///
/// The probe is side-effect-free: it never leaves a partial output file
/// behind (like `export_to_sqlite`, which writes to `<db>.tmp` and renames
/// onto the target only on success). An existing file the user pointed at is
/// opened read-write (not truncated) and left alone; a file that the probe
/// itself had to create is unlinked again so a failed run does not yield a
/// 0-byte file indistinguishable from a real database.
fn preflight_output(path: &std::path::Path) -> Result<(), String> {
    let existed = path.exists();
    let opened = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .map_err(|e| {
            format!(
                "i/o error: cannot open output database {}: {e}",
                path.display()
            )
        });
    match opened {
        Ok(_) => {
            if !existed {
                let _ = std::fs::remove_file(path);
            }
            Ok(())
        }
        Err(msg) => Err(msg),
    }
}

fn run_index(cfg: &IndexConfig) -> Result<TraceIndexResult, String> {
    preflight_output(&cfg.output)?;
    let mut models = FnModelSet::builtin();
    for path in &cfg.models {
        let src = std::fs::read_to_string(path)
            .map_err(|e| format!("failed to read models file {}: {e}", path.display()))?;
        models
            .merge_toml_str(&src)
            .map_err(|e| format!("{}: {e}", path.display()))?;
    }
    let models = Arc::new(models);

    let mut popts = PreprocessOptions::new();
    for inc in &cfg.includes {
        popts.include_paths.push(inc.clone());
    }
    for (name, value) in &cfg.defines {
        popts = popts.with_define(name, value);
    }

    let program =
        build_program_with_jobs(&cfg.root, &popts, cfg.jobs).map_err(|e| e.to_string())?;
    let (pag, analysis) = analyze_with_options(
        &program,
        AnalyzeOptions {
            retain_points_to: cfg.debug_points_to,
            models,
            solve_budget: Some(800_000),
        },
    );
    let model_files: Vec<String> = cfg.models.iter().map(|p| p.display().to_string()).collect();
    export_to_sqlite(
        &program,
        &pag,
        &analysis,
        &ExportOptions {
            output: cfg.output.clone(),
            trace_version: env!("CARGO_PKG_VERSION").to_owned(),
            include_points_to: cfg.debug_points_to,
            full_detail: cfg.full_export,
            model_files,
        },
    )
    .map_err(|e| format!("{e:#}"))?;

    Ok(TraceIndexResult {
        files: program.symbols.files.len() as u64,
        functions: program.symbols.functions.len() as u64,
        call_edges: analysis.call_edges.len() as u64,
        arg_flow_edges: analysis.arg_flow_edges.len() as u64,
    })
}

/// Index a project directory (`opts.root`) into a SQLite database
/// (`opts.output_db`) and fill `out` with summary counters.
///
/// Returns `TRACE_OK` (0) on success. On failure returns a non-zero status
/// and, when `out_err` is non-null, sets it to a message the caller frees
/// with `trace_string_free`. `opts` and every string it references are
/// borrowed for the duration of the call only.
///
/// # Safety
///
/// `opts` and `out` must be valid for the duration of the call, and `opts`
/// must point to a `trace_index_options` at least as large as `size`
/// reports.
#[no_mangle]
pub unsafe extern "C" fn trace_index(
    opts: *const TraceIndexOptions,
    out: *mut TraceIndexResult,
    out_err: *mut *mut c_char,
) -> c_int {
    crate::util::reset_err(out_err);
    if opts.is_null() || out.is_null() {
        return too_small("opts and out must not be null", out_err);
    }
    let cfg = match unsafe { read_config(&*opts) } {
        Ok(c) => c,
        Err(msg) => return too_small(&msg, out_err),
    };

    match guard(|| run_index(&cfg)) {
        Ok(result) => {
            unsafe { *out = result };
            TraceStatus::TraceOk as c_int
        }
        Err(msg) => {
            unsafe { set_error(out_err, &msg) };
            crate::util::status_for(&msg)
        }
    }
}
