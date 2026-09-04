//! C++ lowering integration tests (first-step C++ support).
#![allow(clippy::needless_borrow)]

mod common;

use std::sync::OnceLock;

use common::{default_opts, fixture, fn_name};
use trace_analysis::{analyze, AnalysisResult, ResolutionKind};
use trace_ir::{FnId, Program};
use trace_parse::build_program;

fn direct_targets(program: &Program, analysis: &AnalysisResult, caller: &str) -> Vec<String> {
    analysis
        .call_edges
        .iter()
        .filter(|e| fn_name(&program, e.caller) == caller && e.resolution == ResolutionKind::Direct)
        .map(|e| fn_name(&program, e.callee))
        .collect()
}

#[test]
fn cpp_virtual_dispatch_expands_to_overrides() {
    let root = fixture("cpp_basic");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let targets = direct_targets(&program, &analysis, "main");
    assert!(
        targets.iter().any(|t| t == "gfx::Shape::area"),
        "virtual s->area should target base Shape::area, got {targets:?}"
    );
    assert!(
        targets.iter().any(|t| t == "gfx::Circle::area"),
        "virtual s->area should target override Circle::area, got {targets:?}"
    );
}

#[test]
fn cpp_non_virtual_member_call_exact() {
    let root = fixture("cpp_basic");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let hits = analysis
        .call_edges
        .iter()
        .filter(|e| {
            fn_name(&program, e.caller) == "main"
                && fn_name(&program, e.callee) == "gfx::Shape::common"
        })
        .count();
    assert_eq!(hits, 1, "s->common must resolve to exactly one site-edge");

    let common = program
        .symbols
        .resolve_function("gfx::Shape::common")
        .expect("common defined");
    assert!(
        program.symbols.function(common).is_defined,
        "out-of-class definition must be the merged entry"
    );
}

#[test]
fn cpp_header_inline_method_dedups_with_out_of_class_uses() {
    let root = fixture("cpp_basic");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    // radius is defined inline in util.hpp; main calls it once.
    let hits = analysis
        .call_edges
        .iter()
        .filter(|e| fn_name(&program, e.callee) == "gfx::Circle::radius")
        .count();
    assert_eq!(hits, 1, "header-inline method should dedup across TUs");
}

/// A C++ class in a `.h` (not `.hpp`) must be parsed with the C++ grammar
/// under PCH-style header IR. Extension-only language would lower it as C
/// and drop CHA for out-of-line `Plugin::OnEventProxy`.
#[test]
fn cpp_dot_h_header_virtual_call_expands() {
    let root = fixture("cpp_h_header");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_direct(
            &program,
            &analysis,
            "Plugin::OnEventProxy",
            "Plugin::OnEvent"
        ),
        "implicit this->OnEvent from .h-declared Plugin"
    );
    assert!(
        has_direct(
            &program,
            &analysis,
            "Plugin::OnEventProxy",
            "Derived::OnEvent"
        ),
        "CHA must see Derived::OnEvent declared in plugin.h"
    );
    assert!(has_direct(
        &program,
        &analysis,
        "drive",
        "Plugin::OnEventProxy"
    ));
}

#[test]
fn cpp_ctor_and_dtor_sites() {
    let root = fixture("cpp_basic");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let targets = direct_targets(&program, &analysis, "main");
    assert!(
        targets.iter().any(|t| t == "gfx::Circle::Circle"),
        "new Circle() should emit ctor edge"
    );
    assert!(
        targets.iter().any(|t| t == "gfx::Shape::~Shape"),
        "delete via base ptr should emit base dtor"
    );
    assert!(
        targets.iter().any(|t| t == "gfx::Circle::~Circle"),
        "virtual dtor expansion should include derived dtor"
    );
}

#[test]
fn cpp_overload_resolution_by_arity() {
    let root = fixture("cpp_basic");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let add_edges: Vec<FnId> = analysis
        .call_edges
        .iter()
        .filter(|e| {
            fn_name(&program, e.caller) == "main"
                && fn_name(&program, e.callee).rsplit("::").next() == Some("add")
        })
        .map(|e| e.callee)
        .collect();
    assert_eq!(add_edges.len(), 2, "each arity resolves one overload");

    for callee in add_edges {
        let params = program.symbols.function(callee).params.len();
        let body_marks = direct_targets(&program, &analysis, &fn_name(&program, callee));
        if params == 2 {
            assert!(body_marks.contains(&"mark_i".to_string()));
        } else if params == 1 {
            assert!(body_marks.contains(&"mark_d".to_string()));
        } else {
            panic!("unexpected add overload with {params} params");
        }
    }
}

#[test]
fn cpp_namespace_qualified_call() {
    let root = fixture("cpp_basic");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);
    assert!(
        has_direct(&program, &analysis, "main", "util::tag"),
        "namespaced util::tag should be a direct callee of main"
    );
}

#[test]
fn cpp_anonymous_namespace_is_internal() {
    let root = fixture("cpp_basic");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        program.symbols.resolve_function("hidden").is_none(),
        "anon-namespace functions must not be in external lookup"
    );
    assert!(has_direct(&program, &analysis, "hidden", "util::tag"));
}

fn has_direct(program: &Program, analysis: &AnalysisResult, caller: &str, callee: &str) -> bool {
    analysis.call_edges.iter().any(|e| {
        fn_name(&program, e.caller) == caller
            && fn_name(&program, e.callee) == callee
            && e.resolution == ResolutionKind::Direct
    })
}

