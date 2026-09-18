use crate::index_type::IndexType;

#[derive(Debug)]
pub struct FieldInfo {
    pub is_id: bool,
    pub field_name: String,
    pub index_type: IndexType,
    pub stored: bool,
}

impl FieldInfo {
    pub fn new(field_name: String, index_type: Option<IndexType>, stored: bool) -> Self {
        Self {
            is_id: false,
            field_name,
            index_type: index_type.unwrap_or(IndexType::not_indexed),
            stored,
        }
    }

    pub fn new_id_field(field_name: String) -> Self {
        Self {
            is_id: true,
            field_name,
            index_type: IndexType::indexed,
            stored: true,
        }
    }
}
