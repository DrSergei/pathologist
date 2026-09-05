//! Integration tests for IPC proxy/stub bridge detection.

mod common;

use std::path::PathBuf;
use trace_analysis::{analyze, analyze_with_options, AnalyzeOptions, ResolutionKind};
use trace_parse::build_program;
use trace_preproc::PreprocessOptions;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(name)
}

fn build(
    name: &str,
) -> (
    trace_ir::Program,
    trace_analysis::Pag,
    trace_analysis::AnalysisResult,
) {
    let root = fixture(name);
    let include_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/include");
    let opts = PreprocessOptions::new()
        .with_include(root.clone())
        .with_include(include_dir);
    let program = build_program(&root, &opts).expect("build program");
    let (pag, analysis) = analyze(&program);
    (program, pag, analysis)
}

fn fn_name(program: &trace_ir::Program, id: trace_ir::FnId) -> String {
    program.symbols.function(id).name.clone()
}

fn has_bridge_edge(
    program: &trace_ir::Program,
    analysis: &trace_analysis::AnalysisResult,
    caller: &str,
    callee: &str,
) -> bool {
    analysis.call_edges.iter().any(|e| {
        fn_name(program, e.caller) == caller
            && fn_name(program, e.callee) == callee
            && e.resolution == ResolutionKind::IpcBridge
    })
}

#[test]
fn ipc_basic_bridges_proxy_to_stub() {
    let (program, pag, analysis) = build("ipc_basic");

    // Sanity: both classes are indexed.
    assert!(program
        .symbols
        .functions
        .iter()
        .any(|f| f.name.contains("IFooProxy")));
    assert!(program
        .symbols
        .functions
        .iter()
        .any(|f| f.name.contains("IFooStub")));

    // The bridge proxy→stub handlers must appear as IPC call edges.
    assert!(
        has_bridge_edge(
            &program,
            &analysis,
            "IFooProxy::GetInfo",
            "IFooStub::HandleGetInfo"
        ),
        "expected GetInfo → HandleGetInfo bridge edge, got: {:?}",
        analysis
            .call_edges
            .iter()
            .filter(|e| fn_name(&program, e.caller).contains("IFoo"))
            .map(|e| (fn_name(&program, e.caller), fn_name(&program, e.callee)))
            .collect::<Vec<_>>()
    );
    assert!(
        has_bridge_edge(
            &program,
            &analysis,
            "IFooProxy::SetInfo",
            "IFooStub::HandleSetInfo"
        ),
        "expected SetInfo → HandleSetInfo bridge edge"
    );
    assert!(
        !analysis.call_edges.iter().any(|e| {
            e.resolution == ResolutionKind::IpcBridge
                && fn_name(&program, e.caller) == "IFooProxy::LocalOnly"
        }),
        "a proxy method without SendRequest must not produce an IPC bridge"
    );

    // Bridges are recorded on the Pag.
    assert_eq!(pag.ipc_bridges.len(), 2);
}

#[test]
fn ipc_if_else_bridges_proxy_to_stub() {
    let (program, pag, analysis) = build("ipc_if_else");

    assert!(has_bridge_edge(
        &program,
        &analysis,
        "IThermalProxy::OnTemperatureChanged",
        "IThermalStub::OnTemperatureChanged"
    ));
    assert!(has_bridge_edge(
        &program,
        &analysis,
        "IThermalProxy::OnLevelChanged",
        "IThermalStub::OnLevelChanged"
    ));
    assert_eq!(pag.ipc_bridges.len(), 2);
}

#[test]
fn ipc_enum_bridges_proxy_to_stub() {
    let (program, pag, analysis) = build("ipc_enum");

    assert!(has_bridge_edge(
        &program,
        &analysis,
        "FooProxy::Add",
        "FooStub::Add"
    ));
    assert!(has_bridge_edge(
        &program,
        &analysis,
        "FooProxy::Query",
        "FooStub::Query"
    ));
    assert!(has_bridge_edge(
        &program,
        &analysis,
        "FooProxy::Destroy",
        "FooStub::Destroy"
    ));
    assert!(
        !has_bridge_edge(&program, &analysis, "FooProxy::Add", "FooStub::Add1"),
        "no spurious edge"
    );
    assert_eq!(pag.ipc_bridges.len(), 3);
}

#[test]
fn ipc_callback_bridges_callback_proxy_to_stub() {
    let (program, pag, analysis) = build("ipc_callback");

    assert!(has_bridge_edge(
        &program,
        &analysis,
        "ConnectionProxy::OnConnect",
        "ConnectionStub::OnConnect"
    ));
    assert!(has_bridge_edge(
        &program,
        &analysis,
        "ConnectionProxy::OnDisconnect",
        "ConnectionStub::OnDisconnect"
    ));
    assert_eq!(pag.ipc_bridges.len(), 2);
}

