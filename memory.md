# text-search Project Memory

## Project Overview
A Rust text search library built on top of Tantivy, providing derive macros for indexing and searching structs.

## Architecture
- **text-search**: Main library with `IndexWriter` and `IndexReader`
- **text-search-core**: Core traits (`Indexable`, `IndexField`) and types
- **text-search-derive**: Procedural macros for deriving `Indexable`

## Trait-Based Design

### IndexField Trait
```rust
pub trait IndexField: Sized {
    fn type_name() -> &'static str;
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType);
    fn add_to_document(&self, doc: &mut TantivyDocument, field: Field);
    fn from_owned_value(value: &OwnedValue) -> Option<Self>;
    fn to_term(&self, field: Field) -> Term;
}
```

### Helper Traits
- `IndexFieldSingle`: For non-Vec types to extract single value from document
- `IndexFieldVec<T>`: For Vec<T> to extract all values from document

### Blanket Implementations
Provided for:
- `String` - TEXT/STRING indexed
- `i32`, `i64` - i64 indexed fields
- `u32`, `u64` - u64 indexed fields
- `f64` - f64 indexed fields
- `bool` - boolean indexed fields
- `DateTime` (tantivy::DateTime) - date indexed fields
- `Vec<T>` where T: IndexField - multi-value fields

## Extensibility
Users can implement `IndexField` for their own types:

```rust
impl IndexField for MyCustomType {
    fn type_name() -> &'static str { "MyCustomType" }
    fn add_to_schema(...) { ... }
    fn add_to_document(...) { ... }
    fn from_owned_value(...) { ... }
    fn to_term(...) { ... }
}
```

## Key File Locations
- `text-search-core/src/index_field.rs` - IndexField trait and implementations
- `text-search-core/src/indexable.rs` - Indexable trait
- `text-search-core/src/field_info.rs` - Field metadata (no longer contains type info)
- `text-search-derive/src/field_info.rs` - Derive-time field parsing
- `text-search-derive/src/indexable.rs` - Indexable derive macro
- `text-search/src/index_writer.rs` - IndexWriter implementation
- `text-search/src/index_reader.rs` - IndexReader with search methods

## User Preferences
- Do not fix errors unless explicitly asked
- Do not check for errors/warnings after adding code
- Only provide code, no explanations unless asked
