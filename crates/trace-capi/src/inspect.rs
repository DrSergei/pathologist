//! Inspect queries over an already-indexed database: function/symbol lookup
//! by source position, call-edge listing, and bounded call/dataflow graphs.
//!
//! Every query returns an arena-backed result whose strings are owned by the
//! result itself; the caller frees the result with the matching `trace_*_free`
//! and all strings die with it.

use crate::types::*;
use crate::util::{cstr, guard, reset_err, set_error, status_for, Arena};
use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;
use trace_db::QueryGraph;

/// Opaque handle to a read-only analysis database.
pub struct TraceDb {
    conn: rusqlite::Connection,
}

/// Raise an argument error and return its status code.
unsafe fn arg_err(out_err: *mut *mut c_char, msg: &str) -> c_int {
    set_error(out_err, msg);
    TraceStatus::TraceErrInvalidArg as c_int
}

/// Open `path` read-only. Returns an owned handle or null plus `*out_err`.
/// The caller owns the handle and must release it with `trace_db_close`.
///
/// The handle is opened with `SQLITE_OPEN_READ_ONLY`, so:
/// - a missing or unreadable path fails here (returns null) instead of
///   silently creating an empty database that the first query errors on, and
/// - a genuinely read-only file works fine since every operation is a query.
///
/// # Safety
///
/// `path` must be a valid NUL-terminated string for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn trace_db_open(
    path: *const c_char,
    out_err: *mut *mut c_char,
) -> *mut TraceDb {
    reset_err(out_err);
    match guard(|| {
        let path = cstr(path)?.to_owned();
        let conn = rusqlite::Connection::open_with_flags(
            std::path::Path::new(&path),
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| format!("{e:#}"))?;
        Ok(Box::into_raw(Box::new(TraceDb { conn })))
    }) {
        Ok(h) => h,
        Err(msg) => {
            set_error(out_err, &msg);
            ptr::null_mut()
        }
    }
}

/// Close a database handle and free everything it owns. Safe on null; the
/// handle must not be used afterwards.
///
/// # Safety
///
/// `db` must be a handle returned by `trace_db_open` (or null).
#[no_mangle]
pub unsafe extern "C" fn trace_db_close(db: *mut TraceDb) {
    if !db.is_null() {
        drop(Box::from_raw(db));
    }
}

// ---------------------------------------------------------------------------
// Result builders
// ---------------------------------------------------------------------------

/// Shared impl owner: the arena plus the backing vectors whose pointers are
/// exposed to C. All free of the outer list returns this.
struct ListImpl<T> {
    _arena: Arena,
    _items: Vec<T>,
}

fn leak_list<T>(items: Vec<T>, arena: Arena) -> (usize, *mut T, *mut c_void) {
    let mut b = Box::new(ListImpl {
        _arena: arena,
        _items: items,
    });
    let count = b._items.len();
    let ptr = b._items.as_mut_ptr();
    (count, ptr, Box::into_raw(b) as *mut c_void)
}

unsafe fn free_list<T>(l: *mut c_void, out: *mut c_void, out_size: usize) {
    if !l.is_null() {
        drop(Box::from_raw(l as *mut ListImpl<T>));
    }
    ptr::write_bytes(out.cast::<u8>(), 0, out_size);
}

// ---------------------------------------------------------------------------
// Function lookup
// ---------------------------------------------------------------------------

/// Find functions whose `[line_start, line_end]` contains `line` in files
/// whose path contains `file`, best match first. Fills `out` with an
/// arena-backed list; free it with `trace_function_list_free`.
///
/// # Safety
///
/// `db` must be a live handle, `out` a valid destination, and the C strings
/// valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn trace_db_find_functions(
    db: *mut TraceDb,
    file: *const c_char,
    line: i64,
    out: *mut TraceFunctionList,
    out_err: *mut *mut c_char,
) -> c_int {
    reset_err(out_err);
    if db.is_null() || out.is_null() {
        return arg_err(out_err, "db and out must not be null");
    }
    let conn = &(*db).conn;
    let res = guard(|| {
        let file = cstr(file)?.to_owned();
        if file.is_empty() {
            return Err("invalid argument: file must not be empty".to_string());
        }
        let fns = trace_db::find_functions_at(conn, &file, line).map_err(|e| format!("{e:#}"))?;
        let mut arena = Arena::new();
        let items = fns
            .into_iter()
            .map(|f| TraceFunction {
                id: f.id,
                name: arena.add(&f.name),
                path: arena.add(&f.path),
                line_start: f.line_start,
                line_end: f.line_end,
                is_defined: f.is_defined as i32,
            })
            .collect::<Vec<_>>();
        let (count, ptr, impl_) = leak_list(items, arena);
        Ok(TraceFunctionList {
            items: ptr,
            count,
            _impl: impl_,
        })
    });
    match res {
        Ok(list) => {
            *out = list;
            TraceStatus::TraceOk as c_int
        }
        Err(msg) => {
            set_error(out_err, &msg);
            status_for(&msg)
        }
    }
}

/// Free a function list and every string it owns. Safe on null.
///
/// # Safety
///
/// `list` must have come from `trace_db_find_functions`.
#[no_mangle]
pub unsafe extern "C" fn trace_function_list_free(list: *mut TraceFunctionList) {
    if list.is_null() {
        return;
    }
    let impl_ = (*list)._impl;
    free_list::<TraceFunction>(impl_, list as *mut c_void, size_of::<TraceFunctionList>());
}

