pub enum FieldType {
    String,
    I32,
    Unhandled
}

impl FieldType {
    pub fn get_field_type(_type: &str)-> Self {
        match _type {
            "i32" => FieldType::I32,
            "String" => FieldType::String,
            _ => FieldType::Unhandled
        }
    }
}