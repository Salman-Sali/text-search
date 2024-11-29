#[derive(Clone, Debug)]
pub enum IndexType {
    Text,
    String,
    None,
}

/// Enum for field storage type
#[derive(Clone, Debug)]
pub enum StorageType {
    Stored,
    NotStored,
}

/// Struct to hold field metadata
#[derive(Clone, Debug)]
pub struct FieldMeta {
    pub index_type: IndexType,
    pub storage_type: StorageType,
}