// ---------------------------------------------------------------------------
// Symbol lookup
// ---------------------------------------------------------------------------

fn symbol_kind(k: &str) -> TraceSymbolKind {
    match k {
        "global" => TraceSymbolKind::TraceSymGlobal,
        "file_static" => TraceSymbolKind::TraceSymFileStatic,
        "fn_static" => TraceSymbolKind::TraceSymFnStatic,
        "param" => TraceSymbolKind::TraceSymParam,
        "local" => TraceSymbolKind::TraceSymLocal,
        _ => TraceSymbolKind::TraceSymUnknown,
    }
}

fn symbol_kind_str(k: TraceSymbolKind) -> &'static str {
    match k {
        TraceSymbolKind::TraceSymGlobal => "global",
        TraceSymbolKind::TraceSymFileStatic => "file_static",
        TraceSymbolKind::TraceSymFnStatic => "fn_static",
        TraceSymbolKind::TraceSymParam => "param",
        TraceSymbolKind::TraceSymLocal => "local",
        TraceSymbolKind::TraceSymUnknown => "unknown",
    }
}

/// Find variables declared on/near `line` in files whose path contains
/// `file`, best candidate first. Fills `out`; free with
/// `trace_symbol_list_free`.
///
/// # Safety
///
/// `db` must be a live handle, `out` a valid destination, and the C strings
/// valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn trace_db_find_symbols(
    db: *mut TraceDb,
    file: *const c_char,
    line: i64,
    col: i64,
    out: *mut TraceSymbolList,
    out_err: *mut *mut c_char,
) -> c_int {
    reset_err(out_err);
    if db.is_null() || out.is_null() {
        return arg_err(out_err, "db and out must not be null");
    }
    let conn = &(*db).conn;
    let res = guard(|| {
        let file = cstr(file)?.to_owned();
        if file.is_empty() {
            return Err("invalid argument: file must not be empty".to_string());
        }
        let syms =
            trace_db::find_symbols_at(conn, &file, line, col).map_err(|e| format!("{e:#}"))?;
        let mut arena = Arena::new();
        let items = syms
            .into_iter()
            .map(|s| TraceSymbol {
                var_id: s.var_id,
                name: arena.add(&s.name),
                kind: symbol_kind(&s.kind),
                fn_name: arena.add_opt(s.fn_name.as_deref()),
                path: arena.add(&s.path),
                line: s.line,
                col: s.col,
            })
            .collect::<Vec<_>>();
        let (count, ptr, impl_) = leak_list(items, arena);
        Ok(TraceSymbolList {
            items: ptr,
            count,
            _impl: impl_,
        })
    });
    match res {
        Ok(list) => {
            *out = list;
            TraceStatus::TraceOk as c_int
        }
        Err(msg) => {
            set_error(out_err, &msg);
            status_for(&msg)
        }
    }
}

/// Free a symbol list and every string it owns. Safe on null.
///
/// # Safety
///
/// `list` must have come from `trace_db_find_symbols`.
#[no_mangle]
pub unsafe extern "C" fn trace_symbol_list_free(list: *mut TraceSymbolList) {
    if list.is_null() {
        return;
    }
    let impl_ = (*list)._impl;
    free_list::<TraceSymbol>(impl_, list as *mut c_void, size_of::<TraceSymbolList>());
}

// ---------------------------------------------------------------------------
// Call-edge listing (`calls`)
// ---------------------------------------------------------------------------

fn like_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if matches!(ch, '!' | '%' | '_') {
            out.push('!');
        }
        out.push(ch);
    }
    out
}

