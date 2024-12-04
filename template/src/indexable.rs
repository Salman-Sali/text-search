use tantivy::TantivyDocument;

use crate::struct_info::StructInfo;

pub trait Indexable {
    fn get_struct_info(&self) -> StructInfo;
    fn as_document(&self) -> TantivyDocument;
    //fn add_doc(self);
    // fn delete_doc(self);
    // fn search_doc();
}