#[test]
fn ipc_stub_suffix_handler_fallback() {
    // A stub whose handlers are named only with a `Stub` suffix (no plain
    // interface-method name) is matched via the `{name}Stub` fallback.
    let (program, pag, analysis) = build("ipc_stub_suffix");

    assert!(has_bridge_edge(
        &program,
        &analysis,
        "FooProxy::OnFoo",
        "FooStub::OnFooStub"
    ));
    assert!(has_bridge_edge(
        &program,
        &analysis,
        "FooProxy::OnBar",
        "FooStub::OnBarStub"
    ));
    assert_eq!(pag.ipc_bridges.len(), 2);
}

#[test]
fn no_ipc_bridges_without_proxy_stub_pair() {
    // A fixture with no *Proxy/*Stub classes should produce no bridges.
    let (_program, pag, _analysis) = build("direct_call");
    assert_eq!(pag.ipc_bridges.len(), 0);
}

#[test]
fn ipc_interface_fallback_no_handler_methods() {
    // Stub with no handler method bodies — OnRemoteRequest calls inherited
    // interface methods directly. The fallback should detect bridges to
    // the interface methods (even though they are external).
    let (program, pag, _analysis) = build("ipc_interface_fallback");

    let bridge_names: Vec<_> = pag
        .ipc_bridges
        .iter()
        .map(|b| {
            (
                fn_name(&program, b.proxy_method),
                fn_name(&program, b.stub_handler),
            )
        })
        .collect();

    assert!(
        bridge_names
            .iter()
            .any(|(p, s)| p == "QueryResultProxy::HasNext" && s == "IQueryResult::HasNext"),
        "expected HasNext → IQueryResult::HasNext bridge, got: {:?}",
        bridge_names
    );
    assert!(
        bridge_names
            .iter()
            .any(|(p, s)| p == "QueryResultProxy::GetNext" && s == "IQueryResult::GetNext"),
        "expected GetNext → IQueryResult::GetNext bridge, got: {:?}",
        bridge_names
    );
    assert_eq!(pag.ipc_bridges.len(), 2);
    assert!(
        !bridge_names.iter().any(|(_, s)| s.starts_with("Other::")),
        "interface fallback must stay in the stub namespace: {bridge_names:?}"
    );
}

#[test]
fn ipc_bridge_export_has_no_call_site_and_keeps_its_caller() {
    let (program, pag, analysis) = build("ipc_basic");
    let db = common::export_program(&program, &pag, &analysis);
    let conn = trace_db::open_db(db.path()).expect("open exported database");

    let rows: Vec<(Option<i64>, String, String, String)> = {
        let mut stmt = conn
            .prepare(
                "SELECT ce.call_site_id, caller.name, callee.name, ce.resolution \
                 FROM call_edges ce \
                 JOIN functions caller ON caller.id = ce.caller_fn_id \
                 JOIN functions callee ON callee.id = ce.callee_fn_id \
                 WHERE ce.resolution = 'ipc' ORDER BY caller.name, callee.name",
            )
            .expect("prepare IPC edge query");
        stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .expect("query IPC edges")
        .collect::<Result<_, _>>()
        .expect("read IPC edges")
    };

    assert_eq!(rows.len(), 2);
    assert!(rows
        .iter()
        .all(|(site, _, _, resolution)| { site.is_none() && resolution == "ipc" }));
    assert!(rows.iter().any(|(_, caller, callee, _)| {
        caller == "IFooProxy::GetInfo" && callee == "IFooStub::HandleGetInfo"
    }));
}

#[test]
fn ipc_disabled_via_options() {
    // With enable_ipc = false, no bridge edges are emitted even when the
    // source contains a proxy/stub pair.
    let root = fixture("ipc_basic");
    let include_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/include");
    let opts = PreprocessOptions::new()
        .with_include(root.clone())
        .with_include(include_dir);
    let program = build_program(&root, &opts).expect("build program");

    let (_pag, analysis) = analyze_with_options(
        &program,
        AnalyzeOptions {
            enable_ipc: false,
            ..Default::default()
        },
    );
    let has_bridge = analysis.call_edges.iter().any(|e| {
        e.resolution == ResolutionKind::IpcBridge
            && fn_name(&program, e.caller) == "IFooProxy::GetInfo"
    });
    assert!(!has_bridge, "expected no bridge edges when IPC is disabled");
}
