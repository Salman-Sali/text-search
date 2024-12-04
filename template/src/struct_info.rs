use tantivy::schema::Schema;

use crate::FieldInfo;

pub struct StructInfo {
    pub fields: Vec<FieldInfo>,
}

impl StructInfo {
    pub fn new() -> Self {
        Self { fields: vec![] }
    }

    pub fn add_field(&mut self, field: FieldInfo) {
        self.fields.push(field);
    }

    pub fn generate_schema(&self) -> Schema {
        let mut schema_builder = Schema::builder();
        for field in self.fields.iter() {
            field.add_to_schema(&mut schema_builder);
        }
        schema_builder.build()
    }
}
