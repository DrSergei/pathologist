use crate::{FieldId, TypeId};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeDesc {
    Void,
    Char,
    Int,
    Long,
    SizeT,
    Unknown,
    Ptr(Box<TypeDesc>),
    Array {
        elem: Box<TypeDesc>,
        size: Option<u64>,
    },
    Struct {
        name: String,
        fields: Vec<(String, TypeDesc)>,
    },
    Union {
        name: String,
        fields: Vec<(String, TypeDesc)>,
    },
    FnPtr {
        ret: Box<TypeDesc>,
        params: Vec<TypeDesc>,
    },
}

impl TypeDesc {
    pub fn is_pointer_like(&self) -> bool {
        matches!(self, TypeDesc::Ptr(_) | TypeDesc::FnPtr { .. })
    }

    pub fn pointee(&self) -> Option<&TypeDesc> {
        match self {
            TypeDesc::Ptr(inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeKind {
    Void,
    Char,
    Int,
    Long,
    SizeT,
    Unknown,
    Ptr,
    Array,
    Struct,
    Union,
    FnPtr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    pub id: TypeId,
    pub desc: TypeDesc,
    pub size: u64,
    pub align: u64,
    pub layout: TypeLayout,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeLayout {
    pub fields: IndexMap<FieldId, FieldLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldLayout {
    pub name: String,
    pub offset: u64,
    pub size: u64,
    pub type_id: TypeId,
}

#[derive(Debug, Clone)]
pub struct TypeTable {
    types: Vec<TypeInfo>,
    intern: IndexMap<TypeDesc, TypeId>,
    /// Typedef alias name → resolved descriptor. Alias resolution is needed
    /// because lowering sees bare identifiers (`fn_t`, `SHandle`) whose
    /// pointer-ness is otherwise lost (they degrade to `Int`).
    aliases: IndexMap<String, TypeDesc>,
}

impl Default for TypeTable {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeTable {
    pub fn new() -> Self {
        let mut table = Self {
            types: Vec::new(),
            intern: IndexMap::new(),
            aliases: IndexMap::new(),
        };
        table.intern(TypeDesc::Void);
        table.intern(TypeDesc::Char);
        table.intern(TypeDesc::Int);
        table.intern(TypeDesc::Long);
        table.intern(TypeDesc::SizeT);
        table.intern(TypeDesc::Unknown);
        table
    }

    pub fn intern(&mut self, desc: TypeDesc) -> TypeId {
        let desc = self.canonicalize_desc(desc);
        if let Some(id) = self.lookup_tag_ref(&desc) {
            return id;
        }
        if let Some(id) = self.intern.get(&desc) {
            return *id;
        }
        let (size, align, layout) = compute_layout(&desc, self);
        let id = TypeId(self.types.len() as u32);
        self.types.push(TypeInfo {
            id,
            desc: desc.clone(),
            size,
            align,
            layout,
        });
        self.intern.insert(desc, id);
        id
    }

    /// Rewrite empty `struct Foo` / `union Foo` tags (and pointers to them)
    /// to the most complete interned layout of that tag, when one exists.
    fn canonicalize_desc(&self, desc: TypeDesc) -> TypeDesc {
        match desc {
            TypeDesc::Ptr(inner) => TypeDesc::Ptr(Box::new(self.canonicalize_desc(*inner))),
            TypeDesc::Array { elem, size } => TypeDesc::Array {
                elem: Box::new(self.canonicalize_desc(*elem)),
                size,
            },
            TypeDesc::Struct { name, fields } if fields.is_empty() && !name.is_empty() => {
                if let Some(id) = self.type_id_by_tag(&name, TypeKind::Struct) {
                    return self.get(id).desc.clone();
                }
                TypeDesc::Struct { name, fields }
            }
            TypeDesc::Union { name, fields } if fields.is_empty() && !name.is_empty() => {
                if let Some(id) = self.type_id_by_tag(&name, TypeKind::Union) {
                    return self.get(id).desc.clone();
                }
                TypeDesc::Union { name, fields }
            }
            other => other,
        }
    }

    /// After merging another unit, point nested incomplete tag fields at the
    /// most complete layout interned so far (PCH headers are parsed in
    /// isolation; a later header may supply `IDeviceIoService` after
    /// `StreamHost { struct IDeviceIoService service; }` was interned).
    pub fn complete_nested_tags(&mut self) {
        let n = self.types.len();
        for i in 0..n {
            let fids: Vec<FieldId> = self.types[i].layout.fields.keys().copied().collect();
            for fid in fids {
                let old = self.types[i].layout.fields[&fid].type_id;
                let new_id = self.complete_type_id(old);
                if new_id != old {
                    if let Some(fl) = self.types[i].layout.fields.get_mut(&fid) {
                        fl.type_id = new_id;
                    }
                }
            }
        }
    }

    fn complete_type_id(&mut self, id: TypeId) -> TypeId {
        match self.get(id).desc.clone() {
            TypeDesc::Struct { name, fields } if fields.is_empty() && !name.is_empty() => self
                .type_id_by_tag(&name, TypeKind::Struct)
                .unwrap_or(id),
            TypeDesc::Union { name, fields } if fields.is_empty() && !name.is_empty() => {
                self.type_id_by_tag(&name, TypeKind::Union).unwrap_or(id)
            }
            TypeDesc::Ptr(inner) => {
                let completed = self.canonicalize_desc(*inner);
                let richer = match &completed {
                    TypeDesc::Struct { fields, .. } | TypeDesc::Union { fields, .. } => {
                        !fields.is_empty()
                    }
                    _ => false,
                };
                if richer {
                    self.intern(TypeDesc::Ptr(Box::new(completed)))
                } else {
                    id
                }
            }
            _ => id,
        }
    }

    pub fn get(&self, id: TypeId) -> &TypeInfo {
        &self.types[id.0 as usize]
    }

    pub fn void(&self) -> TypeId {
        TypeId(0)
    }

    pub fn int(&self) -> TypeId {
        TypeId(2)
    }

    pub fn ptr_to(&mut self, inner: TypeDesc) -> TypeId {
        self.intern(TypeDesc::Ptr(Box::new(inner)))
    }

    /// Record a typedef alias (`typedef void (*fn_t)(int);` → `fn_t`) so that
    /// later declarations using the bare alias keep their pointer-ness.
    pub fn register_alias(&mut self, alias: &str, desc: TypeDesc) {
        if !alias.is_empty() {
            self.aliases.insert(alias.to_string(), desc);
        }
    }

    pub fn resolve_alias(&self, alias: &str) -> Option<&TypeDesc> {
        self.aliases.get(alias)
    }

    pub fn all_aliases(&self) -> &IndexMap<String, TypeDesc> {
        &self.aliases
    }

    pub fn all(&self) -> &[TypeInfo] {
        &self.types
    }

    pub fn compute_struct_layout(
        &mut self,
        name: String,
        fields: Vec<(String, TypeDesc)>,
    ) -> TypeId {
        self.intern(TypeDesc::Struct { name, fields })
    }

    pub fn compute_union_layout(
        &mut self,
        name: String,
        fields: Vec<(String, TypeDesc)>,
    ) -> TypeId {
        self.intern(TypeDesc::Union { name, fields })
    }

    pub fn field_id_by_name(&self, type_id: TypeId, fname: &str) -> Option<FieldId> {
        let info = self.get(type_id);
        info.layout
            .fields
            .iter()
            .find(|(_, fl)| fl.name == fname)
            .map(|(id, _)| *id)
    }

    fn lookup_tag_ref(&self, desc: &TypeDesc) -> Option<TypeId> {
        match desc {
            TypeDesc::Struct { name, fields } if fields.is_empty() && !name.is_empty() => {
                self.type_id_by_tag(name, TypeKind::Struct)
            }
            TypeDesc::Union { name, fields } if fields.is_empty() && !name.is_empty() => {
                self.type_id_by_tag(name, TypeKind::Union)
            }
            _ => None,
        }
    }

    pub fn type_id_by_tag(&self, name: &str, kind: TypeKind) -> Option<TypeId> {
        self.types
            .iter()
            .filter(|t| tag_name_matches(&t.desc, name, kind))
            .max_by_key(|t| {
                let layout_n = t.layout.fields.len();
                let desc_n = match &t.desc {
                    TypeDesc::Struct { fields, .. } | TypeDesc::Union { fields, .. } => {
                        fields.len()
                    }
                    _ => 0,
                };
                layout_n.max(desc_n)
            })
            .map(|t| t.id)
    }

    pub fn resolve_type_id(&self, desc: &TypeDesc) -> TypeId {
        if let Some(id) = self.lookup_tag_ref(desc) {
            return id;
        }
        if let Some(id) = self.intern.get(desc) {
            return *id;
        }
        if let TypeDesc::Ptr(inner) = desc {
            let pointee = self.lookup_tag_ref(inner).unwrap_or_else(|| {
                self.intern
                    .get(inner.as_ref())
                    .copied()
                    .unwrap_or(TypeId(5))
            });
            if pointee != TypeId(5) {
                let pointee_desc = self.get(pointee).desc.clone();
                let ptr_desc = TypeDesc::Ptr(Box::new(pointee_desc));
                if let Some(id) = self.intern.get(&ptr_desc) {
                    return *id;
                }
            }
        }
        TypeId(5)
    }
}

fn tag_name_matches(desc: &TypeDesc, name: &str, kind: TypeKind) -> bool {
    match (desc, kind) {
        (TypeDesc::Struct { name: n, .. }, TypeKind::Struct) => n == name,
        (TypeDesc::Union { name: n, .. }, TypeKind::Union) => n == name,
        _ => false,
    }
}

fn compute_layout(desc: &TypeDesc, table: &mut TypeTable) -> (u64, u64, TypeLayout) {
    match desc {
        TypeDesc::Void => (0, 1, TypeLayout::default()),
        TypeDesc::Char => (1, 1, TypeLayout::default()),
        TypeDesc::Int => (4, 4, TypeLayout::default()),
        TypeDesc::Long => (8, 8, TypeLayout::default()),
        TypeDesc::SizeT => (8, 8, TypeLayout::default()),
        TypeDesc::Unknown => (8, 8, TypeLayout::default()),
        TypeDesc::Ptr(_) | TypeDesc::FnPtr { .. } => (8, 8, TypeLayout::default()),
        TypeDesc::Array { elem, size } => {
            let (elem_size, elem_align, _) = compute_layout(elem, table);
            let count = size.unwrap_or(0);
            (elem_size * count, elem_align, TypeLayout::default())
        }
        TypeDesc::Struct { fields, .. } | TypeDesc::Union { fields, .. } => {
            let mut layout = TypeLayout::default();
            let mut offset = 0u64;
            let mut max_align = 1u64;
            let mut total_size = 0u64;
            for (idx, (name, field_desc)) in fields.iter().enumerate() {
                let fid = FieldId(idx as u32);
                let field_type_id = table.intern(field_desc.clone());
                let (field_size, field_align, _) = compute_layout(field_desc, table);
                max_align = max_align.max(field_align);
                offset = align_up(offset, field_align);
                layout.fields.insert(
                    fid,
                    FieldLayout {
                        name: name.clone(),
                        offset,
                        size: field_size,
                        type_id: field_type_id,
                    },
                );
                offset += field_size;
                total_size = total_size.max(offset);
            }
            total_size = align_up(total_size, max_align);
            (total_size, max_align, layout)
        }
    }
}

fn align_up(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    value.div_ceil(align) * align
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_nested_tags_rewrites_empty_embedded_struct() {
        let mut t = TypeTable::new();
        let host = t.compute_struct_layout(
            "StreamHost".into(),
            vec![(
                "service".into(),
                TypeDesc::Struct {
                    name: "IDeviceIoService".into(),
                    fields: Vec::new(),
                },
            )],
        );
        t.compute_struct_layout(
            "IDeviceIoService".into(),
            vec![(
                "Dispatch".into(),
                TypeDesc::FnPtr {
                    ret: Box::new(TypeDesc::Int),
                    params: Vec::new(),
                },
            )],
        );
        t.complete_nested_tags();
        let service_tid = t.get(host).layout.fields.get(&FieldId(0)).unwrap().type_id;
        assert!(
            t.field_id_by_name(service_tid, "Dispatch").is_some(),
            "embedded IDeviceIoService must expose Dispatch after completion"
        );
    }
}
