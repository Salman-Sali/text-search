use crate::StructInfo;
use tantivy::{TantivyDocument, schema::Schema};

pub trait Indexable: Clone {
    fn as_document(&self) -> TantivyDocument;
    fn from_doc(doc: tantivy::TantivyDocument) -> Self;
    fn get_id_term(&self) -> tantivy::Term;
    fn generate_schema() -> Schema;
    fn get_struct_info() -> StructInfo;
}
