//! Regression tests for cross-TU resolution bugs found on a real HDF-scale
//! codebase: pointer-returning prototypes shadowed by phantom variables,
//! direct calls whose definition lives in another TU, and file-`static`
//! definitions that must shadow same-name external functions.

mod common;

use common::*;
use trace_analysis::{analyze, ResolutionKind};
use trace_ir::Linkage;
use trace_parse::build_program;

/// `struct Widget *WidgetGet(void);` is declared in a header and defined in
/// another TU. Lowering used to register a *variable* named `WidgetGet` for
/// the pointer-returning prototype, turning every call into an indirect call
/// through a variable that never receives function addresses (no edge).
#[test]
fn ptr_return_prototype_resolves_direct_edge() {
    let root = fixture("ptr_return_proto");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_edge(
            &program,
            &analysis,
            "CheckReady",
            "WidgetGet",
            ResolutionKind::Direct
        ),
        "cross-TU call to pointer-returning function must produce a direct edge"
    );

    // The prototype must not leak into the variable table.
    assert!(
        !program
            .symbols
            .variables
            .iter()
            .any(|v| v.name == "WidgetGet"),
        "prototype registered as phantom variable"
    );
}

/// A plain call to a function defined in another TU (no fn-ptr var) must
/// still yield a Direct edge after merge, even though lowering could not see
/// the callee in the calling TU.
#[test]
fn cross_tu_direct_call_recovers_edge() {
    let root = fixture("ptr_return_proto");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let edges = callees_of(&program, &analysis, "CheckReady");
    assert_eq!(edges.len(), 1, "exactly one edge from CheckReady");
    assert_eq!(edges[0].0, "WidgetGet");
    assert_eq!(edges[0].1, ResolutionKind::Direct);
}

/// Within a.c, the internal-linkage `helper` shadows b.c's external `helper`.
#[test]
fn static_definition_shadows_external_same_name() {
    let root = fixture("static_shadow");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let caller_a_id = program
        .symbols
        .functions
        .iter()
        .find(|f| f.name == "caller_a")
        .expect("caller_a exists")
        .id;
    let edges_to_helper: Vec<_> = analysis
        .call_edges
        .iter()
        .filter(|e| e.caller == caller_a_id)
        .map(|e| program.symbols.function(e.callee))
        .filter(|f| f.name == "helper")
        .collect();

    assert_eq!(edges_to_helper.len(), 1, "one helper edge from caller_a");
    assert_eq!(
        edges_to_helper[0].linkage,
        Linkage::Internal,
        "caller_a must bind to its own file-static helper"
    );

    // caller_b still binds to the external helper in b.c.
    let caller_b_id = program
        .symbols
        .functions
        .iter()
        .find(|f| f.name == "caller_b")
        .expect("caller_b exists")
        .id;
    let b_edges: Vec<_> = analysis
        .call_edges
        .iter()
        .filter(|e| e.caller == caller_b_id && fn_name(&program, e.callee) == "helper")
        .collect();
    assert_eq!(b_edges.len(), 1, "one helper edge from caller_b");
    assert_eq!(
        program.symbols.function(b_edges[0].callee).linkage,
        Linkage::External,
        "caller_b binds to the external helper"
    );
}

/// Arrays of structs with fn-ptr members, initialized with nested positional
/// initializer lists (`{ { FnA }, { FnB } }`), must feed ArrayFnMember facts
/// into the table var; an element field call resolves to every listed fn.
#[test]
fn nested_positional_init_table_resolves_members() {
    let root = fixture("fn_ptr_nested_table");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    for callee in ["FnA", "FnB"] {
        assert!(
            has_edge(
                &program,
                &analysis,
                "CallTbl",
                callee,
                ResolutionKind::Indirect
            ),
            "tbl[i].fn call must resolve to {callee}"
        );
    }
}

/// Same table shape with designated initializers inside the nested lists
/// (`{ .name = "..", .init = Fn }`), invoked through a pointer to an element.
#[test]
fn nested_designated_init_table_resolves_members() {
    let root = fixture("fn_ptr_nested_table");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    for callee in ["InitNet", "InitFs"] {
        assert!(
            has_edge(
                &program,
                &analysis,
                "CallMod",
                callee,
                ResolutionKind::Indirect
            ),
            "m->init call through &g_modules[i] must resolve to {callee}"
        );
    }
}

