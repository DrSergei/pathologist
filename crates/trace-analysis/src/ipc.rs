//! IPC proxy/stub bridge detection.
//!
//! OpenHarmony services communicate over Binder IPC: a proxy method calls
//! `remote->SendRequest(...)` and a stub dispatches in `OnRemoteRequest`.
//! When both proxy and stub live under the analyzed root, we can connect them
//! with a synthetic call edge so the call graph has no gap at the IPC
//! boundary.
//!
//! Detection is purely name-based — no control-flow / opcode analysis:
//!
//! - A **stub** class is identified by its name ending in `Stub`.
//! - A **proxy** class is identified by its name ending in `Proxy` or `Client` plus
//!   the presence of a `SendRequest`-family call in the method body.
//! - Bridges pair proxy methods to stub handlers by interface class name +
//!   method name correspondence (e.g. `FooProxy::Bar` → `FooStub::Bar`).

use rustc_hash::{FxHashMap, FxHashSet};
use trace_ir::{FnId, IpcBridge, Program};

/// `(qualified_class_name, handler FnIds)` for each detected stub class.
type StubClasses = Vec<(String, Vec<FnId>)>;
/// `(qualified_class_name, simple_method_name, method_fn)` for each
/// IPC-sending proxy method.
type ProxyMethods = Vec<(String, String, FnId)>;

/// Detect proxy/stub IPC bridges in a post-merge program.
///
/// Pure: reads `program` and returns the matched bridges. Runs after merge,
/// during PAG build. No `Program` mutation required.
pub fn detect_ipc_pairs(program: &Program) -> Vec<IpcBridge> {
    let (stubs, proxies) = scan(program);

    let mut stub_index: FxHashMap<String, &Vec<FnId>> = FxHashMap::default();
    for (class, handlers) in &stubs {
        stub_index.insert(class.clone(), handlers);
    }

    let mut bridges: Vec<IpcBridge> = Vec::new();
    for (proxy_class, method, proxy_method) in &proxies {
        let stub_class = derive_stub_class(proxy_class);
        let Some(handlers) = stub_index.get(&stub_class) else {
            continue;
        };
        let matched_handlers = find_handlers(program, handlers, method);
        if !matched_handlers.is_empty() {
            bridges.extend(matched_handlers.into_iter().map(|stub_handler| IpcBridge {
                proxy_method: *proxy_method,
                stub_handler,
                descriptor: String::new(),
            }));
            continue;
        }
        // Fallback: stub has no handler methods (only dispatcher + boilerplate).
        // The stub's OnRemoteRequest switch calls interface methods directly on
        // `this` (inherited from the parent interface). Match proxy methods
        // against external (interface) functions with the same simple name.
        bridges.extend(
            find_interface_methods(program, &stub_class, method)
                .into_iter()
                .map(|stub_handler| IpcBridge {
                    proxy_method: *proxy_method,
                    stub_handler,
                    descriptor: String::new(),
                }),
        );
    }
    bridges
}

