use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use trace_ir::{
    CallSite, CallSiteId, FlowConstraint, FnId, Function, Program, ReturnFlow, TypeDesc, TypeId,
    VarId, Variable,
};

/// Per-file indexing result merged into a single [`Program`].
#[derive(Debug, Clone, Default)]
pub struct UnitIndex {
    pub path: PathBuf,
    /// Unit-local file table: index == unit-local [`trace_ir::FileId`] value.
    /// Includes the TU itself plus every `#include`d origin that produced
    /// attributed entities.
    pub files: Vec<PathBuf>,
    pub types: trace_ir::TypeTable,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
    pub call_sites: Vec<CallSite>,
    pub flow: Vec<FlowConstraint>,
    pub fn_returns: std::collections::HashMap<FnId, Vec<ReturnFlow>>,
    pub diagnostics: Vec<trace_ir::Diagnostic>,
    pub anon_type_counter: u32,
    /// Per-unit `(derived, base)` class edges (C++).
    pub inheritance: Vec<(String, String)>,
    /// Classes declared `final` in this unit.
    pub final_classes: Vec<String>,
}

impl UnitIndex {
    /// Types, aliases, and inheritance only — used when a header is lowered
    /// after its includes so nested structs/typedefs are visible, without
    /// copying the child's functions/flow into every ancestor unit.
    pub fn clone_types_only(&self) -> UnitIndex {
        UnitIndex {
            path: self.path.clone(),
            files: self.files.clone(),
            types: self.types.clone(),
            inheritance: self.inheritance.clone(),
            final_classes: self.final_classes.clone(),
            anon_type_counter: self.anon_type_counter,
            ..Default::default()
        }
    }

    /// Symbols without executable IR. Header bodies are already in the
    /// global program from PCH; TUs only need types and prototypes so
    /// lowering can resolve names and field layouts.
    pub fn clone_symbols_only(&self) -> UnitIndex {
        UnitIndex {
            path: self.path.clone(),
            files: self.files.clone(),
            types: self.types.clone(),
            functions: self.functions.clone(),
            variables: self.variables.clone(),
            inheritance: self.inheritance.clone(),
            final_classes: self.final_classes.clone(),
            anon_type_counter: self.anon_type_counter,
            diagnostics: Vec::new(),
            ..Default::default()
        }
    }
}

type SiteKey = (trace_ir::FileId, u32, u32, String);

