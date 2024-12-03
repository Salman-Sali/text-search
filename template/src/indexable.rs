use crate::struct_info::StructInfo;

pub trait Indexable {
    fn get_struct_info(self) -> StructInfo;
}       