// --- cpp_more: overload ties, templates, multiple inheritance,
// ctor-initializer lists, static member functions ---

fn edges_to(
    program: &Program,
    analysis: &AnalysisResult,
    caller: &str,
    callee_suffix: &str,
    resolution: ResolutionKind,
) -> Vec<String> {
    analysis
        .call_edges
        .iter()
        .filter(|e| {
            fn_name(&program, e.caller) == caller
                && fn_name(&program, e.callee).ends_with(callee_suffix)
                && e.resolution == resolution
        })
        .map(|e| fn_name(&program, e.callee))
        .collect()
}

#[test]
fn cpp_overload_tie_emits_both_sites() {
    let root = fixture("cpp_more");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let hits = edges_to(&program, &analysis, "drive", "tie", ResolutionKind::Direct);
    assert_eq!(
        hits.len(),
        2,
        "same-arity overload tie must emit one site per candidate"
    );
}

#[test]
fn cpp_template_class_method_resolves_by_primary_name() {
    let root = fixture("cpp_more");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_direct(&program, &analysis, "drive", "Box::put"),
        "Box<Widget>::put call should resolve under primary name"
    );
}

#[test]
fn cpp_virtual_call_through_base_of_multiple_inheritance() {
    let root = fixture("cpp_more");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(has_direct(&program, &analysis, "drive", "A::fa"));
    assert!(
        has_direct(&program, &analysis, "drive", "AB::fa"),
        "virtual expansion must include the multiple-inheritance override"
    );
    assert!(!has_direct(&program, &analysis, "drive", "B::fb"));
}

#[test]
fn cpp_ctor_initializer_list_targets() {
    let root = fixture("cpp_more");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(has_direct(&program, &analysis, "D::D", "Base::Base"));
    assert!(has_direct(&program, &analysis, "D::D", "Member::Member"));
    // D d2(5): constructor-declaration with argument list.
    assert!(has_direct(&program, &analysis, "drive", "D::D"));
}

#[test]
fn cpp_static_member_function_resolves() {
    let root = fixture("cpp_more");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let hits = edges_to(
        &program,
        &analysis,
        "drive",
        "S::Make",
        ResolutionKind::Direct,
    );
    assert!(hits.len() >= 2, "both S::Make calls should resolve");
}

#[test]
fn cpp_inherited_non_virtual_via_derived_receiver() {
    let root = fixture("cpp_more");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_direct(&program, &analysis, "drive", "Base::base_value"),
        "d2.base_value() should walk up to Base"
    );
    assert!(has_direct(&program, &analysis, "sink_w", "Widget::make"));
}

// --- cpp_implicit_this: bare method calls, smart_ptr unwrap ---

#[test]
fn cpp_implicit_this_virtual_call_expands() {
    let root = fixture("cpp_implicit_this");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(has_direct(&program, &analysis, "drive", "Base::go"));
    let hooks = analysis
        .call_edges
        .iter()
        .filter(|e| {
            fn_name(&program, e.caller) == "Base::go" && e.resolution == ResolutionKind::Direct
        })
        .map(|e| fn_name(&program, e.callee))
        .collect::<Vec<_>>();
    assert!(
        hooks.iter().any(|t| t == "Base::hook"),
        "implicit this->hook should hit Base::hook, got {hooks:?}"
    );
    assert!(
        hooks.iter().any(|t| t == "Derived::hook"),
        "virtual expansion should include Derived::hook, got {hooks:?}"
    );
}

#[test]
fn cpp_smart_ptr_member_call_unwraps_pointee() {
    let root = fixture("cpp_implicit_this");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_direct(&program, &analysis, "call_sp", "Plugin::OnEventProxy"),
        "shared_ptr<Plugin> p; p->OnEventProxy should type as Plugin"
    );
    assert!(
        has_direct(&program, &analysis, "call_sp_ref", "Plugin::OnEventProxy"),
        "const shared_ptr<Plugin> & should unwrap to Plugin"
    );
    assert!(
        has_direct(
            &program,
            &analysis,
            "Plugin::OnEventProxy",
            "Plugin::OnEvent"
        ),
        "OnEventProxy body implicit this->OnEvent"
    );
    assert!(
        has_direct(&program, &analysis, "call_up", "Plugin::OnEventProxy"),
        "unique_ptr<Plugin> should unwrap like shared_ptr"
    );
    assert!(
        has_direct(&program, &analysis, "call_wp", "Plugin::OnEventProxy"),
        "weak_ptr<Plugin> should unwrap like shared_ptr"
    );
}

#[test]
fn cpp_smart_ptr_field_receiver_unwraps() {
    let root = fixture("cpp_implicit_this");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_direct(&program, &analysis, "Holder::go", "Plugin::OnEvent"),
        "plugin_->OnEvent on a shared_ptr field should type as Plugin"
    );
}

