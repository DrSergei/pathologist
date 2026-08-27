use crate::{CallSiteId, FileId, FnId, Span, TypeId, VarId};
use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Linkage {
    External,
    Internal,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StorageClass {
    Global,
    FileStatic,
    FnStatic,
    Param,
    Local,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub id: VarId,
    pub name: String,
    pub type_id: TypeId,
    pub storage: StorageClass,
    pub fn_id: Option<FnId>,
    pub param_index: Option<u32>,
    pub span: Span,
    pub is_pointer: bool,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: FnId,
    pub name: String,
    pub linkage: Linkage,
    pub return_type: TypeId,
    pub params: Vec<VarId>,
    pub locals: Vec<VarId>,
    /// Start position (original-file coordinates via LineMap).
    pub span: Span,
    /// Last line of the definition body in original-file coordinates.
    /// Equal to `span.line` for prototypes, synthesized externals, and
    /// bodies whose end could not be mapped back (e.g. cross-header ends).
    /// On merge, overwritten together with `file`/`span` so the range always
    /// describes the surviving definition row in its own file's coordinates.
    pub end_line: u32,
    pub file: FileId,
    pub is_defined: bool,
    /// Declared `virtual` (C++ methods). Virtual dispatch expansion treats a
    /// method as virtual if *any* entry with its qualified name carries this
    /// flag, so out-of-class definitions without the token still participate.
    pub is_virtual: bool,
    /// Declared `final` (C++ methods). CHA does not look for overrides in
    /// subclasses of a class that finalizes this method.
    pub is_final: bool,
    /// Entry may coexist with same-name externals of a different signature
    /// (C++ overloads). When neither side sets this, name merges behave
    /// exactly as in C (prototype + definition collapse into one entry).
    /// A C `.c` definition merging into a C++-parsed `.h` prototype clears
    /// this flag so a later TU merge does not treat the pair as overloads.
    pub is_cpp: bool,
}

#[derive(Debug, Clone)]
pub struct CallSite {
    pub id: crate::CallSiteId,
    pub caller: FnId,
    pub callee_name: String,
    pub callee_var: Option<VarId>,
    /// Callee fixed up after lowering: a definition/prototype resolved at
    /// lowering time, or a synthesized external entry for a plain-identifier
    /// call that no tree-local symbol declares (libc calls, macro-emitted
    /// logging backends). `None` for indirect sites.
    pub callee_fn_id: Option<FnId>,
    pub var_args: Vec<(u32, VarId)>,
    pub fn_args: Vec<(u32, FnId)>,
    /// Argument positions recorded as `&base.member` / `&arr[i]` addresses.
    /// Lowering resolves these to the *base* variable, so function-model
    /// alias effects must not treat them as whole-object copies (copying
    /// the containing object would pollute unrelated fields).
    pub addr_of_member_args: Vec<u32>,
    pub span: Span,
    pub is_direct: bool,
    /// Static class of a C++ member-call receiver (`this`, typed pointer).
    /// Post-merge virtual expansion uses this so `final` types are not
    /// re-expanded from the declaring base.
    pub receiver_class: Option<String>,
    /// LHS of `dst = callee(...)` when the call's value is used (`CallReturn`
    /// destination). `dlsym` models write function addresses here.
    pub return_dst: Option<VarId>,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub id: FileId,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub files: Vec<FileInfo>,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
    pub call_sites: Vec<CallSite>,
    pub fn_by_name: IndexMap<String, FnId>,
    /// Every external entry per name, overloads included (C++). Unlike
    /// `fn_by_name` this never collapses to a single id.
    pub externals_by_name: FxHashMap<String, Vec<FnId>>,
    pub global_by_name: IndexMap<String, VarId>,
    /// Internal-linkage definitions per file: `(file, name) -> FnId`.
    /// In C, a file-`static` definition shadows any external definition of
    /// the same name for references inside that file.
    fn_by_scope: FxHashMap<FileId, FxHashMap<String, FnId>>,
    /// Headers whose entities were attributed to this TU during lowering
    /// (`#include`d code). Scope resolution consults them so a `static`
    /// inline defined in a header stays visible to its includers after
    /// cross-TU deduplication collapsed the per-TU copies.
    headers_of: FxHashMap<FileId, std::collections::BTreeSet<FileId>>,
    file_by_path: FxHashMap<PathBuf, FileId>,
    /// `FnId -> slot in functions`. Ids are not dense (merged duplicates and
    /// superseded rows leave gaps), so lookups need this index to stay O(1).
    fn_slots: FxHashMap<FnId, u32>,
    next_fn: u32,
    next_var: u32,
    next_call: u32,
}

impl SymbolTable {
    pub fn add_file(&mut self, path: PathBuf) -> FileId {
        let id = FileId(self.files.len() as u32);
        self.files.push(FileInfo { id, path });
        id
    }

    /// Intern a file by path: repeated origins (the same header reached
    /// through many TUs) map to one [`FileId`].
    pub fn add_file_interned(&mut self, path: impl AsRef<Path>) -> FileId {
        let path = path.as_ref();
        if let Some(&id) = self.file_by_path.get(path) {
            return id;
        }
        let path = path.to_path_buf();
        let id = self.add_file(path.clone());
        self.file_by_path.insert(path, id);
        id
    }

    pub fn file_by_path(&self, path: &Path) -> Option<FileId> {
        self.file_by_path.get(path).copied()
    }

    /// Register that `header` contributes entities lowered while indexing
    /// `tu` (directly or transitively).
    pub fn register_included_header(&mut self, tu: crate::FileId, header: crate::FileId) {
        if tu != header {
            self.headers_of.entry(tu).or_default().insert(header);
        }
    }

    pub fn included_headers(
        &self,
        tu: crate::FileId,
    ) -> Option<&std::collections::BTreeSet<crate::FileId>> {
        self.headers_of.get(&tu)
    }

    pub fn add_function(&mut self, func: Function) -> FnId {
        if func.linkage == Linkage::External {
            if let Some(existing_id) = self.fn_by_name.get(&func.name).copied() {
                // Merge only compatible redeclarations (prototype + definition).
                // Distinct arities mean C++ overloads — and only then: keep
                // both entries so call-site resolution can pick between them.
                //
                // Overload splitting requires *both* sides to be C++. A `.h`
                // reached from a C++ TU is parsed as C++ (`is_cpp`), but the
                // `.c` definition is not. Treating that as an overload (the
                // old `||`) left callers bound to the undefined prototype —
                // HDF `GpioSetIrq` never reached `GpioRegListener`, so
                // `gpio->func` stayed empty. Mixed-language same-name entries
                // still require matching arity when both sides have params,
                // so a coincidental C++ overload is not swallowed.
                let existing_fn = self.functions.iter().find(|f| f.id == existing_id);
                let both_cpp = func.is_cpp && existing_fn.map(|e| e.is_cpp).unwrap_or(false);
                let mergeable = existing_fn
                    .map(|existing| {
                        if !func.is_cpp && !existing.is_cpp {
                            // Pure C: prototype + definition always collapse.
                            return true;
                        }
                        let arity_ok = existing.params.is_empty()
                            || func.params.is_empty()
                            || existing.params.len() == func.params.len();
                        if !both_cpp {
                            // Header parsed as C++ vs `.c` body: merge by
                            // arity and ignore param-type mismatch (typedef
                            // `GpioIrqFunc` vs decayed `Int`).
                            return arity_ok;
                        }
                        // C++: prototypes and definitions of the *same*
                        // function merge; distinct same-arity overloads
                        // must stay apart. Parameter types disambiguate.
                        arity_ok
                            && (existing.params.is_empty()
                                || func.params.is_empty()
                                || existing
                                    .params
                                    .iter()
                                    .zip(func.params.iter())
                                    .all(|(a, b)| self.param_type(*a) == self.param_type(*b)))
                    })
                    .unwrap_or(false);
                if mergeable {
                    if let Some(existing) = self.functions.iter_mut().find(|f| f.id == existing_id)
                    {
                        if func.is_defined {
                            existing.is_defined = true;
                            existing.file = func.file;
                            existing.span = func.span;
                            existing.end_line = func.end_line;
                            if !func.params.is_empty() {
                                existing.params = func.params.clone();
                            }
                        } else if existing.params.is_empty() && !func.params.is_empty() {
                            existing.params = func.params.clone();
                        }
                        if func.is_virtual {
                            existing.is_virtual = true;
                        }
                        if func.is_final {
                            existing.is_final = true;
                        }
                        // A C definition merging into a C++-parsed header
                        // prototype must drop `is_cpp`. Otherwise a later
                        // TU merge sees both_cpp and refuses the body
                        // (param TypeIds are still unit-local, so the
                        // overload type-check always fails).
                        existing.is_cpp = existing.is_cpp && func.is_cpp;
                    }
                    let bucket = self.externals_by_name.entry(func.name.clone()).or_default();
                    if !bucket.contains(&existing_id) {
                        bucket.push(existing_id);
                    }
                    return existing_id;
                }
            }
            self.fn_by_name.insert(func.name.clone(), func.id);
            self.externals_by_name
                .entry(func.name.clone())
                .or_default()
                .push(func.id);
        }
        if func.linkage == Linkage::Internal {
            // Merge forward declarations with definitions for internal
            // (static) functions. Without this, a forward declaration
            // lower bounds before its definition creates a separate entry;
            // call-sites resolved between the two point at the declaration
            // (is_defined=false) and the solver never expands its body.
            if let Some(scope_map) = self.fn_by_scope.get(&func.file) {
                if let Some(&existing_id) = scope_map.get(&func.name) {
                    if let Some(existing) = self.functions.iter_mut().find(|f| f.id == existing_id)
                    {
                        if func.is_defined && !existing.is_defined {
                            existing.is_defined = true;
                            existing.file = func.file;
                            existing.span = func.span;
                            existing.end_line = func.end_line;
                            if !func.params.is_empty() {
                                existing.params = func.params.clone();
                            }
                        } else if !func.is_defined
                            && existing.params.is_empty()
                            && !func.params.is_empty()
                        {
                            existing.params = func.params.clone();
                        }
                        if func.is_virtual {
                            existing.is_virtual = true;
                        }
                        if func.is_final {
                            existing.is_final = true;
                        }
                        return existing_id;
                    }
                }
            }
            // Index every internal-linkage entry, declarations included:
            // lowering resolves identifiers against this table *while the
            // file streams in*, so a designated initializer like
            // `.Read = StaticFn` must bind before the definition is lowered.
            self.fn_by_scope
                .entry(func.file)
                .or_default()
                .insert(func.name.clone(), func.id);
        }
        self.push_indexed(func)
    }

    /// Push a synthesized `Function` (e.g. extern-callee stubs) without
    /// registering it in the name/scope resolution maps, so it cannot shadow
    /// real definitions. Only the id index is maintained.
    pub fn push_synthetic_function(&mut self, func: Function) -> FnId {
        debug_assert!(
            !self.fn_by_name.contains_key(&func.name),
            "synthetic function must not shadow a registered name"
        );
        self.push_indexed(func)
    }

    fn push_indexed(&mut self, func: Function) -> FnId {
        let id = func.id;
        self.fn_slots.insert(id, self.functions.len() as u32);
        self.functions.push(func);
        id
    }

    /// Slot of `id` in [`SymbolTable::functions`], O(1).
    pub fn function_index(&self, id: FnId) -> Option<usize> {
        self.fn_slots.get(&id).map(|&s| s as usize)
    }

    /// Type of a parameter variable, for overload signature comparison.
    fn param_type(&self, var: VarId) -> Option<TypeId> {
        self.variables
            .iter()
            .find(|v| v.id == var)
            .map(|v| v.type_id)
    }

    pub fn add_variable(&mut self, var: Variable) -> VarId {
        let id = var.id;
        if var.storage == StorageClass::Global {
            self.global_by_name.insert(var.name.clone(), id);
        }
        self.variables.push(var);
        id
    }

    pub fn alloc_fn_id(&mut self) -> FnId {
        let id = FnId(self.next_fn);
        self.next_fn += 1;
        id
    }

    pub fn alloc_var_id(&mut self) -> VarId {
        let id = VarId(self.next_var);
        self.next_var += 1;
        id
    }

    pub fn alloc_call_id(&mut self) -> CallSiteId {
        let id = CallSiteId(self.next_call);
        self.next_call += 1;
        id
    }

    pub fn resolve_function(&self, name: &str) -> Option<FnId> {
        self.fn_by_name.get(name).copied()
    }

    /// Resolve by C scoping rules: an internal-linkage (`static`) definition
    /// in `file` shadows any external definition of the same name for
    /// references inside that file; otherwise fall back to the external name
    /// table. `#include`d headers contributing entities to `file` are part
    /// of its scope (TU-local wins over header-defined on name collision).
    pub fn resolve_function_in_scope(
        &self,
        name: &str,
        file: Option<crate::FileId>,
    ) -> Option<FnId> {
        if let Some(file) = file {
            if let Some(id) = self.lookup_in_scopes(name, file) {
                return Some(id);
            }
        }
        self.fn_by_name.get(name).copied()
    }

    fn lookup_in_scopes(&self, name: &str, file: crate::FileId) -> Option<FnId> {
        if let Some(scope) = self.fn_by_scope.get(&file) {
            if let Some(id) = scope.get(name) {
                return Some(*id);
            }
        }
        if let Some(headers) = self.headers_of.get(&file) {
            for h in headers {
                if let Some(scope) = self.fn_by_scope.get(h) {
                    if let Some(id) = scope.get(name) {
                        return Some(*id);
                    }
                }
            }
        }
        None
    }

    /// All functions a post-merge name lookup may refer to.
    ///
    /// Name-based facts (`CallReturn`, `ReturnFlow::Call`, recovered direct
    /// calls) lose the calling TU's visibility context at merge time, so a
    /// name that matches both a file-`static` definition and an external
    /// definition is genuinely ambiguous there. Per may-analysis semantics
    /// (over-approximate when uncertain) callers must consider every
    /// candidate. Paths that preserved callee ids through lowering + merge
    /// should use those ids directly instead — they are exact.
    pub fn resolve_function_candidates(
        &self,
        name: &str,
        file: Option<crate::FileId>,
    ) -> Vec<FnId> {
        let mut out = Vec::with_capacity(2);
        if let Some(file) = file {
            if let Some(scope) = self.fn_by_scope.get(&file) {
                if let Some(&id) = scope.get(name) {
                    out.push(id);
                }
            }
            if let Some(headers) = self.headers_of.get(&file) {
                for h in headers {
                    if let Some(scope) = self.fn_by_scope.get(h) {
                        if let Some(&id) = scope.get(name) {
                            if !out.contains(&id) {
                                out.push(id);
                            }
                        }
                    }
                }
            }
        }
        if let Some(&id) = self.fn_by_name.get(name) {
            if !out.contains(&id) {
                out.push(id);
            }
        }
        // C++ overloads: additional entries under the same name that the
        // first-wins `fn_by_name` table hides.
        if let Some(bucket) = self.externals_by_name.get(name) {
            for &id in bucket {
                if !out.contains(&id) {
                    out.push(id);
                }
            }
        }
        out
    }

    /// Every external entry declared or defined under `name` (overloads
    /// included), in declaration order.
    pub fn functions_named(&self, name: &str) -> Vec<FnId> {
        self.externals_by_name
            .get(name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn function_by_id(&self, id: FnId) -> Option<&Function> {
        let slot = *self.fn_slots.get(&id)?;
        self.functions.get(slot as usize).filter(|f| f.id == id)
    }

    pub fn function(&self, id: FnId) -> &Function {
        self.function_by_id(id)
            .unwrap_or_else(|| panic!("unknown function id {}", id.0))
    }

    pub fn variable_by_id(&self, id: VarId) -> Option<&Variable> {
        self.variables.get(id.0 as usize).filter(|v| v.id == id)
    }

    pub fn variable(&self, id: VarId) -> &Variable {
        self.variable_by_id(id)
            .unwrap_or_else(|| panic!("unknown variable id {}", id.0))
    }

    pub fn variable_mut(&mut self, id: VarId) -> &mut Variable {
        self.variables
            .get_mut(id.0 as usize)
            .filter(|v| v.id == id)
            .unwrap_or_else(|| panic!("unknown variable id {}", id.0))
    }

    pub fn call_site_by_id(&self, id: CallSiteId) -> Option<&CallSite> {
        self.call_sites.iter().find(|c| c.id == id)
    }

    pub fn function_ids_unique(&self) -> bool {
        let mut seen = std::collections::HashSet::new();
        self.functions.iter().all(|f| seen.insert(f.id))
    }
}