/// List call edges, optionally filtered by caller name (`from`), callee name
/// (`to`) and a path substring (`file`). `from`/`to` match an exact name or
/// a C++ `::`-qualified suffix (`--from Plugin` matches `ns::Plugin::foo`).
/// Fills `out`; free with `trace_call_edge_list_free`.
///
/// # Safety
///
/// `db` must be a live handle, `out` a valid destination. Filter strings may
/// be null (that filter is skipped) but must otherwise be valid for the
/// duration of the call.
#[no_mangle]
pub unsafe extern "C" fn trace_db_call_edges(
    db: *mut TraceDb,
    from: *const c_char,
    to: *const c_char,
    file: *const c_char,
    out: *mut TraceCallEdgeList,
    out_err: *mut *mut c_char,
) -> c_int {
    reset_err(out_err);
    if db.is_null() || out.is_null() {
        return arg_err(out_err, "db and out must not be null");
    }
    let conn = &(*db).conn;
    let res = guard(|| {
        let from = if from.is_null() {
            None
        } else {
            Some(cstr(from)?.to_owned())
        };
        let to = if to.is_null() {
            None
        } else {
            Some(cstr(to)?.to_owned())
        };
        let file = if file.is_null() {
            None
        } else {
            Some(cstr(file)?.to_owned())
        };

        let mut sql = String::from(
            "SELECT caller.id, caller.name, csf.path, callee.name, callee_f.path, ce.resolution, \
             cs.line, cs.col \
             FROM call_edges ce \
             JOIN call_sites cs ON cs.id = ce.call_site_id \
             JOIN functions caller ON caller.id = cs.caller_fn_id \
             JOIN files csf ON csf.id = cs.file_id \
             JOIN functions callee ON callee.id = ce.callee_fn_id \
             JOIN files callee_f ON callee_f.id = callee.file_id WHERE 1=1",
        );
        let mut params: Vec<String> = Vec::new();
        if let Some(f) = from.as_deref() {
            push_fn_name_filter(&mut sql, &mut params, "caller.name", f);
        }
        if let Some(t) = to.as_deref() {
            push_fn_name_filter(&mut sql, &mut params, "callee.name", t);
        }
        if let Some(p) = file.as_deref() {
            params.push(format!("%{}%", like_escape(p)));
            let n = params.len();
            sql.push_str(&format!(
                " AND (csf.path LIKE ?{n} ESCAPE '!' OR callee_f.path LIKE ?{n} ESCAPE '!')"
            ));
        }
        sql.push_str(" ORDER BY csf.path, cs.line");

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| format!("query failed: {e}"))?;
        let rows = stmt
            .query_map(rusqlite::params_from_iter(params.iter()), |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, i64>(7)?,
                ))
            })
            .map_err(|e| format!("query failed: {e}"))?;

        let mut arena = Arena::new();
        let mut items = Vec::new();
        for r in rows {
            let (caller_id, caller_name, caller_path, callee_name, callee_path, res, line, col) =
                r.map_err(|e| format!("query failed: {e}"))?;
            items.push(TraceCallEdge {
                caller_id,
                caller_name: arena.add(&caller_name),
                caller_path: arena.add(&caller_path),
                callee_name: arena.add(&callee_name),
                callee_path: arena.add(&callee_path),
                resolution: resolution_from_str(&res),
                path: arena.add(&caller_path),
                line: line as i32,
                col: col as i32,
            });
        }
        let (count, ptr, impl_) = leak_list(items, arena);
        Ok(TraceCallEdgeList {
            items: ptr,
            count,
            _impl: impl_,
        })
    });
    match res {
        Ok(list) => {
            *out = list;
            TraceStatus::TraceOk as c_int
        }
        Err(msg) => {
            set_error(out_err, &msg);
            status_for(&msg)
        }
    }
}

fn push_fn_name_filter(sql: &mut String, params: &mut Vec<String>, column: &str, name: &str) {
    params.push(name.to_string());
    let eq = params.len();
    params.push(format!("%::{}", like_escape(name)));
    let like = params.len();
    sql.push_str(&format!(
        " AND ({column} = ?{eq} OR {column} LIKE ?{like} ESCAPE '!')"
    ));
}

/// Free a call-edge list and every string it owns. Safe on null.
///
/// # Safety
///
/// `list` must have come from `trace_db_call_edges`.
#[no_mangle]
pub unsafe extern "C" fn trace_call_edge_list_free(list: *mut TraceCallEdgeList) {
    if list.is_null() {
        return;
    }
    let impl_ = (*list)._impl;
    free_list::<TraceCallEdge>(impl_, list as *mut c_void, size_of::<TraceCallEdgeList>());
}

fn resolution_from_str(s: &str) -> TraceResolution {
    match s {
        "direct" => TraceResolution::TraceResolutionDirect,
        "indirect" => TraceResolution::TraceResolutionIndirect,
        "ambiguous" => TraceResolution::TraceResolutionAmbiguous,
        "external" => TraceResolution::TraceResolutionExternal,
        _ => TraceResolution::TraceResolutionUnknown,
    }
}

fn flow_kind_from_str(s: &str) -> TraceFlowKind {
    match s {
        "copy" => TraceFlowKind::TraceFlowCopy,
        "addr_of" => TraceFlowKind::TraceFlowAddrOf,
        "load" => TraceFlowKind::TraceFlowLoad,
        "store" => TraceFlowKind::TraceFlowStore,
        "gep" => TraceFlowKind::TraceFlowGep,
        "points_to" => TraceFlowKind::TraceFlowPointsTo,
        "call_arg" => TraceFlowKind::TraceFlowCallArg,
        "terminates" => TraceFlowKind::TraceFlowTerminates,
        "dlsym" => TraceFlowKind::TraceFlowDlsym,
        _ => TraceFlowKind::TraceFlowUnknown,
    }
}

fn node_kind(k: Option<trace_db::FlowNodeKind>) -> TraceNodeKind {
    use trace_db::FlowNodeKind as K;
    match k {
        None => TraceNodeKind::TraceNodeUnknown,
        Some(K::Var) => TraceNodeKind::TraceNodeVar,
        Some(K::Loc) => TraceNodeKind::TraceNodeLoc,
        Some(K::CallTarget) => TraceNodeKind::TraceNodeCallTarget,
        Some(K::Terminator) => TraceNodeKind::TraceNodeTerminator,
    }
}

fn node_loc_kind(k: Option<trace_db::LocKind>) -> TraceLocKind {
    use trace_db::LocKind as L;
    match k {
        None => TraceLocKind::TraceLocUnknown,
        Some(L::Global) => TraceLocKind::TraceLocGlobal,
        Some(L::FileStatic) => TraceLocKind::TraceLocFileStatic,
        Some(L::FnStatic) => TraceLocKind::TraceLocFnStatic,
        Some(L::Local) => TraceLocKind::TraceLocLocal,
        Some(L::Heap) => TraceLocKind::TraceLocHeap,
        Some(L::Field) => TraceLocKind::TraceLocField,
        Some(L::FieldSummary) => TraceLocKind::TraceLocFieldSummary,
        Some(L::ArraySummary) => TraceLocKind::TraceLocArraySummary,
        Some(L::Function) => TraceLocKind::TraceLocFunction,
        Some(L::StringLit) => TraceLocKind::TraceLocStringLit,
    }
}

