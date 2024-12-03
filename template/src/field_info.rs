use std::fmt::Display;

use crate::symbol::*;

pub struct FieldInfo {
    pub is_id: bool,
    pub field_name: String,
    pub field_type: FieldType,
    pub stored: Stored,
}

impl FieldInfo {
    pub fn new(field_name: String, field_type: Option<FieldType>, _stored: Option<Stored>) -> Self {
        Self {
            is_id: false,
            field_name,
            field_type: field_type.unwrap_or(FieldType::not_indexed),
            stored: _stored.unwrap_or(Stored::stored),
        }
    }

    pub fn new_id_field(field_name: String) -> Self {
        Self {
            is_id: false,
            field_name,
            field_type: FieldType::not_indexed,
            stored: Stored::stored,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FieldType {
    indexed_string = 0,
    indexed_text = 1,
    not_indexed = 2,
}

#[derive(Debug, Clone)]
pub enum Stored {
    stored,
    not_stored,
}