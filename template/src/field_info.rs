
use tantivy::schema::{NumericOptions, SchemaBuilder, TextOptions, INDEXED as TavtivyINDEXED, STRING, TEXT};

use crate::{index_type::IndexType, FieldType};

pub struct FieldInfo {
    pub is_id: bool,
    pub field_type: FieldType,
    pub field_name: String,
    pub index_type: IndexType,
    pub stored: bool,
}

impl FieldInfo {
    pub fn new(field_name: String, field_type: FieldType, index_type: Option<IndexType>, stored: bool) -> Self {
        Self {
            is_id: false,
            field_type,
            field_name,
            index_type: index_type.unwrap_or(IndexType::not_indexed),
            stored,
        }
    }

    pub fn new_id_field(field_name: String, field_type: FieldType) -> Self {
        Self {
            is_id: true,
            field_type,
            field_name,
            index_type: IndexType::indexed,
            stored: true
        }
    }

    pub fn add_to_schema(&self, schema_builder: &mut SchemaBuilder) {
        match self.field_type {
            FieldType::String => {
                let mut text_options:TextOptions = match self.index_type {
                    IndexType::indexed_string => STRING,
                    IndexType::indexed_text => TEXT,
                    IndexType::indexed => STRING,
                    IndexType::not_indexed => Default::default(),
                };                
                
                if self.stored {
                    text_options = text_options.set_stored();
                }

                schema_builder.add_text_field(&self.field_name, text_options);
            },
            FieldType::I32 => {
                let mut numeric_options: NumericOptions = match self.index_type {
                    IndexType::indexed_string => panic!("`indexed_string` is not supported on numeric fields."),
                    IndexType::indexed_text => panic!("`indexed_text` is not supported on numeric fields."),
                    IndexType::indexed => TavtivyINDEXED.into(),
                    IndexType::not_indexed => Default::default(),
                };

                if self.stored {
                    numeric_options = numeric_options.set_stored();
                }

                schema_builder.add_i64_field(&self.field_name, numeric_options);
            },
            FieldType::Unhandled => panic!("Unhandled field type")
        }
    }
}