/// Returns the stub classes and the IPC-sending proxy methods collected from
/// a post-merge program.
fn scan(program: &Program) -> (StubClasses, ProxyMethods) {
    // Index IPC-sending methods once. Scanning the entire call-site list for
    // every proxy method is quadratic on proxy-heavy trees.
    let senders: FxHashSet<FnId> = program
        .symbols
        .call_sites
        .iter()
        .filter(|cs| cs.callee_name.contains("SendRequest"))
        .map(|cs| cs.caller)
        .collect();

    // Index all defined C++ methods by their qualified class.
    // (qualified_class → (simple_method_name, FnId)).
    let mut methods_by_class: FxHashMap<String, Vec<(String, FnId)>> = FxHashMap::default();
    for f in &program.symbols.functions {
        if !f.is_defined || !f.is_cpp {
            continue;
        }
        let Some((class, method)) = split_qualified(&f.name) else {
            continue;
        };
        methods_by_class
            .entry(class)
            .or_default()
            .push((method, f.id));
    }

    let mut stubs: StubClasses = Vec::new();
    let mut proxies: ProxyMethods = Vec::new();
    let mut seen_stub = std::collections::HashSet::new();

    for (class, methods) in &methods_by_class {
        if is_stub_class(class) {
            // A stub class: handlers are its methods that are not the
            // dispatcher/descriptor boilerplate.
            let handlers: Vec<FnId> = methods
                .iter()
                .filter(|(m, _)| !is_stub_entry(m) && !is_boilerplate(class, m))
                .map(|(_, id)| *id)
                .collect();
            // Register stubs with no handler methods too — the interface
            // fallback needs to find them. A stub must have either handler
            // methods OR OnRemoteRequest (the dispatcher) to be registered.
            let has_dispatcher = methods.iter().any(|(m, _)| is_stub_entry(m));
            if (!handlers.is_empty() || has_dispatcher) && seen_stub.insert(class.clone()) {
                stubs.push((class.clone(), handlers));
            }
        } else if is_proxy_class(class) {
            // A proxy class: its methods that call SendRequest are IPC sends.
            for (method, id) in methods {
                if senders.contains(id) {
                    proxies.push((class.clone(), method.clone(), *id));
                }
            }
        }
    }

    (stubs, proxies)
}

/// Derive the matching stub class name from a proxy/client class name.
/// `FooProxy` → `FooStub`, `FooClient` → `FooStub`.
fn derive_stub_class(proxy_class: &str) -> String {
    if let Some(base) = proxy_class.strip_suffix("Proxy") {
        return format!("{base}Stub");
    }
    if let Some(base) = proxy_class.strip_suffix("Client") {
        return format!("{base}Stub");
    }
    proxy_class.to_string()
}

/// Find stub handlers matching a proxy method name. Tries, in order:
/// exact name, a `Handle` prefix variant, then a `Stub` suffix variant
/// (the marshalling shim name used by some IDL generators).
/// Returns every overload at the first tier with any matches: name-based IPC
/// detection cannot distinguish overloads, so may-analysis retains them all.
/// Candidate resolution follows the symbol table's scope/overload rules and
/// keeps only definitions; declarations are left to the interface fallback.
fn find_handlers(program: &Program, handlers: &[FnId], method_name: &str) -> Vec<FnId> {
    for name in [
        method_name.to_string(),
        format!("Handle{method_name}"),
        format!("{method_name}Stub"),
    ] {
        let matching_entries: Vec<FnId> = handlers
            .iter()
            .copied()
            .filter(|&id| {
                program
                    .symbols
                    .function(id)
                    .name
                    .ends_with(&format!("::{name}"))
            })
            .collect();
        let mut seen = FxHashSet::default();
        let mut matches = Vec::new();
        for id in matching_entries {
            let matched = program.symbols.function(id);
            for candidate in program
                .symbols
                .resolve_function_candidates(&matched.name, Some(matched.file))
            {
                let function = program.symbols.function(candidate);
                if function.is_defined && seen.insert(candidate) {
                    matches.push(candidate);
                }
            }
        }
        if !matches.is_empty() {
            return matches;
        }
    }
    Vec::new()
}

