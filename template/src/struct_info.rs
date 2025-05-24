use tantivy::schema::Schema;

use crate::FieldInfo;

pub struct StructInfo {
    pub struct_name: String,
    pub fields: Vec<FieldInfo>,
}

impl StructInfo {
    pub fn new(name: String) -> Self {
        Self {
            struct_name: name,
            fields: vec![],
        }
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

    pub fn get_id_field(&self) -> &FieldInfo {
        return self.fields.iter().find(|x| x.is_id).expect("Missing id field.");
    }
}