/// Owner of a graph result: the string arena plus the node/edge arrays whose
/// pointers are exposed to C.
struct GraphImpl {
    _arena: Arena,
    _nodes: Vec<TraceGraphNode>,
    _edges: Vec<TraceGraphEdge>,
}

/// Convert an inspect-layer `QueryGraph` into the C-facing `TraceGraph`,
/// keeping every string in one arena while the node/edge arrays reference it.
fn build_graph(g: &QueryGraph, is_callgraph: bool) -> TraceGraph {
    let mut arena = Arena::new();

    let mut nodes = Vec::with_capacity(g.nodes.len());
    for &(id, depth) in &g.order {
        let (kind, loc_kind, label, detail) = match g.nodes.get(&id) {
            Some(n) => (
                node_kind(n.kind),
                node_loc_kind(n.loc_kind),
                arena.add(&n.label),
                arena.add(&n.detail),
            ),
            None => (
                TraceNodeKind::TraceNodeUnknown,
                TraceLocKind::TraceLocUnknown,
                arena.add("?"),
                arena.add(""),
            ),
        };
        nodes.push(TraceGraphNode {
            id,
            depth: depth as i64,
            kind,
            loc_kind,
            label,
            detail,
        });
    }

    let mut edges = Vec::with_capacity(g.edges.len());
    for e in &g.edges {
        let (resolution, flow_kind) = if is_callgraph {
            (
                resolution_from_str(&e.label),
                TraceFlowKind::TraceFlowUnknown,
            )
        } else {
            (
                TraceResolution::TraceResolutionUnknown,
                flow_kind_from_str(&e.label),
            )
        };
        edges.push(TraceGraphEdge {
            from: e.from,
            to: e.to,
            resolution,
            flow_kind,
            path: arena.add(&e.path),
            line: e.line as i32,
            col: e.col as i32,
        });
    }

    let mut b = Box::new(GraphImpl {
        _arena: arena,
        _nodes: nodes,
        _edges: edges,
    });
    let n_nodes = b._nodes.len();
    let n_edges = b._edges.len();
    let nodes_ptr = b._nodes.as_mut_ptr();
    let edges_ptr = b._edges.as_mut_ptr();
    let _impl = Box::into_raw(b) as *mut c_void;

    TraceGraph {
        nodes: nodes_ptr,
        n_nodes,
        edges: edges_ptr,
        n_edges,
        truncated: g.truncated as i32,
        _impl,
    }
}

/// Free a graph result and every string it owns. Safe on null.
///
/// # Safety
///
/// `graph` must have come from `trace_db_callgraph` or `trace_db_dataflow`.
#[no_mangle]
pub unsafe extern "C" fn trace_graph_free(graph: *mut TraceGraph) {
    if graph.is_null() {
        return;
    }
    let impl_ = (*graph)._impl;
    if !impl_.is_null() {
        drop(Box::from_raw(impl_ as *mut GraphImpl));
    }
    ptr::write_bytes(graph.cast::<u8>(), 0, size_of::<TraceGraph>());
}

// ---------------------------------------------------------------------------
// Graph queries
// ---------------------------------------------------------------------------

/// Validate the raw `i32` direction crossed from C and map to the inspect
/// layer. C enums are ints; accepting `i32` (instead of the Rust enum) means
/// out-of-band values are ordinary inputs here, not UB.
unsafe fn check_dir(direction: i32, out_err: *mut *mut c_char) -> Option<trace_db::Direction> {
    match direction {
        0 => Some(trace_db::Direction::Down),
        1 => Some(trace_db::Direction::Up),
        other => {
            arg_err(
                out_err,
                &format!(
                    "direction must be TRACE_DIRECTION_DOWN (0) or TRACE_DIRECTION_UP (1), got {other}"
                ),
            );
            None
        }
    }
}

fn check_graph_args(db: *mut TraceDb, out: *mut TraceGraph, depth: u32) -> Option<&'static str> {
    if db.is_null() || out.is_null() {
        return Some("db and out must not be null");
    }
    if depth == 0 {
        return Some("depth must be >= 1");
    }
    None
}

/// Bounded BFS over the call graph from `root_fn_id`. Fills `out`; free with
/// `trace_graph_free`. `depth >= 1`.
///
/// # Safety
///
/// `db` must be a live handle, `out` a valid destination.
#[no_mangle]
pub unsafe extern "C" fn trace_db_callgraph(
    db: *mut TraceDb,
    root_fn_id: i64,
    direction: i32,
    depth: u32,
    out: *mut TraceGraph,
    out_err: *mut *mut c_char,
) -> c_int {
    reset_err(out_err);
    if let Some(msg) = check_graph_args(db, out, depth) {
        return arg_err(out_err, msg);
    }
    let Some(dir) = check_dir(direction, out_err) else {
        return TraceStatus::TraceErrInvalidArg as c_int;
    };
    let conn = &(*db).conn;
    let res = guard(|| {
        let g = trace_db::call_graph(conn, root_fn_id, dir, depth).map_err(|e| format!("{e:#}"))?;
        Ok(build_graph(&g, true))
    });
    match res {
        Ok(g) => {
            *out = g;
            TraceStatus::TraceOk as c_int
        }
        Err(msg) => {
            set_error(out_err, &msg);
            status_for(&msg)
        }
    }
}

