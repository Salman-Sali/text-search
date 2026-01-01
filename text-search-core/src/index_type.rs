#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum IndexType {
    /// String Indexed but untokenized
    indexed_string,
    /// String Indexed but tokenized
    indexed_text,
    /// Number Indexed
    indexed,
    not_indexed,
}