#[test]
fn cpp_member_virtual_overload_filters_by_arity() {
    let root = fixture("cpp_implicit_this");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let unary: Vec<(String, usize)> = analysis
        .call_edges
        .iter()
        .filter(|e| {
            fn_name(&program, e.caller) == "call_unary"
                && e.resolution == ResolutionKind::Direct
                && fn_name(&program, e.callee).ends_with("::foo")
        })
        .map(|e| {
            (
                fn_name(&program, e.callee),
                program.symbols.function(e.callee).params.len(),
            )
        })
        .collect();
    assert!(
        unary.iter().any(|(t, _)| t == "Over::foo") && unary.iter().any(|(t, _)| t == "OverD::foo"),
        "p->foo(1) should CHA to unary Over::foo / OverD::foo, got {unary:?}"
    );
    for (t, n) in &unary {
        assert_eq!(*n, 2, "{t} should be this+int, params={n}, all={unary:?}");
    }

    let binary: Vec<(String, usize)> = analysis
        .call_edges
        .iter()
        .filter(|e| {
            fn_name(&program, e.caller) == "call_binary"
                && e.resolution == ResolutionKind::Direct
                && fn_name(&program, e.callee).ends_with("::foo")
        })
        .map(|e| {
            (
                fn_name(&program, e.callee),
                program.symbols.function(e.callee).params.len(),
            )
        })
        .collect();
    for (t, n) in &binary {
        assert_eq!(
            *n, 3,
            "{t} should be this+int+int, params={n}, all={binary:?}"
        );
    }
}

#[test]
fn cpp_unused_attr_on_ref_param_keeps_definition() {
    let root = fixture("cpp_implicit_this");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let sink = program
        .symbols
        .resolve_function("Sink::consume")
        .expect("Sink::consume");
    assert!(
        program.symbols.function(sink).is_defined,
        "T& param __UNUSED must remain a function_definition"
    );
}

// --- cpp_callable: lambdas, std::function, functors, fn-ptr fields ---

fn has_resolution(
    program: &Program,
    analysis: &AnalysisResult,
    caller: &str,
    callee: &str,
    resolution: ResolutionKind,
) -> bool {
    analysis.call_edges.iter().any(|e| {
        fn_name(&program, e.caller) == caller
            && fn_name(&program, e.callee) == callee
            && e.resolution == resolution
    })
}

#[test]
fn cpp_fn_ptr_field_and_local_resolve_indirect() {
    let root = fixture("cpp_callable");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(has_resolution(
        &program,
        &analysis,
        "call_field",
        "target",
        ResolutionKind::Indirect
    ));
    assert!(has_resolution(
        &program,
        &analysis,
        "call_local",
        "target",
        ResolutionKind::Indirect
    ));
}

#[test]
fn cpp_lambda_is_addr_of_fn_and_indirect_call() {
    let root = fixture("cpp_callable");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let lambda_names: Vec<String> = program
        .symbols
        .functions
        .iter()
        .filter(|f| f.name.contains("$lambda"))
        .map(|f| f.name.clone())
        .collect();
    assert!(
        !lambda_names.is_empty(),
        "lambda_expression should lower to a $lambda function"
    );
    assert!(
        analysis.call_edges.iter().any(|e| {
            fn_name(&program, e.caller).contains("$lambda")
                && fn_name(&program, e.callee) == "target"
        }),
        "lambda body should call target, lambdas={lambda_names:?}"
    );
    assert!(
        analysis.call_edges.iter().any(|e| {
            fn_name(&program, e.caller) == "call_lambda"
                && fn_name(&program, e.callee).contains("$lambda")
                && e.resolution == ResolutionKind::Indirect
        }),
        "g() should be an indirect call to the lambda"
    );
}

#[test]
fn cpp_functor_operator_call_resolves() {
    let root = fixture("cpp_callable");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_direct(&program, &analysis, "call_functor", "Fn::operator()"),
        "f() on a functor should target operator()"
    );
    assert!(
        has_direct(&program, &analysis, "call_functor_field", "Fn::operator()"),
        "w->cb() when cb is a functor field should target operator()"
    );
    assert!(has_direct(&program, &analysis, "Fn::operator()", "target"));
    assert!(
        has_direct(
            &program,
            &analysis,
            "call_bare_function_type",
            "function::operator()"
        ),
        "a class named function (not std::function) should still be a functor"
    );
}

#[test]
fn cpp_std_function_resolves_like_fn_ptr() {
    let root = fixture("cpp_callable");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_resolution(
            &program,
            &analysis,
            "call_std_function",
            "target",
            ResolutionKind::Indirect
        ),
        "std::function local assigned a function should call it indirectly"
    );
    assert!(
        has_resolution(
            &program,
            &analysis,
            "call_std_field",
            "target",
            ResolutionKind::Indirect
        ),
        "std::function field call should resolve like a fn-ptr field"
    );
}

#[test]
fn cpp_qualified_undeclared_becomes_external() {
    let root = fixture("cpp_callable");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        has_resolution(
            &program,
            &analysis,
            "check_exists",
            "FileUtil::Exists",
            ResolutionKind::External
        ),
        "qualified FileUtil::Exists prototype should be an external edge, not unresolved indirect"
    );
}

// --- cpp_flow: cross-language C dispatcher + C++ impl (HDF sbuf pattern) ---

#[test]
fn cpp_impl_registered_into_c_ops_table_resolves_indirect() {
    let root = fixture("cpp_flow");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    for target in ["RawImplRead", "MParcelImplRead"] {
        let hits = edges_to(
            &program,
            &analysis,
            "Read",
            target,
            ResolutionKind::Indirect,
        );
        assert_eq!(
            hits.len(),
            1,
            "{target} must be an indirect target of s->impl->read exactly once"
        );
    }
}

// --- cpp_dispatch: virtual inheritance + final class/method ---

