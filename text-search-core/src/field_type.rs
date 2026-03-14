pub enum FieldType {
    String,
    I32,
    VecString,
    Unhandled,
}

impl FieldType {
    pub fn get_field_type(_type: &str) -> Self {
        match _type {
            "i32" => FieldType::I32,
            "String" => FieldType::String,
            "Vec" => FieldType::VecString,
            _ => {
                FieldType::Unhandled
                //panic!("{}", _type);
            }
        }
    }
}