pub fn merge_unit_index(program: &mut Program, unit: UnitIndex) {
    program.diagnostics.extend(unit.diagnostics);
    program.anon_type_counter = program.anon_type_counter.max(unit.anon_type_counter);
    for (derived, base) in unit.inheritance {
        program.add_inheritance(&derived, &base);
    }
    for cls in unit.final_classes {
        program.mark_class_final(&cls);
    }

    let type_map = merge_types(&mut program.types, &unit.types);

    // Intern the unit's file table by path so headers reached through many
    // TUs collapse to one FileId and spans keep their original attribution.
    let mut file_map: Vec<trace_ir::FileId> = Vec::with_capacity(unit.files.len());
    for path in &unit.files {
        file_map.push(program.symbols.add_file_interned(path.clone()));
    }
    let primary_file_id = program.symbols.add_file_interned(unit.path.clone());
    for &mapped in &file_map {
        if mapped != primary_file_id {
            program
                .symbols
                .register_included_header(primary_file_id, mapped);
        }
    }
    let map_file = |id: trace_ir::FileId| -> trace_ir::FileId {
        file_map
            .get(id.0 as usize)
            .copied()
            .unwrap_or(primary_file_id)
    };
    let mut fn_map: HashMap<FnId, FnId> = HashMap::new();
    let mut dropped_fns: HashSet<FnId> = HashSet::new();
    // Functions whose params/locals still carry unit-local VarIds and therefore
    // need remapping once `var_map` is complete:
    // - freshly inserted rows (their own params/locals),
    // - canonical rows whose params add_function just backfilled with this
    //   unit's definition params.
    // Redirect targets that were NOT modified must stay untouched: remapping
    // them against this unit's var_map would silently drop their existing
    // global-id params (observed as missing arg-flow for every callee that a
    // later TU re-declares).
    let mut remap_params: HashSet<FnId> = HashSet::new();
    let mut remap_locals: HashSet<FnId> = HashSet::new();
    for func in unit.functions {
        let old_id = func.id;
        let span_file = map_file(func.span.file);
        let key = (span_file, func.name.clone(), func.span.line);
        if let Some(&canonical) = program.dedup.fn_keys.get(&key) {
            // Same origin entity lowered by another TU (or re-included):
            // keep the first copy, redirect all references to it.
            fn_map.insert(old_id, canonical);
            dropped_fns.insert(old_id);
            continue;
        }
        let new_id = program.symbols.alloc_fn_id();
        let mut f = func;
        f.id = new_id;
        f.span.file = span_file;
        f.file = span_file;
        f.return_type = remap_type(f.return_type, &type_map);
        let carries_params = !f.params.is_empty();
        let is_definition = f.is_defined;
        // Will add_function backfill these unit-local params onto an existing
        // canonical entry? True when the incoming row defines the body, or
        // when it carries params and the canonical has none so far.
        let backfills_params = carries_params
            && (is_definition
                || program
                    .symbols
                    .fn_by_name
                    .get(&f.name)
                    .and_then(|&eid| program.symbols.function_index(eid))
                    .map(|idx| program.symbols.functions[idx].params.is_empty())
                    .unwrap_or(false));
        let merged = program.symbols.add_function(f);
        if merged == new_id {
            remap_params.insert(merged);
            remap_locals.insert(merged);
        } else if backfills_params {
            // add_function copied these unit-local params onto the canonical
            // entry; remap them below.
            remap_params.insert(merged);
        }
        fn_map.insert(old_id, merged);
        program.dedup.fn_keys.insert(key, merged);
    }

    let mut var_map: HashMap<VarId, VarId> = HashMap::new();
    for var in unit.variables {
        // Locals/temps of dropped duplicate bodies are redundant.
        if var
            .fn_id
            .map(|id| dropped_fns.contains(&id))
            .unwrap_or(false)
        {
            continue;
        }
        let new_id = program.symbols.alloc_var_id();
        let mut v = var;
        let old = v.id;
        v.id = new_id;
        v.type_id = remap_type(v.type_id, &type_map);
        v.fn_id = v.fn_id.and_then(|id| fn_map.get(&id).copied());
        v.span.file = map_file(v.span.file);
        program.symbols.add_variable(v);
        var_map.insert(old, new_id);
    }

    for merged_id in &remap_locals {
        if let Some(idx) = program.symbols.function_index(*merged_id) {
            let func = &mut program.symbols.functions[idx];
            func.locals = func
                .locals
                .iter()
                .filter_map(|v| var_map.get(v).copied())
                .collect();
        }
    }
    for merged_id in &remap_params {
        if let Some(idx) = program.symbols.function_index(*merged_id) {
            let func = &mut program.symbols.functions[idx];
            func.params = func
                .params
                .iter()
                .filter_map(|v| var_map.get(v).copied())
                .collect();
        }
    }

    let mut call_map: HashMap<CallSiteId, CallSiteId> = HashMap::new();
    for cs in unit.call_sites {
        // A duplicate header body's own call sites are re-observed copies;
        // after caller redirection they collapse onto the canonical ones.
        if dropped_fns.contains(&cs.caller) {
            continue;
        }
        let span_file = map_file(cs.span.file);
        let key: SiteKey = (span_file, cs.span.line, cs.span.col, cs.callee_name.clone());
        if let Some(&existing) = program.dedup.site_keys.get(&key) {
            call_map.insert(cs.id, existing);
            continue;
        }
        let new_id = program.symbols.alloc_call_id();
        let old = cs.id;
        let mut site = cs;
        site.id = new_id;
        site.caller = fn_map.get(&site.caller).copied().unwrap_or(site.caller);
        // Pre-resolved targets (overload picks, member-call override sets)
        // must follow the entity remap like every other reference.
        site.callee_fn_id = site.callee_fn_id.and_then(|f| fn_map.get(&f).copied());
        site.callee_var = site.callee_var.and_then(|v| var_map.get(&v).copied());
        site.var_args = site
            .var_args
            .into_iter()
            .filter_map(|(i, v)| var_map.get(&v).map(|nv| (i, *nv)))
            .collect();
        site.fn_args = site
            .fn_args
            .into_iter()
            .filter_map(|(i, f)| fn_map.get(&f).map(|nf| (i, *nf)))
            .collect();
        site.return_dst = site.return_dst.and_then(|v| var_map.get(&v).copied());
        site.span.file = span_file;
        program.symbols.call_sites.push(site);
        program.dedup.site_keys.insert(key, new_id);
        call_map.insert(old, new_id);
    }

    for flow in unit.flow {
        if !flow_vars(&flow).all(|v| var_map.contains_key(&v)) {
            continue;
        }
        program.flow.push(remap_flow(flow, &fn_map, &var_map));
    }

    for (old_fn, flows) in unit.fn_returns {
        if dropped_fns.contains(&old_fn) {
            continue;
        }
        let Some(&new_fn) = fn_map.get(&old_fn) else {
            continue;
        };
        let remapped: Vec<ReturnFlow> = flows
            .into_iter()
            .filter(|f| return_flow_vars(f).all(|v| var_map.contains_key(&v)))
            .map(|f| remap_return_flow(f, &fn_map, &var_map))
            .collect();
        program
            .fn_returns
            .entry(new_fn)
            .or_default()
            .extend(remapped);
    }
}