/// Read a C array of `TraceSymbol` back into inspect-layer `SymbolRef`s
/// (round-trip of `trace_db_find_symbols` output into dataflow roots).
unsafe fn read_symbols(
    roots: *const TraceSymbol,
    n: usize,
) -> Result<Vec<trace_db::SymbolRef>, String> {
    if n == 0 {
        return Ok(Vec::new());
    }
    if roots.is_null() {
        return Err("n_roots > 0 but roots is null".to_string());
    }
    let slice = std::slice::from_raw_parts(roots, n);
    let mut out = Vec::with_capacity(n);
    for s in slice {
        let fn_name = if s.fn_name.is_null() {
            None
        } else {
            Some(CStr::from_ptr(s.fn_name).to_string_lossy().into_owned())
        };
        out.push(trace_db::SymbolRef {
            var_id: s.var_id,
            name: cstr(s.name)?.to_owned(),
            kind: symbol_kind_str(s.kind).to_owned(),
            fn_name,
            path: cstr(s.path)?.to_owned(),
            line: s.line,
            col: s.col,
        });
    }
    Ok(out)
}

/// Bounded BFS over the value-flow graph starting at the variables described
/// by `roots` (typically output of `trace_db_find_symbols`). Fills `out`;
/// free with `trace_graph_free`. `depth >= 1`.
///
/// # Safety
///
/// `db` must be a live handle, `out` a valid destination. When `n_roots > 0`,
/// `roots` must be a valid array of `n_roots` symbols whose strings are valid
/// for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn trace_db_dataflow(
    db: *mut TraceDb,
    roots: *const TraceSymbol,
    n_roots: usize,
    direction: i32,
    depth: u32,
    out: *mut TraceGraph,
    out_err: *mut *mut c_char,
) -> c_int {
    reset_err(out_err);
    if let Some(msg) = check_graph_args(db, out, depth) {
        return arg_err(out_err, msg);
    }
    if n_roots == 0 {
        return arg_err(out_err, "n_roots must be >= 1");
    }
    let Some(dir) = check_dir(direction, out_err) else {
        return TraceStatus::TraceErrInvalidArg as c_int;
    };
    let conn = &(*db).conn;
    let res = guard(|| {
        let syms = read_symbols(roots, n_roots)?;
        let g = trace_db::dataflow_graph(conn, &syms, dir, depth).map_err(|e| format!("{e:#}"))?;
        Ok(build_graph(&g, false))
    });
    match res {
        Ok(g) => {
            *out = g;
            TraceStatus::TraceOk as c_int
        }
        Err(msg) => {
            set_error(out_err, &msg);
            status_for(&msg)
        }
    }
}