fn cpp_direct_set(program: &Program, analysis: &AnalysisResult, caller: &str) -> Vec<String> {
    let mut v = direct_targets(program, analysis, caller);
    v.sort();
    v.dedup();
    v
}

#[test]
fn cpp_virtual_inheritance_diamond_resolves_overrides() {
    let root = fixture("cpp_dispatch");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        program.bases_of("Left").iter().any(|b| b == "VBase"),
        "virtual base Left : virtual VBase must be recorded"
    );
    assert!(
        program.bases_of("Right").iter().any(|b| b == "VBase"),
        "virtual base Right : virtual VBase must be recorded"
    );
    let hits = cpp_direct_set(&program, &analysis, "diamond_drive");
    assert!(
        hits.iter().any(|t| t == "VBase::id"),
        "diamond through VBase* should include VBase::id, got {hits:?}"
    );
    assert!(
        hits.iter().any(|t| t == "Left::id"),
        "diamond through VBase* should include Left::id, got {hits:?}"
    );
    assert!(
        hits.iter().any(|t| t == "Diamond::id"),
        "diamond through VBase* should include Diamond::id, got {hits:?}"
    );
}

#[test]
fn cpp_final_class_devirtualizes_receiver() {
    let root = fixture("cpp_dispatch");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(
        program.class_is_final("Sealed"),
        "class Sealed final must be recorded"
    );
    let sealed = cpp_direct_set(&program, &analysis, "sealed_drive");
    assert_eq!(
        sealed,
        vec!["Sealed::f".to_string()],
        "Sealed* is final: only Sealed::f, not OpenSib::f"
    );
    let open = cpp_direct_set(&program, &analysis, "open_drive");
    assert!(open.iter().any(|t| t == "Open::f"), "got {open:?}");
    assert!(open.iter().any(|t| t == "Sealed::f"), "got {open:?}");
    assert!(open.iter().any(|t| t == "OpenSib::f"), "got {open:?}");
}

#[test]
fn cpp_final_method_stops_further_overrides() {
    let root = fixture("cpp_dispatch");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let mid_fn = program
        .symbols
        .resolve_function("MMid::g")
        .expect("MMid::g");
    assert!(
        program.symbols.function(mid_fn).is_final,
        "int g() final must set is_final"
    );
    let mid = cpp_direct_set(&program, &analysis, "mid_drive");
    assert_eq!(
        mid,
        vec!["MMid::g".to_string()],
        "MMid* with g() final is a unique target"
    );
    let base = cpp_direct_set(&program, &analysis, "mbase_drive");
    assert!(base.iter().any(|t| t == "MBase::g"), "got {base:?}");
    assert!(base.iter().any(|t| t == "MMid::g"), "got {base:?}");
    assert!(
        !base.iter().any(|t| t.contains("MLeaf")),
        "final method must not pick up MLeaf, got {base:?}"
    );
}

// --- cpp_extern_c_driver: C caller + C++ `extern "C"` heap/ops registration ---

#[test]
fn cpp_extern_c_driver_resolves_ipc_and_dispatch() {
    let root = fixture("cpp_extern_c_driver");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    assert!(has_direct(
        &program,
        &analysis,
        "test_ipc_read",
        "SbufObtainIpc"
    ));
    assert!(has_resolution(
        &program,
        &analysis,
        "test_ipc_read",
        "MParcelReadBuffer",
        ResolutionKind::Indirect
    ));
    assert!(has_direct(
        &program,
        &analysis,
        "test_ipc_dispatch",
        "GetServiceOps"
    ));
    assert!(has_resolution(
        &program,
        &analysis,
        "test_ipc_dispatch",
        "ServiceDispatch",
        ResolutionKind::Indirect
    ));
}

// --- cpp_templates_overloads: scalar-type overload resolution, template
// member calls with explicit arguments, in-class template methods ---

/// Param type descriptors of a function, in signature order.
fn fn_param_descs(program: &Program, id: FnId) -> Vec<String> {
    program
        .symbols
        .function(id)
        .params
        .iter()
        .map(|v| {
            let tid = program.symbols.variable(*v).type_id;
            format!("{:?}", program.types.get(tid).desc)
        })
        .collect()
}

#[test]
fn cpp_same_arity_overloads_stay_distinct_by_scalar_type() {
    let root = fixture("cpp_templates_overloads");
    let program = build_program(&root, &default_opts(&root)).expect("build");

    let candidates = program.symbols.resolve_function_candidates("f", None);
    let sigs: Vec<Vec<String>> = candidates
        .iter()
        .map(|&f| fn_param_descs(&program, f))
        .collect();
    assert!(
        sigs.contains(&vec!["Int".to_string()]),
        "f(int) must survive as its own overload, got {sigs:?}"
    );
    assert!(
        sigs.contains(&vec!["Double".to_string()]),
        "f(double) must survive as its own overload, got {sigs:?}"
    );
    assert!(
        sigs.contains(&vec!["Short".to_string()]),
        "f(short) must survive as its own overload, got {sigs:?}"
    );
    assert!(
        sigs.contains(&vec!["Int".to_string(), "Int".to_string()]),
        "f(int, int) must survive as its own overload, got {sigs:?}"
    );
    let distinct: std::collections::HashSet<_> = sigs.iter().cloned().collect();
    assert_eq!(distinct.len(), 4, "all four signatures distinct: {sigs:?}");
}