fn flow_vars(flow: &FlowConstraint) -> impl Iterator<Item = VarId> + '_ {
    match flow {
        FlowConstraint::Copy { dst, src }
        | FlowConstraint::Load { dst, src }
        | FlowConstraint::Store { dst, src } => vec![*dst, *src],
        FlowConstraint::AddrOfVar { dst, src } => vec![*dst, *src],
        FlowConstraint::AddrOfFn { dst, .. } => vec![*dst],
        FlowConstraint::GepField { dst, base, .. } => vec![*dst, *base],
        FlowConstraint::ArrayFnMember { array, .. } => vec![*array],
        FlowConstraint::CallReturn { dst, .. } => vec![*dst],
        FlowConstraint::CallReturnIndirect { dst, callee_var } => vec![*dst, *callee_var],
        FlowConstraint::NewHeap { dst, .. } => vec![*dst],
        FlowConstraint::StringConst { dst, .. } => vec![*dst],
    }
    .into_iter()
}

fn return_flow_vars(flow: &ReturnFlow) -> impl Iterator<Item = VarId> + '_ {
    match flow {
        ReturnFlow::AddrOfVar { src } => vec![*src],
        ReturnFlow::Copy { src } => vec![*src],
        ReturnFlow::AddrOfFn { .. } | ReturnFlow::Call { .. } => Vec::new(),
    }
    .into_iter()
}

fn remap_type(id: TypeId, map: &HashMap<TypeId, TypeId>) -> TypeId {
    map.get(&id).copied().unwrap_or(id)
}

fn merge_types(
    dst: &mut trace_ir::TypeTable,
    src: &trace_ir::TypeTable,
) -> HashMap<TypeId, TypeId> {
    let mut map = HashMap::new();
    for info in src.all() {
        let new_id = match &info.desc {
            TypeDesc::Struct { name, fields } if !fields.is_empty() => {
                dst.compute_struct_layout(name.clone(), fields_from_layout(src, info))
            }
            TypeDesc::Union { name, fields } if !fields.is_empty() => {
                dst.compute_union_layout(name.clone(), fields_from_layout(src, info))
            }
            other => dst.intern(other.clone()),
        };
        map.insert(info.id, new_id);
    }
    // Typedef aliases resolve by name; identical headers lowered in many TUs
    // produce identical descs, so first registration wins.
    for (alias, desc) in src.all_aliases() {
        if dst.resolve_alias(alias).is_none() {
            dst.register_alias(alias, desc.clone());
        }
    }
    map
}

