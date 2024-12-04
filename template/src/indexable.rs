use tantivy::TantivyDocument;

use crate::struct_info::StructInfo;

pub trait Indexable {
    fn get_struct_info() -> StructInfo;
    fn as_document(&self) -> TantivyDocument;
    fn from_doc(doc: tantivy::TantivyDocument) -> Self;
}