#[test]
fn cpp_call_sites_prefer_exact_scalar_match() {
    let root = fixture("cpp_templates_overloads");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    for (lit_call, descs) in [
        ("f(1)", vec!["Int"]),
        ("f(1.5)", vec!["Double"]),
        ("f(s)", vec!["Short"]),
        ("f(1, 2)", vec!["Int", "Int"]),
    ] {
        let matching: Vec<FnId> = analysis
            .call_edges
            .iter()
            .filter(|e| {
                fn_name(&program, e.caller) == "main"
                    && fn_name(&program, e.callee) == "f"
                    && e.resolution == ResolutionKind::Direct
                    && fn_param_descs(&program, e.callee) == descs
            })
            .map(|e| e.callee)
            .collect();
        assert_eq!(
            matching.len(),
            1,
            "{lit_call} must pick exactly one overload with {descs:?}, got {matching:?}"
        );
    }

    // No call site may emit more than one edge: the type-resolved overload is
    // unambiguous rather than the may-tie set.
    let multi = analysis.call_edges.iter().any(|e| {
        analysis
            .call_edges
            .iter()
            .filter(|e2| e2.call_site == e.call_site)
            .count()
            > 1
    });
    assert!(
        !multi,
        "type-resolved overloads must emit one site per call"
    );
}

#[test]
fn cpp_template_member_calls_resolve_to_primary_name() {
    let root = fixture("cpp_templates_overloads");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    // Template primary registrations exist.
    let candidates = program
        .symbols
        .resolve_function_candidates("FieldValue::GetNumber", None);
    assert_eq!(
        candidates.len(),
        3,
        "in-class template GetNumber must register alongside its overloads"
    );

    // `fv.GetNumber<int>()` and `b.read<short>()` resolve directly.
    assert!(
        has_direct(&program, &analysis, "main", "FieldValue::GetNumber"),
        "fv.GetNumber<int>() must resolve to FieldValue::GetNumber"
    );
    assert!(
        has_direct(&program, &analysis, "main", "Box::read"),
        "b.read<short>() must resolve to Box::read"
    );
    assert!(
        has_direct(&program, &analysis, "main", "Box::read")
            && has_direct(&program, &analysis, "main", "FieldValue::GetNumber"),
        "template member calls must be direct, not external stubs"
    );
}

#[test]
fn cpp_pointer_casts_rank_against_pointer_overloads() {
    let root = fixture("cpp_pointer_cast_overloads");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    let sig_count = |name: &str, sigs: Vec<String>| {
        analysis
            .call_edges
            .iter()
            .filter(|e| {
                fn_name(&program, e.caller) == "main" && e.resolution == ResolutionKind::Direct
            })
            .filter(|e| fn_name(&program, e.callee) == name)
            .filter(|e| fn_param_descs(&program, e.callee) == sigs)
            .count()
    };
    let int = vec!["Int".to_string()];
    let ptr_int = vec!["Ptr(Int)".to_string()];
    let ch = vec!["Char".to_string()];
    let ptr_ch = vec!["Ptr(Char)".to_string()];
    let ptr_ptr_int = vec!["Ptr(Ptr(Int))".to_string()];

    assert_eq!(
        sig_count("f", int.clone()),
        1,
        "f(i) must pick exactly f(int)"
    );
    assert_eq!(
        sig_count("f", ptr_int),
        2,
        "f((int*)&i) and f(pi) must resolve to f(int*), not f(int)"
    );
    assert_eq!(
        sig_count("f", ch.clone()),
        1,
        "f(c) must pick f(char), not f(char*)"
    );
    assert_eq!(
        sig_count("f", ptr_ch),
        2,
        "f((char*)&c) and f(pc) must resolve to f(char*)"
    );
    assert_eq!(
        sig_count("f", ptr_ptr_int.clone()),
        2,
        "f((int**)&pi) and f(pp) must resolve to f(int**), not one pointer level short"
    );
    let f_direct_total = analysis
        .call_edges
        .iter()
        .filter(|e| {
            fn_name(&program, e.caller) == "main"
                && fn_name(&program, e.callee) == "f"
                && e.resolution == ResolutionKind::Direct
        })
        .count();
    assert_eq!(
        f_direct_total, 8,
        "all eight f() call sites must resolve to exactly one callee each"
    );
}

#[test]
fn cpp_unresolvable_member_args_keep_full_candidate_set() {
    let root = fixture("cpp_pointer_cast_overloads");
    let program = build_program(&root, &default_opts(&root)).expect("build");
    let (_pag, analysis) = analyze(&program);

    // `g(gh.val)` and `g(hp->val)` cannot be ranked past the receiver
    // (struct or pointer-to-struct), so BOTH the int and the Holder overload
    // stay for each member call (may-approximation) — five edges total:
    // g(42) -> g(int) only, plus two member calls each keeping both.
    let g_targets: Vec<Vec<String>> = analysis
        .call_edges
        .iter()
        .filter(|e| {
            fn_name(&program, e.caller) == "main"
                && fn_name(&program, e.callee) == "g"
                && e.resolution == ResolutionKind::Direct
        })
        .map(|e| fn_param_descs(&program, e.callee))
        .collect();
    assert_eq!(
        g_targets.len(),
        5,
        "g(42) + g(gh.val) + g(hp->val) must contribute 1 + 2 + 2 edges, got {g_targets:?}"
    );
    let mut seen: Vec<Vec<String>> = g_targets.clone();
    seen.sort();
    seen.dedup();
    assert!(
        seen.contains(&vec!["Int".to_string()]),
        "g(int) must be present, got {g_targets:?}"
    );
    assert!(
        seen.iter()
            .any(|s| !s.is_empty() && s[0].starts_with("Struct")),
        "g(Holder) must be among the kept candidates (both receiver shapes), got {g_targets:?}"
    );
}