/// Rust-side tests call the extern entry points directly.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::trace_index;
    use crate::types::TraceIndexOptions;
    use crate::util::trace_string_free;
    use std::ffi::CString;
    use std::path::Path;

    fn line_of(src: &str, needle: &str) -> i64 {
        let pos = src.find(needle).expect(needle);
        src[..pos].matches('\n').count() as i64 + 1
    }

    /// Build a tiny project in a temp dir, index it, open the result.
    /// If the test pre-wrote `main.c` (e.g. a multi-TU fixture), it is kept.
    fn analyze_fixture(dir: &Path) -> *mut TraceDb {
        if !dir.join("main.c").exists() {
            std::fs::write(dir.join("main.c"), MAIN_C).unwrap();
        }
        let root = CString::new(dir.to_str().unwrap()).unwrap();
        let out_path = dir.join("out.db");
        let out_c = CString::new(out_path.to_str().unwrap()).unwrap();
        let opts = TraceIndexOptions {
            size: std::mem::size_of::<TraceIndexOptions>(),
            root: root.as_ptr(),
            output_db: out_c.as_ptr(),
            includes: ptr::null(),
            n_includes: 0,
            defines: ptr::null(),
            n_defines: 0,
            jobs: 1,
            full_export: 1,
            debug_points_to: 0,
            models: ptr::null(),
            n_models: 0,
        };
        let mut result = TraceIndexResult {
            files: 0,
            functions: 0,
            call_edges: 0,
            arg_flow_edges: 0,
        };
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { trace_index(&opts, &mut result, &mut err) };
        assert_eq!(
            status,
            TraceStatus::TraceOk as c_int,
            "err={}",
            cstr_show(err)
        );
        if !err.is_null() {
            unsafe { trace_string_free(err) };
        }
        assert!(result.functions >= 2, "{result:?}");

        let db_path = CString::new(out_path.to_str().unwrap()).unwrap();
        let mut oerr: *mut c_char = ptr::null_mut();
        let db = unsafe { trace_db_open(db_path.as_ptr(), &mut oerr) };
        assert!(!db.is_null(), "err={}", cstr_show(oerr));
        if !oerr.is_null() {
            unsafe { trace_string_free(oerr) };
        }
        db
    }

    fn cstr_show(p: *const c_char) -> String {
        if p.is_null() {
            return "<null>".to_string();
        }
        unsafe { CStr::from_ptr(p).to_string_lossy().into_owned() }
    }

    const MAIN_C: &str = "/* demo project for the C API */\n\
         int helper(int *p) {\n\
         \x20   return *p;\n\
         }\n\
         \n\
         int global = 0;\n\
         \n\
         int main(void) {\n\
         \x20   int x = global;\n\
         \x20   x = helper(&x);\n\
         \x20   return x;\n\
         }\n";

    #[test]
    fn index_open_and_function_lookup() {
        let dir = tempfile::tempdir().unwrap();
        let db = analyze_fixture(dir.path());
        let line = line_of(MAIN_C, "int main(void)");
        let file = CString::new("main.c").unwrap();
        let mut list = TraceFunctionList {
            items: ptr::null_mut(),
            count: 0,
            _impl: ptr::null_mut(),
        };
        let mut err: *mut c_char = ptr::null_mut();
        let status =
            unsafe { trace_db_find_functions(db, file.as_ptr(), line, &mut list, &mut err) };
        assert_eq!(
            status,
            TraceStatus::TraceOk as c_int,
            "err={}",
            cstr_show(err)
        );
        assert_eq!(list.count, 1, "expected exactly main");
        let f = unsafe { &*list.items };
        assert_eq!(cstr_show(f.name), "main");
        assert!(cstr_show(f.path).ends_with("main.c"));
        assert!(f.is_defined != 0);
        assert_eq!(f.line_start, line);
        unsafe { trace_function_list_free(&mut list) };
        unsafe { trace_db_close(db) };
    }

    #[test]
    fn callgraph_edges_carry_file_line_col() {
        let dir = tempfile::tempdir().unwrap();
        let db = analyze_fixture(dir.path());

        // Root: main.
        let main_line = line_of(MAIN_C, "int main(void)");
        let file = CString::new("main.c").unwrap();
        let mut fl = TraceFunctionList {
            items: ptr::null_mut(),
            count: 0,
            _impl: ptr::null_mut(),
        };
        let mut err: *mut c_char = ptr::null_mut();
        assert_eq!(
            unsafe { trace_db_find_functions(db, file.as_ptr(), main_line, &mut fl, &mut err) },
            TraceStatus::TraceOk as c_int
        );
        let main_id = unsafe { (*fl.items).id };
        unsafe { trace_function_list_free(&mut fl) };

        let call_line = line_of(MAIN_C, "helper(&x)");
        let mut g = TraceGraph {
            nodes: ptr::null_mut(),
            n_nodes: 0,
            edges: ptr::null_mut(),
            n_edges: 0,
            truncated: 0,
            _impl: ptr::null_mut(),
        };
        assert_eq!(
            unsafe {
                trace_db_callgraph(
                    db,
                    main_id,
                    TraceDirection::TraceDirectionDown as i32,
                    3,
                    &mut g,
                    &mut err,
                )
            },
            TraceStatus::TraceOk as c_int,
            "err={}",
            cstr_show(err)
        );
        assert!(g.n_nodes >= 2);
        let edge = unsafe {
            (0..g.n_edges)
                .map(|i| &*g.edges.add(i))
                .find(|e| {
                    e.resolution == TraceResolution::TraceResolutionDirect
                        && e.line as i64 == call_line
                })
                .copied()
                .expect("main -> helper edge with the call-site line")
        };
        assert!(cstr_show(edge.path).ends_with("main.c"));
        assert!(edge.col > 0);
        // Call-graph edges are not flow edges.
        assert_eq!(edge.flow_kind, TraceFlowKind::TraceFlowUnknown);
        unsafe { trace_graph_free(&mut g) };
        unsafe { trace_db_close(db) };
    }

    #[test]
    fn symbols_and_dataflow_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let db = analyze_fixture(dir.path());

        let g_line = line_of(MAIN_C, "int global = 0");
        let file = CString::new("main.c").unwrap();
        let mut sl = TraceSymbolList {
            items: ptr::null_mut(),
            count: 0,
            _impl: ptr::null_mut(),
        };
        let mut err: *mut c_char = ptr::null_mut();
        assert_eq!(
            unsafe { trace_db_find_symbols(db, file.as_ptr(), g_line, 5, &mut sl, &mut err) },
            TraceStatus::TraceOk as c_int,
            "err={}",
            cstr_show(err)
        );
        assert!(sl.count >= 1);
        let sym = unsafe { *sl.items };
        assert_eq!(cstr_show(sym.name), "global");
        assert_eq!(sym.kind, TraceSymbolKind::TraceSymGlobal);
        assert!(sym.fn_name.is_null(), "global has no enclosing function");

        let mut g = TraceGraph {
            nodes: ptr::null_mut(),
            n_nodes: 0,
            edges: ptr::null_mut(),
            n_edges: 0,
            truncated: 0,
            _impl: ptr::null_mut(),
        };
        assert_eq!(
            unsafe {
                trace_db_dataflow(
                    db,
                    sl.items,
                    sl.count,
                    TraceDirection::TraceDirectionDown as i32,
                    4,
                    &mut g,
                    &mut err,
                )
            },
            TraceStatus::TraceOk as c_int,
            "err={}",
            cstr_show(err)
        );
        assert!(g.n_nodes >= 1);
        for i in 0..g.n_edges {
            let e = unsafe { &*g.edges.add(i) };
            assert_eq!(e.resolution, TraceResolution::TraceResolutionUnknown);
        }
        // Some flow edges exist and carry a flow kind.
        assert!(g.n_edges >= 1);
        let edge = unsafe { &*g.edges };
        assert_ne!(edge.flow_kind, TraceFlowKind::TraceFlowUnknown);
        // Dataflow edges have no source site.
        assert!(cstr_show(edge.path).is_empty());
        assert_eq!(edge.line, 0);
        assert_eq!(edge.col, 0);

        unsafe { trace_graph_free(&mut g) };
        unsafe { trace_symbol_list_free(&mut sl) };
        unsafe { trace_db_close(db) };
    }

    #[test]
    fn call_edges_filter_by_caller() {
        let dir = tempfile::tempdir().unwrap();
        let db = analyze_fixture(dir.path());
        let mut list = TraceCallEdgeList {
            items: ptr::null_mut(),
            count: 0,
            _impl: ptr::null_mut(),
        };
        let mut err: *mut c_char = ptr::null_mut();
        let from = CString::new("main").unwrap();
        assert_eq!(
            unsafe {
                trace_db_call_edges(
                    db,
                    from.as_ptr(),
                    ptr::null(),
                    ptr::null(),
                    &mut list,
                    &mut err,
                )
            },
            TraceStatus::TraceOk as c_int,
            "err={}",
            cstr_show(err)
        );
        assert!(list.count >= 1);
        let e = unsafe { &*list.items };
        assert_eq!(cstr_show(e.caller_name), "main");
        assert_eq!(cstr_show(e.callee_name), "helper");
        assert_eq!(e.resolution, TraceResolution::TraceResolutionDirect);
        unsafe { trace_call_edge_list_free(&mut list) };
        unsafe { trace_db_close(db) };
    }

    #[test]
    fn call_edge_path_is_the_call_site_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("main.c"),
            "void f(void);\nint main(void) { f(); return 0; }\n",
        )
        .unwrap();
        std::fs::write(dir.path().join("lib.c"), "void f(void) {}\n").unwrap();
        let db = analyze_fixture(dir.path());
        let mut list = TraceCallEdgeList {
            items: ptr::null_mut(),
            count: 0,
            _impl: ptr::null_mut(),
        };
        let mut err: *mut c_char = ptr::null_mut();
        assert_eq!(
            unsafe {
                trace_db_call_edges(
                    db,
                    ptr::null(),
                    ptr::null(),
                    ptr::null(),
                    &mut list,
                    &mut err,
                )
            },
            TraceStatus::TraceOk as c_int,
            "err={}",
            cstr_show(err)
        );
        assert!(list.count >= 1);
        let e = unsafe { &*list.items };
        // `path` must be the file containing the call, not the callee's file.
        assert!(
            cstr_show(e.path).ends_with("main.c"),
            "{}",
            cstr_show(e.path)
        );
        assert!(cstr_show(e.caller_path).ends_with("main.c"));
        assert!(cstr_show(e.callee_path).ends_with("lib.c"));
        assert_eq!(e.line, 2);
        unsafe { trace_call_edge_list_free(&mut list) };
        unsafe { trace_db_close(db) };
    }

    #[test]
    fn index_io_error_is_reportable() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("main.c"), MAIN_C).unwrap();
        let root = CString::new(dir.path().to_str().unwrap()).unwrap();
        // Output path under a directory that does not exist.
        let out = CString::new(format!("{}/no/such/dir/out.db", dir.path().display())).unwrap();
        let opts = TraceIndexOptions {
            size: std::mem::size_of::<TraceIndexOptions>(),
            root: root.as_ptr(),
            output_db: out.as_ptr(),
            includes: ptr::null(),
            n_includes: 0,
            defines: ptr::null(),
            n_defines: 0,
            jobs: 1,
            full_export: 0,
            debug_points_to: 0,
            models: ptr::null(),
            n_models: 0,
        };
        let mut result = TraceIndexResult {
            files: 0,
            functions: 0,
            call_edges: 0,
            arg_flow_edges: 0,
        };
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { trace_index(&opts, &mut result, &mut err) };
        assert_eq!(status, TraceStatus::TraceErrIo as c_int);
        assert!(!err.is_null());
        assert!(
            cstr_show(err).starts_with("i/o error:"),
            "{}",
            cstr_show(err)
        );
        unsafe { trace_string_free(err) };
    }

    #[test]
    fn failed_index_leaves_no_output_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("main.c"), MAIN_C).unwrap();
        let root = CString::new(dir.path().to_str().unwrap()).unwrap();
        let out_path = dir.path().join("out.db");
        assert!(!out_path.exists());
        let out_c = CString::new(out_path.to_str().unwrap()).unwrap();
        // Fail AFTER the preflight: a nonexistent models file makes
        // run_index bail at the merge step, not at the output probe.
        let bad_model =
            CString::new(format!("{}/no-such-models.toml", dir.path().display())).unwrap();
        let models = [bad_model.as_ptr()];
        let opts = TraceIndexOptions {
            size: std::mem::size_of::<TraceIndexOptions>(),
            root: root.as_ptr(),
            output_db: out_c.as_ptr(),
            includes: ptr::null(),
            n_includes: 0,
            defines: ptr::null(),
            n_defines: 0,
            jobs: 1,
            full_export: 0,
            debug_points_to: 0,
            models: models.as_ptr(),
            n_models: 1,
        };
        let mut result = TraceIndexResult {
            files: 0,
            functions: 0,
            call_edges: 0,
            arg_flow_edges: 0,
        };
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { trace_index(&opts, &mut result, &mut err) };
        assert_eq!(status, TraceStatus::TraceErrAnalysis as c_int);
        if !err.is_null() {
            unsafe { trace_string_free(err) };
        }
        // The index failed after the probe; no partial 0-byte file may exist.
        assert!(
            !out_path.exists(),
            "failed index must not leave a stale output file"
        );
    }

    #[test]
    fn open_missing_database_fails() {
        let dir = tempfile::tempdir().unwrap();
        let missing = CString::new(format!("{}/no-such.db", dir.path().display())).unwrap();
        let mut err: *mut c_char = ptr::null_mut();
        let db = unsafe { trace_db_open(missing.as_ptr(), &mut err) };
        assert!(db.is_null(), "opening a missing path must not succeed");
        assert!(!err.is_null(), "a missing path must set *out_err");
        unsafe { trace_string_free(err) };
    }

    #[test]
    fn errors_are_reported_and_errors_strings_freeable() {
        let dir = tempfile::tempdir().unwrap();
        let db = analyze_fixture(dir.path());
        let mut err: *mut c_char = ptr::null_mut();
        let mut g = TraceGraph {
            nodes: ptr::null_mut(),
            n_nodes: 0,
            edges: ptr::null_mut(),
            n_edges: 0,
            truncated: 0,
            _impl: ptr::null_mut(),
        };
        let status = unsafe {
            trace_db_callgraph(
                db,
                999_999,
                TraceDirection::TraceDirectionDown as i32,
                3,
                &mut g,
                &mut err,
            )
        };
        assert_eq!(status, TraceStatus::TraceErrNotFound as c_int);
        assert!(!err.is_null());
        assert!(cstr_show(err).contains("not found"));
        unsafe { trace_string_free(err) };
        unsafe { trace_db_close(db) };
    }

    #[test]
    fn abi_size_guard_rejects_too_small_options() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("main.c"), MAIN_C).unwrap();
        // A truncated consumer struct: only `size` + `root` fit.
        #[repr(C)]
        struct SmallOpts {
            size: usize,
            root: *const c_char,
        }
        let root = CString::new(dir.path().to_str().unwrap()).unwrap();
        let small = SmallOpts {
            size: std::mem::size_of::<SmallOpts>(),
            root: root.as_ptr(),
        };
        let mut result = TraceIndexResult {
            files: 0,
            functions: 0,
            call_edges: 0,
            arg_flow_edges: 0,
        };
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            trace_index(
                &small as *const SmallOpts as *const TraceIndexOptions,
                &mut result,
                &mut err,
            )
        };
        assert_eq!(status, TraceStatus::TraceErrInvalidArg as c_int);
        assert!(!err.is_null());
        assert!(cstr_show(err).contains("too small"));
        unsafe { trace_string_free(err) };
    }

    #[test]
    fn depth_zero_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let db = analyze_fixture(dir.path());
        let mut g = TraceGraph {
            nodes: ptr::null_mut(),
            n_nodes: 0,
            edges: ptr::null_mut(),
            n_edges: 0,
            truncated: 0,
            _impl: ptr::null_mut(),
        };
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            trace_db_callgraph(
                db,
                0,
                TraceDirection::TraceDirectionDown as i32,
                0,
                &mut g,
                &mut err,
            )
        };
        assert_eq!(status, TraceStatus::TraceErrInvalidArg as c_int);
        assert!(cstr_show(err).contains("depth"));
        unsafe { trace_db_close(db) };
    }

    #[test]
    fn out_of_band_direction_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let db = analyze_fixture(dir.path());
        let mut g = TraceGraph {
            nodes: ptr::null_mut(),
            n_nodes: 0,
            edges: ptr::null_mut(),
            n_edges: 0,
            truncated: 0,
            _impl: ptr::null_mut(),
        };
        let mut err: *mut c_char = ptr::null_mut();
        // 42 is not DOWN(0) or UP(1); must be rejected, not coerced.
        let status = unsafe { trace_db_callgraph(db, 0, 42i32, 3, &mut g, &mut err) };
        assert_eq!(status, TraceStatus::TraceErrInvalidArg as c_int);
        assert!(cstr_show(err).contains("direction"), "{}", cstr_show(err));
        assert!(cstr_show(err).contains("42"), "{}", cstr_show(err));
        unsafe { trace_db_close(db) };
    }

    #[test]
    fn empty_file_filter_is_invalid_argument() {
        let dir = tempfile::tempdir().unwrap();
        let db = analyze_fixture(dir.path());
        let mut list = TraceFunctionList {
            items: ptr::null_mut(),
            count: 0,
            _impl: ptr::null_mut(),
        };
        let mut err: *mut c_char = ptr::null_mut();
        let empty = CString::new("").unwrap();
        let status = unsafe { trace_db_find_functions(db, empty.as_ptr(), 1, &mut list, &mut err) };
        assert_eq!(status, TraceStatus::TraceErrInvalidArg as c_int);
        assert!(cstr_show(err).contains("file"), "{}", cstr_show(err));
        assert!(!err.is_null());
        unsafe { trace_string_free(err) };
        unsafe { trace_db_close(db) };
    }

    #[test]
    fn dataflow_without_roots_is_invalid_argument() {
        let dir = tempfile::tempdir().unwrap();
        let db = analyze_fixture(dir.path());
        let mut g = TraceGraph {
            nodes: ptr::null_mut(),
            n_nodes: 0,
            edges: ptr::null_mut(),
            n_edges: 0,
            truncated: 0,
            _impl: ptr::null_mut(),
        };
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            trace_db_dataflow(
                db,
                ptr::null(),
                0,
                TraceDirection::TraceDirectionDown as i32,
                3,
                &mut g,
                &mut err,
            )
        };
        assert_eq!(status, TraceStatus::TraceErrInvalidArg as c_int);
        assert!(cstr_show(err).contains("n_roots"), "{}", cstr_show(err));
        unsafe { trace_db_close(db) };
    }
}