/// `&outer.member` must yield the member subobject location (typed by the
/// member's own struct), not the flattened outer instance. A Dispatch load
/// through `dev.service` must not pick up functions stored only in other
/// fields of the outer struct (HDF RegulatorTest.TestEntry vs
/// IDeviceIoService.Dispatch shared positional index 2).
#[test]
fn member_address_of_preserves_field_identity() {
    let root = fixture("fn_ptr_nested_table");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_edge(
            &program,
            &analysis,
            "InvokeTest",
            "EntryFn",
            ResolutionKind::Indirect
        ),
        "inst.TestEntry call must resolve to EntryFn"
    );
    assert!(
        has_edge(
            &program,
            &analysis,
            "CoreRun",
            "RealDispatch",
            ResolutionKind::Indirect
        ),
        "dev.service Dispatch call must resolve to RealDispatch"
    );
    assert!(
        !has_edge(
            &program,
            &analysis,
            "CoreRun",
            "EntryFn",
            ResolutionKind::Indirect
        ),
        "Dispatch load must not see fns stored in sibling fields of the outer struct"
    );
}

/// A `static inline` defined in a header and called from several TUs must
/// appear once, attributed to the header file (not once per including TU),
/// and its internal call sites must be deduplicated with header-origin
/// spans. Direct edges into/out of the canonical copy must survive.
#[test]
fn header_inline_calls_deduplicate_to_header_attribution() {
    let root = fixture("header_dedup");
    let program = build_program(&root, &default_opts(&root)).expect("build");

    let file_path = |program: &trace_ir::Program, id: trace_ir::FileId| -> String {
        program
            .symbols
            .files
            .iter()
            .find(|f| f.id == id)
            .map(|f| f.path.display().to_string())
            .unwrap_or_default()
    };

    let hdr_adds: Vec<_> = program
        .symbols
        .functions
        .iter()
        .filter(|f| f.name == "hdr_add")
        .collect();
    assert_eq!(hdr_adds.len(), 1, "hdr_add must collapse to one row");
    let hdr_add = hdr_adds[0];
    assert!(
        file_path(&program, hdr_add.span.file).ends_with("shared.h"),
        "hdr_add span must attribute to shared.h, got {}",
        file_path(&program, hdr_add.span.file)
    );
    assert_eq!(hdr_add.file, hdr_add.span.file);

    let helpers: Vec<_> = program
        .symbols
        .functions
        .iter()
        .filter(|f| f.name == "hdr_helper")
        .collect();
    assert_eq!(helpers.len(), 1, "hdr_helper must collapse to one row");
    let helper_id = helpers[0].id;

    // One deduplicated call site inside hdr_add, attributed to the header.
    let sites: Vec<_> = program
        .symbols
        .call_sites
        .iter()
        .filter(|cs| cs.caller == hdr_add.id && cs.callee_name == "hdr_helper")
        .collect();
    assert_eq!(sites.len(), 1, "duplicate hdr_helper call sites must merge");
    assert!(
        file_path(&program, sites[0].span.file).ends_with("shared.h"),
        "call site span must attribute to shared.h"
    );

    let (_pag, analysis) = analyze(&program);
    for (caller, callee) in [
        ("use_a", "hdr_add"),
        ("use_b", "hdr_add"),
        ("hdr_add", "hdr_helper"),
    ] {
        assert!(
            has_edge(&program, &analysis, caller, callee, ResolutionKind::Direct),
            "{caller} -> {callee} direct edge must survive dedup"
        );
    }
    assert_eq!(helpers[0].id, helper_id);

    // TU-local functions stay distinct.
    assert_eq!(
        program
            .symbols
            .functions
            .iter()
            .filter(|f| f.name == "use_a")
            .count(),
        1
    );
}

