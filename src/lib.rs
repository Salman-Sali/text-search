pub mod error;
mod index_reader;
mod index_writer;
pub mod paginated_result;

pub use index_reader::IndexReader;
pub use index_writer::IndexWriter;
pub use tantivy;
pub use text_search_core::FieldInfo;
pub use text_search_core::IndexField;
pub use text_search_core::IndexFieldSingle;
pub use text_search_core::IndexFieldVec;
pub use text_search_core::IndexType;
pub use text_search_core::Indexable;
pub use text_search_core::StructInfo;
pub use text_search_derive::Indexed;