/// Fallback for stubs with no handler methods: find an external (interface)
/// function whose simple name matches the proxy method. The stub's
/// `OnRemoteRequest` switch calls these interface methods directly on `this`
/// (inherited from the parent interface class).
fn find_interface_methods(program: &Program, stub_class: &str, method_name: &str) -> Vec<FnId> {
    // Prefer concrete in-tree implementations deriving from the stub. This
    // keeps reachability alive beyond the IPC boundary when the stub itself
    // only declares the interface methods.
    let mut concrete = Vec::new();
    let mut seen = FxHashSet::default();
    for class in program.subclass_closure(stub_class).into_iter().skip(1) {
        let name = format!("{class}::{method_name}");
        for id in program.symbols.resolve_function_candidates(&name, None) {
            if program.symbols.function(id).is_defined && seen.insert(id) {
                concrete.push(id);
            }
        }
    }
    if !concrete.is_empty() {
        return concrete;
    }

    // The stub base is the class name with "Stub" stripped,
    // e.g. `OHOS::HiviewDFX::FaultLogQueryResultStub` → `FaultLogQueryResult`.
    let stub_base = stub_class.strip_suffix("Stub").unwrap_or(stub_class);
    let (stub_namespace, stub_simple) = stub_base.rsplit_once("::").unwrap_or(("", stub_base));

    program
        .symbols
        .functions
        .iter()
        .filter(|f| {
            !f.is_defined && f.name.ends_with(&format!("::{method_name}")) && {
                let class_part = f.name.rsplit_once("::").map(|(c, _)| c).unwrap_or("");
                if class_part == stub_class {
                    false
                } else {
                    // Strip leading 'I' from the interface class name to get
                    // the base name, e.g. `IFaultLogQueryResult` → `FaultLogQueryResult`.
                    let (iface_namespace, iface_simple) =
                        class_part.rsplit_once("::").unwrap_or(("", class_part));
                    let iface_base = iface_simple.strip_prefix('I').unwrap_or(iface_simple);
                    // Match when the base names coincide, e.g.
                    // `FaultLogQueryResult` == `FaultLogQueryResult`.
                    !iface_base.is_empty()
                        && iface_namespace == stub_namespace
                        && (iface_simple == stub_simple || iface_base == stub_simple)
                }
            }
        })
        .map(|f| f.id)
        .collect()
}

fn is_stub_class(class: &str) -> bool {
    class.ends_with("Stub")
}

fn is_stub_entry(method: &str) -> bool {
    method == "OnRemoteRequest"
}

fn is_boilerplate(class: &str, method: &str) -> bool {
    let class_name = class.rsplit("::").next().unwrap_or(class);
    method == class_name || method == "GetDescriptor" || method.starts_with('~')
}

fn is_proxy_class(class: &str) -> bool {
    class.ends_with("Proxy") || class.ends_with("Client")
}

/// Split a qualified C++ function name into `(class, method)`.
/// Returns `None` for plain/non-member functions and destructors.
fn split_qualified(name: &str) -> Option<(String, String)> {
    let mut parts: Vec<&str> = name.split("::").collect();
    if parts.len() < 2 {
        return None;
    }
    let method = parts.pop().unwrap().to_string();
    if method.starts_with('~') {
        return None;
    }
    let class = parts.join("::");
    Some((class, method))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use trace_ir::{Function, Linkage, Span, TypeId};

    fn add_external_method(program: &mut Program, file: trace_ir::FileId, name: &str) -> FnId {
        let id = program.symbols.alloc_fn_id();
        program.symbols.push_synthetic_function(Function {
            id,
            name: name.to_string(),
            linkage: Linkage::External,
            return_type: TypeId(0),
            params: Vec::new(),
            locals: Vec::new(),
            span: Span::new(file, 1, 1),
            end_line: 1,
            file,
            is_defined: false,
            param_type_ids: Vec::new(),
            is_virtual: true,
            is_final: false,
            is_cpp: true,
        });
        id
    }

    #[test]
    fn interface_fallback_retains_all_matching_overloads() {
        let mut program = Program::new(PathBuf::from("/fixture"));
        let file = program
            .symbols
            .add_file(PathBuf::from("/fixture/interface.cpp"));
        let first = add_external_method(&mut program, file, "svc::IFoo::Run");
        let second = add_external_method(&mut program, file, "svc::IFoo::Run");
        add_external_method(&mut program, file, "other::IFoo::Run");

        let handlers = find_interface_methods(&program, "svc::FooStub", "Run");

        assert_eq!(handlers, vec![first, second]);
    }
}