/// Functions referenced before their definition (no forward declaration) —
/// in a global designated initializer, in a function-body field store, and
/// through a fn-ptr variable initializer — must still resolve. Lowering
/// used to drop these silently when the definition had not been interned
/// yet (verified FN class on a real corpus).
#[test]
fn later_defined_fn_resolves_from_initializer_and_store() {
    let root = fixture("later_defined_init");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    // `g_tbl.init = LaterBody;` inside Bind(), defined after Bind.
    assert!(
        has_edge(
            &program,
            &analysis,
            "Caller",
            "LaterBody",
            ResolutionKind::Indirect
        ),
        "call through g_tbl.init must reach LaterBody despite definition order"
    );

    // `g_fp = LaterInit;` where LaterInit is defined below BindFp.
    assert!(
        has_edge(
            &program,
            &analysis,
            "UseFp",
            "LaterInit",
            ResolutionKind::Indirect
        ),
        "fn-ptr stored from a later-defined fn must flow"
    );
}

/// Plain-identifier calls to functions with no definition under the root
/// (declared-only or fully implicit) must be classified as External edges,
/// not left as unresolved indirect noise. Synthesized extern entries carry
/// `is_defined == false` and stay out of the variable table.
#[test]
fn unresolved_plain_ident_calls_become_external() {
    let root = fixture("extern_call");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    for callee in ["ext_helper", "undeclared_stub"] {
        assert!(
            has_edge(
                &program,
                &analysis,
                "local_wrap",
                callee,
                ResolutionKind::External
            ),
            "call to {callee} must produce an external edge"
        );
    }

    // The synthesized entries exist as bodyless functions...
    for callee in ["ext_helper", "undeclared_stub"] {
        let f = program
            .symbols
            .functions
            .iter()
            .find(|f| f.name == callee)
            .unwrap_or_else(|| panic!("{callee} must be interned"));
        assert!(!f.is_defined, "{callee} is not defined in the tree");
    }
    // ...and none of these sites leak indirect edges or phantom variables.
    for e in &analysis.call_edges {
        if fn_name(&program, e.caller) == "local_wrap" {
            assert_eq!(
                e.resolution,
                ResolutionKind::External,
                "external calls must not degrade into other resolutions"
            );
        }
    }
    assert!(!program
        .symbols
        .variables
        .iter()
        .any(|v| v.name == "ext_helper"));
    assert!(!program
        .symbols
        .variables
        .iter()
        .any(|v| v.name == "undeclared_stub"));
}

/// A call whose target is defined in another TU but has no prototype in the
/// caller must recover the REAL definition (Direct edge, arg-flow into its
/// body) — not be swallowed by external-callee synthesis.
#[test]
fn cross_tu_no_proto_call_recovers_definition() {
    let root = fixture("extern_call");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_edge(
            &program,
            &analysis,
            "caller",
            "helper",
            ResolutionKind::Direct
        ),
        "cross-TU call to a defined-but-undeclared function must produce a direct edge"
    );
    assert!(
        !has_edge(
            &program,
            &analysis,
            "caller",
            "helper",
            ResolutionKind::External
        ),
        "must not degrade into an external edge"
    );

    // The real definition keeps its identity: exactly one `helper`, defined.
    let helpers: Vec<_> = program
        .symbols
        .functions
        .iter()
        .filter(|f| f.name == "helper")
        .collect();
    assert_eq!(helpers.len(), 1, "no phantom duplicate rows for helper");
    assert!(helpers[0].is_defined);
}

/// HDF-shaped: struct in a header, designated `.Init = fn` in one TU, load
/// `entry->Init()` in another. PCH-style header IR must still connect them
/// through the field summary (HdfDeviceLaunchNode / DeviceDriverBind).
#[test]
fn cross_tu_designated_init_resolves_indirect() {
    let root = fixture("cross_tu_designated");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_edge(
            &program,
            &analysis,
            "launch",
            "my_init",
            ResolutionKind::Indirect
        ),
        "launch -> g_entry.Init must reach my_init stored in the other TU"
    );
    assert!(
        has_edge(
            &program,
            &analysis,
            "launch",
            "my_bind",
            ResolutionKind::Indirect
        ),
        "launch -> g_entry.Bind must reach my_bind stored in the other TU"
    );
}

