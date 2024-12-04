#[derive(Debug, Clone)]
pub enum IndexType {
    indexed_string,
    indexed_text,
    indexed,
    not_indexed,
}