// --- cpp_name_lookup: ADL, using directives, namespace-relative lookup ---

fn cpp_name_lookup() -> (Program, trace_analysis::AnalysisResult) {
    static SHARED: OnceLock<(Program, trace_analysis::AnalysisResult)> = OnceLock::new();
    SHARED
        .get_or_init(|| {
            let root = fixture("cpp_name_lookup");
            let program = build_program(&root, &default_opts(&root)).expect("build");
            let (_pag, analysis) = analyze(&program);
            (program, analysis)
        })
        .clone()
}

#[test]
fn cpp_adl_free_function_resolves() {
    let (program, analysis) = cpp_name_lookup();
    // `swap(_a, _b)` at global scope with `kit::Widget*` args: ADL finds
    // `kit::swap`. It must be a direct in-tree edge, not an external stub.
    assert!(
        has_resolution(
            &program,
            &analysis,
            "adl_drive",
            "kit::swap",
            ResolutionKind::Direct
        ),
        "ADL swap(kit::Widget*) must resolve to kit::swap"
    );
    assert!(
        !program
            .symbols
            .functions
            .iter()
            .any(|f| f.name == "swap" && !f.is_defined),
        "bare 'swap' must not survive as an undefined external stub"
    );
}

#[test]
fn cpp_using_namespace_resolves_free_functions() {
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "using_ns_drive",
            "util::helper",
            ResolutionKind::Direct
        ),
        "using namespace util; helper() must resolve"
    );
    assert!(
        has_resolution(
            &program,
            &analysis,
            "using_ns_drive",
            "util::twice",
            ResolutionKind::Direct
        ),
        "using namespace util; twice(3) must resolve"
    );
}

#[test]
fn cpp_using_member_import_resolves() {
    let (program, analysis) = cpp_name_lookup();
    // `using lib::bump;` imports the exact qualified function.
    assert!(
        has_resolution(
            &program,
            &analysis,
            "using_member_drive",
            "lib::bump",
            ResolutionKind::Direct
        ),
        "using lib::bump; bump(c) must resolve to the imported function"
    );
}

#[test]
fn cpp_using_import_of_static_resolves_internal_linkage() {
    let (program, analysis) = cpp_name_lookup();
    // `using import_static::only;` + `only(1)` must resolve to the file-local
    // static `import_static::only(int)` (internal linkage), not degrade to
    // the global external/overload or an external stub.
    assert!(
        has_resolution(
            &program,
            &analysis,
            "using_static_drive",
            "import_static::only",
            ResolutionKind::Direct
        ),
        "using import_static::only; only(1) must resolve to the static definition"
    );
}

#[test]
fn cpp_namespace_relative_call_resolves() {
    let (program, analysis) = cpp_name_lookup();
    // From inside `a::b`, bare `clamp` finds the innermost `a::b::clamp`.
    assert!(
        has_resolution(
            &program,
            &analysis,
            "a::b::go",
            "a::b::clamp",
            ResolutionKind::Direct
        ),
        "bare clamp() inside a::b must resolve to a::b::clamp"
    );
}

#[test]
fn cpp_qualified_call_unchanged() {
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "qualified_drive",
            "util::helper",
            ResolutionKind::Direct
        ),
        "util::helper() must still resolve explicitly"
    );
    assert!(has_resolution(
        &program,
        &analysis,
        "qualified_drive",
        "util::twice",
        ResolutionKind::Direct
    ));
}

#[test]
fn cpp_header_prototypes_register_qualified_names() {
    // Header-declared `void swap(Widget*, Widget*)` inside `namespace kit`
    // must register as `kit::swap` (not bare `swap`), so it folds into the
    // out-of-line definition and ADL resolves exactly once.
    let (program, _) = cpp_name_lookup();
    let proto = program.symbols.functions_named("kit::swap");
    assert!(
        proto
            .iter()
            .any(|&f| program.symbols.function(f).is_defined),
        "kit::swap must have its in-tree definition registered"
    );
    // The header must not leave a bare `swap` *external stub* — the whole
    // point of qualifying prototypes. (A deliberate global `swap`
    // definition in main.cpp is fine and expected.)
    assert!(
        !program
            .symbols
            .functions
            .iter()
            .any(|f| f.name == "swap" && !f.is_defined),
        "the header must not produce an undefined bare 'swap' external stub"
    );
}

// --- additional name-lookup edge cases ---

#[test]
fn cpp_adl_may_approx_keeps_global_overload() {
    // A global `swap(Widget*, Widget*)` and `kit::swap(Widget*, Widget*)`
    // share base name + arity. Under may-analysis the bare `swap(_a, _b)`
    // call must keep BOTH candidates (global + ADL namespace), never
    // collapse to a single wrong target.
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "adl_may_approx",
            "swap",
            ResolutionKind::Direct
        ),
        "global ::swap must remain a candidate"
    );
    assert!(
        has_resolution(
            &program,
            &analysis,
            "adl_may_approx",
            "kit::swap",
            ResolutionKind::Direct
        ),
        "ADL kit::swap must remain a candidate"
    );
    assert!(
        !has_resolution(
            &program,
            &analysis,
            "adl_may_approx",
            "swap",
            ResolutionKind::External
        ),
        "both candidates are defined in-tree; neither may degrade to external"
    );
}

