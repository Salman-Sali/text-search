#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum IndexType {
    indexed_string,
    indexed_text,
    indexed,
    not_indexed,
}