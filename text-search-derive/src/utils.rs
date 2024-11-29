#[derive(Clone, Debug)]
pub struct FieldMeta {
    pub indexed_as: Option<String>, // E.g., "text", "string", or "none"
}