/// HDF `DeviceNodeExtDispatch`: nested `host->service.Dispatch = Fn` where
/// `IDeviceIoService` lives in a different header than `StreamHost`. PCH
/// isolation used to intern `service` as an empty tag, so the Dispatch
/// store was dropped.
#[test]
fn nested_header_struct_field_store_resolves() {
    let root = fixture("nested_host_dispatch");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_edge(
            &program,
            &analysis,
            "launch",
            "StreamDispatch",
            ResolutionKind::Indirect
        ),
        "launch -> s->Dispatch must see StreamDispatch stored via host->service.Dispatch"
    );
}

/// HDF `GpioOnDevEventReceive`: `GpioIrqFunc` typedef in one header, field
/// `func` on a struct in another. PCH isolation typed the field as `Int`
/// and dropped fn-ptr arg-flow into `set_irq`.
#[test]
fn typedef_fnptr_field_store_resolves() {
    let root = fixture("typedef_fnptr_field");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_edge(
            &program,
            &analysis,
            "fire",
            "Handler",
            ResolutionKind::Indirect
        ),
        "fire -> p->func must see Handler stored through GpioIrqFunc"
    );

    // C++-parsed header prototype must collapse into the C definition so
    // `register_it` (in register.cpp) actually reaches `set_irq`'s body.
    let set_irq: Vec<_> = program
        .symbols
        .functions
        .iter()
        .filter(|f| f.name == "set_irq")
        .collect();
    assert!(
        set_irq.iter().any(|f| f.is_defined),
        "set_irq prototype must merge with the C definition"
    );
}

/// Different struct types with function pointers at the same positional index
/// (FieldId) but different field names must NOT leak across structs. Before
/// the field_name guard in the solver, GEP accesses into struct A would
/// pick up function pointers stored in struct B's same-index field, causing
/// massive false-positive indirect call edges (observed as 140 false
/// targets for HdfSbufReadBuffer in the real HDF corpus).
#[test]
fn cross_struct_field_id_no_pollution() {
    let root = fixture("fn_ptr_cross_struct");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    // CallWithOpsA loads "callback" — must resolve to CallbackImplA only.
    assert!(
        has_edge(
            &program,
            &analysis,
            "CallWithOpsA",
            "CallbackImplA",
            ResolutionKind::Indirect
        ),
        "CallWithOpsA must resolve to CallbackImplA"
    );
    assert!(
        !has_edge(
            &program,
            &analysis,
            "CallWithOpsA",
            "HandlerImplB",
            ResolutionKind::Indirect
        ),
        "CallWithOpsA must NOT see HandlerImplB from OpsB (cross-struct pollution)"
    );

    // CallWithOpsB loads "handler" — must resolve to HandlerImplB only.
    assert!(
        has_edge(
            &program,
            &analysis,
            "CallWithOpsB",
            "HandlerImplB",
            ResolutionKind::Indirect
        ),
        "CallWithOpsB must resolve to HandlerImplB"
    );
    assert!(
        !has_edge(
            &program,
            &analysis,
            "CallWithOpsB",
            "CallbackImplA",
            ResolutionKind::Indirect
        ),
        "CallWithOpsB must NOT see CallbackImplA from OpsA (cross-struct pollution)"
    );

    // CallBoth exercises both paths — verify it calls both dispatchers
    // directly, and that the indirect edges are inside them.
    for callee in ["CallWithOpsA", "CallWithOpsB"] {
        assert!(
            has_edge(
                &program,
                &analysis,
                "CallBoth",
                callee,
                ResolutionKind::Direct
            ),
            "CallBoth must directly call {callee}"
        );
    }

    // Total indirect edges must be exactly 2 (one per field load)
    let all_indirect: Vec<_> = analysis
        .call_edges
        .iter()
        .filter(|e| e.resolution == ResolutionKind::Indirect)
        .map(|e| (fn_name(&program, e.caller), fn_name(&program, e.callee)))
        .collect();
    assert_eq!(
        all_indirect.len(),
        2,
        "total indirect edges must be exactly 2, got: {:?}",
        all_indirect
    );
}
