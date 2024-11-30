use crate::field_info::FieldInfo;

pub trait Indexable {
    fn get_field_configs(self) -> Vec<FieldInfo>;
}