#[test]
fn cpp_using_nested_member_import_resolves() {
    // `using deep::inner::fold;` — a *nested* qualified import that no
    // ordinary/ADL namespace covers.
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "adl_nested_import",
            "deep::inner::fold",
            ResolutionKind::Direct
        ),
        "using deep::inner::fold must resolve the nested import"
    );
}

#[test]
fn cpp_file_static_shadows_adl() {
    // A file-scope `static void shadowed(int)` must resolve ahead of any
    // global/ADL candidate of the same base name (internal linkage wins).
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "adl_static_shadow",
            "shadowed",
            ResolutionKind::Direct
        ),
        "file-local static shadowed() must resolve"
    );
    assert!(
        !program
            .symbols
            .functions
            .iter()
            .any(|f| f.name == "shadowed" && !f.is_defined),
        "static shadowed must not leave an external stub"
    );
}

#[test]
fn cpp_function_scoped_using_namespace_resolves() {
    // `using namespace body;` inside a function body must make `poke()`
    // resolvable only for that function.
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "adl_function_scoped_using",
            "body::poke",
            ResolutionKind::Direct
        ),
        "function-scoped using namespace body; poke() must resolve"
    );
}

#[test]
fn cpp_function_scoped_using_namespace_does_not_leak() {
    // The `using namespace body;` inside `adl_function_scoped_using` must NOT
    // make `body::poke` a candidate in `adl_using_no_leak` — a leaked
    // directive would rob the correct in-scope global `poke` edge when the
    // ranking later collapses to one candidate (under-approximation).
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "adl_using_no_leak",
            "poke",
            ResolutionKind::Direct
        ),
        "global poke must resolve for a caller without the using directive"
    );
    assert!(
        !has_resolution(
            &program,
            &analysis,
            "adl_using_no_leak",
            "body::poke",
            ResolutionKind::Direct
        ),
        "function-body using namespace must not leak into other functions"
    );
}

#[test]
fn cpp_relative_using_namespace_target_finds_enclosing_namespace() {
    // `using namespace detail;` is written inside `relns::via_directive`
    // while an *enclosing* `relns::detail` namespace exists. C++ resolves
    // the relative first segment to the enclosing namespace, so
    // `drive_ns`'s bare `bump(1)` must reach `relns::detail::bump` (and may
    // over-approximate the global `detail::bump` too; it must not miss the
    // enclosing one).
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "relns::directive_host::user::drive_ns",
            "relns::detail::bump",
            ResolutionKind::Direct
        ),
        "relative using-namespace target must resolve against the enclosing namespace"
    );
}

#[test]
fn cpp_relative_using_member_target_finds_enclosing_namespace() {
    // `using detail::bump;` written inside `relns::via_import` names the
    // enclosing `relns::detail::bump` (first segment resolved against the
    // namespace stack), which must end up in `drive_import`'s candidate set
    // — not just the global-spelled `detail::bump`.
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "relns::import_host::user::drive_import",
            "relns::detail::bump",
            ResolutionKind::Direct
        ),
        "relative using-declaration target must resolve against the enclosing namespace"
    );
}

#[test]
fn cpp_global_qualified_definition_inside_namespace_block() {
    // `void ::qualified_global() {}` written inside `namespace global_block`
    // registers at global scope under the normalized name `qualified_global`
    // (leading `::` stripped by `qualify_decl` so that merge dedup works and
    // `functions_in_namespace` needs only one comparison).  The enclosing
    // namespace prefix must NOT be prepended.
    // `global_block::caller`'s bare call must reach the global function.
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "global_block::caller",
            "qualified_global",
            ResolutionKind::Direct
        ),
        "::global definition inside a namespace block must stay at global scope"
    );
}

#[test]
fn cpp_namespace_scoped_using_namespace_applies_inside_block_only() {
    // `using namespace boost_ish;` lives inside `scoped_use::inner`. It must
    // apply to `in_scope` but not leak to `scoped_use::out_of_scope` (which
    // is in the enclosing namespace, declared after the block). A TU-wide
    // leak would make `out_of_scope` bind to the better-ranking
    // `boost_ish::tick(int)` and drop the correct global `tick(double)` edge.
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "scoped_use::inner::in_scope",
            "boost_ish::tick",
            ResolutionKind::Direct
        ),
        "in-scope caller must resolve through the block-scoped directive"
    );
    assert!(
        has_resolution(
            &program,
            &analysis,
            "scoped_use::out_of_scope",
            "tick",
            ResolutionKind::Direct
        ),
        "caller outside the block must fall back to the in-scope global tick"
    );
    assert!(
        !has_resolution(
            &program,
            &analysis,
            "scoped_use::out_of_scope",
            "boost_ish::tick",
            ResolutionKind::Direct
        ),
        "namespace-block using namespace must not leak into the enclosing namespace"
    );
}

#[test]
fn cpp_adl_free_function_direct_in_one_of_many_candidates() {
    // Sanity: the original ADL drive still resolves exactly through ADL with
    // the additional global overload present.
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "adl_drive",
            "kit::swap",
            ResolutionKind::Direct
        ),
        "adl_drive swap must still resolve to kit::swap"
    );
}

