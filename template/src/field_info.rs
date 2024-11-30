pub struct FieldInfo {
    pub field_name: String,
    pub field_type: FieldType,
    pub stored: Stored 
}

impl FieldInfo {
    pub fn new(field_name: String, field_type: FieldType, stored: Stored) -> Self {
        Self { field_name, field_type, stored }
    }
}

pub enum FieldType {
    String,
    Text,
    NotIndexed
}

pub enum Stored {
    Yes,
    No
}