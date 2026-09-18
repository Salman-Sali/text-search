use crate::FieldInfo;

pub struct StructInfo {
    pub struct_name: String,
    pub fields: Vec<FieldInfo>,
}

impl StructInfo {
    pub fn new(name: String) -> Self {
        Self {
            struct_name: name,
            fields: Vec::new(),
        }
    }
}
