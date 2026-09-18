pub enum FieldType {
    String,
    I32,
    I64,
    U32,
    U64,
    F64,
    Bool,
    Date,
    VecString,
    Unhandled,
}

impl FieldType {
    pub fn get_field_type(_type: &str) -> Self {
        match _type {
            "i32" => FieldType::I32,
            "i64" => FieldType::I64,
            "u32" => FieldType::U32,
            "u64" => FieldType::U64,
            "f64" => FieldType::F64,
            "bool" => FieldType::Bool,
            "DateTime" => FieldType::Date,
            "String" => FieldType::String,
            "Vec" => FieldType::VecString,
            _ => {
                FieldType::Unhandled
                //panic!("{}", _type);
            }
        }
    }
}