/// Prefer layout field types over the interned `TypeDesc` field list: PCH
/// intern may have rewritten nested empty tags (`struct IDeviceIoService`)
/// in the layout while the desc still stores the incomplete tag.
fn fields_from_layout(
    src: &trace_ir::TypeTable,
    info: &trace_ir::TypeInfo,
) -> Vec<(String, TypeDesc)> {
    info.layout
        .fields
        .iter()
        .map(|(_, fl)| (fl.name.clone(), src.get(fl.type_id).desc.clone()))
        .collect()
}

fn remap_flow(
    flow: FlowConstraint,
    fn_map: &HashMap<FnId, FnId>,
    var_map: &HashMap<VarId, VarId>,
) -> FlowConstraint {
    let rv = |v: VarId| var_map.get(&v).copied().unwrap_or(v);
    let rf = |f: FnId| fn_map.get(&f).copied().unwrap_or(f);
    match flow {
        FlowConstraint::Copy { dst, src } => FlowConstraint::Copy {
            dst: rv(dst),
            src: rv(src),
        },
        FlowConstraint::AddrOfVar { dst, src } => FlowConstraint::AddrOfVar {
            dst: rv(dst),
            src: rv(src),
        },
        FlowConstraint::AddrOfFn { dst, callee } => FlowConstraint::AddrOfFn {
            dst: rv(dst),
            callee: rf(callee),
        },
        FlowConstraint::Load { dst, src } => FlowConstraint::Load {
            dst: rv(dst),
            src: rv(src),
        },
        FlowConstraint::Store { dst, src } => FlowConstraint::Store {
            dst: rv(dst),
            src: rv(src),
        },
        FlowConstraint::GepField {
            dst,
            base,
            field,
            field_name,
        } => FlowConstraint::GepField {
            dst: rv(dst),
            base: rv(base),
            field,
            field_name: field_name.clone(),
        },
        FlowConstraint::ArrayFnMember { array, callee } => FlowConstraint::ArrayFnMember {
            array: rv(array),
            callee: rf(callee),
        },
        FlowConstraint::CallReturn { dst, callee_name } => FlowConstraint::CallReturn {
            dst: rv(dst),
            callee_name,
        },
        FlowConstraint::CallReturnIndirect { dst, callee_var } => {
            FlowConstraint::CallReturnIndirect {
                dst: rv(dst),
                callee_var: rv(callee_var),
            }
        }
        FlowConstraint::NewHeap { dst } => FlowConstraint::NewHeap { dst: rv(dst) },
        FlowConstraint::StringConst { dst, value } => FlowConstraint::StringConst {
            dst: rv(dst),
            value,
        },
    }
}

fn remap_return_flow(
    flow: ReturnFlow,
    fn_map: &HashMap<FnId, FnId>,
    var_map: &HashMap<VarId, VarId>,
) -> ReturnFlow {
    let rv = |v: VarId| var_map.get(&v).copied().unwrap_or(v);
    let rf = |f: FnId| fn_map.get(&f).copied().unwrap_or(f);
    match flow {
        ReturnFlow::AddrOfVar { src } => ReturnFlow::AddrOfVar { src: rv(src) },
        ReturnFlow::AddrOfFn { callee } => ReturnFlow::AddrOfFn { callee: rf(callee) },
        ReturnFlow::Copy { src } => ReturnFlow::Copy { src: rv(src) },
        ReturnFlow::Call { callee_name } => ReturnFlow::Call { callee_name },
    }
}