#[test]
fn cpp_inner_block_using_namespace_applies_inside_block_only() {
    // `using namespace innerlib;` inside the `if` body must apply only to
    // that block. Two `g()` call sites in one function: the one inside the
    // block resolves through `innerlib::g`; the sibling call after the block
    // must stay on the global `g`. A directive leaked to the whole function
    // would add `innerlib::g` to the sibling call site too (over-approx that
    // can collapse the ranking and rob the correct in-scope edge) — so
    // `innerlib::g` must appear exactly once (the in-block call).
    let (program, analysis) = cpp_name_lookup();
    let innerlib_edges = analysis
        .call_edges
        .iter()
        .filter(|e| {
            fn_name(&program, e.caller) == "inner_block_using_scoped"
                && fn_name(&program, e.callee) == "innerlib::g"
                && e.resolution == ResolutionKind::Direct
        })
        .count();
    assert_eq!(
        innerlib_edges, 1,
        "inner-block using namespace must not leak to the sibling call site \
         (expected exactly 1 innerlib::g edge, from the in-block call)"
    );
    assert!(
        has_resolution(
            &program,
            &analysis,
            "inner_block_using_scoped",
            "g",
            ResolutionKind::Direct
        ),
        "sibling call after the block must resolve to the global g"
    );
}

#[test]
fn cpp_adl_leading_global_scope_tag_finds_namespace() {
    // `::kit::LeadWidget` (global-scope spelling) must still derive ADL
    // namespace `kit` (the leading `::` is the global marker, not part of
    // the namespace), so the bare `lead_swap` resolves to `kit::lead_swap`.
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "adl_leading_global_scope_tag",
            "kit::lead_swap",
            ResolutionKind::Direct
        ),
        "leading-:: ADL tag must resolve through ADL to kit::lead_swap"
    );
}

#[test]
fn cpp_inner_namespace_hides_global_overload() {
    // `hide::g() { f(1); }` with a global `::f(int)` and an inner
    // `hide::f(double)`. The bare name inside `hide` must resolve to
    // `hide::f` only — the global `::f` is a wrong single answer and must be
    // dropped (its presence must not be re-added by an out-of-band global
    // lookup that runs ahead of the hiding walk).
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "hide::g",
            "hide::f",
            ResolutionKind::Direct
        ),
        "inner-namespace declaration must shadow the global overload"
    );
    assert!(
        !has_resolution(&program, &analysis, "hide::g", "f", ResolutionKind::Direct),
        "global f(int) must be hidden by hide::f, not kept as a candidate"
    );
}

#[test]
fn cpp_inner_namespace_hides_global_static() {
    // `hidesf::g() { sf(1); }` with a global file-scope `static sf(int)` and
    // an inner `hidesf::sf(double)`. The nested namespace declaration must
    // shadow the file-static, resolving to `hidesf::sf` only — not the
    // wrong single global-static answer.
    let (program, analysis) = cpp_name_lookup();
    assert!(
        has_resolution(
            &program,
            &analysis,
            "hidesf::g",
            "hidesf::sf",
            ResolutionKind::Direct
        ),
        "inner-namespace declaration must shadow the global file-static"
    );
    assert!(
        !has_resolution(
            &program,
            &analysis,
            "hidesf::g",
            "sf",
            ResolutionKind::Direct
        ),
        "global static sf must be hidden by hidesf::sf, not kept as a candidate"
    );
}

/// Every function name the index holds for `src`, lowered as C++.
fn member_names(tag: &str, src: &str) -> Vec<String> {
    let dir = tempfile::Builder::new()
        .prefix(&format!("trace_{tag}_"))
        .tempdir()
        .unwrap();
    let root = dir.path();
    std::fs::write(root.join("k.cpp"), src).unwrap();
    let program = build_program(root, &default_opts(root)).expect("build");
    let mut names: Vec<String> = program
        .symbols
        .functions
        .iter()
        .map(|f| f.name.clone())
        .collect();
    names.sort();
    names
}

#[test]
fn decltype_return_type_does_not_swallow_the_member_name() {
    // Issue #29: `member_short_name` walked the whole field_declaration in
    // order and took the first `identifier` it met. In a `decltype(...)`
    // return type that identifier belongs to the *operand expression*, so
    // `decltype(*p_) Deref() const;` was indexed as the member `p_` at the
    // decltype's line and `Deref` was dropped — silently, with no
    // diagnostic, since the file parses cleanly.
    let names = member_names(
        "decltype_ret",
        "class K {\n\
         public:\n\
         \x20   decltype(*p_) Deref() const;\n\
         \x20   decltype(kSize) Sized() const;\n\
         \x20   int Plain() const;\n\
         \x20   int *p_;\n\
         };\n",
    );
    assert!(
        names.iter().any(|n| n == "K::Deref"),
        "decltype-returning member must be indexed: {names:?}"
    );
    assert!(
        names.iter().any(|n| n == "K::Sized"),
        "a decltype over a plain identifier too: {names:?}"
    );
    assert!(
        !names.iter().any(|n| n == "K::p_" || n == "K::kSize"),
        "the decltype operand must not be indexed as the member: {names:?}"
    );
    assert!(names.iter().any(|n| n == "K::Plain"), "{names:?}");
}
