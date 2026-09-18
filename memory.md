# text-search Project Memory

## Project Overview
A Rust text search library built on top of Tantivy, providing derive macros for indexing and searching structs with a Diesel-like query builder API.

## Derive Macro Usage

The `Indexed` derive macro works without requiring `Indexable` trait in scope:

```rust
use text_search::Indexed; // Only need to import the derive macro

#[derive(Indexed, Clone, Debug)]
pub struct Book {
    #[text_search(id)]
    pub id: i32,
    #[text_search(indexed_text, stored)]
    pub name: String,
    #[text_search(indexed_string, stored)]
    pub author: String,
}
```

## Diesel-like SearchQuery API

### FieldRef and Filter Combinators
```rust
// Generate FieldRef constants via derive macro
#[derive(Indexed)]
pub struct Book {
    #[text_search(id)]
    pub id: i32,
    #[text_search(indexed_text, stored)]
    pub name: String,
    #[text_search(indexed_string, stored)]
    pub author: String,
}

// Use field references for type-safe queries
let query = SearchQuery::new(Book::name, "Rust")
    .with_filter(Book::author.eq("Bogdan").and(Book::published_on.gt(2020)))
    .page(1)
    .per_page(10);

let results = reader.search(&query)?;
```

### Filter Operations
- `.eq(value)` - Equal
- `.ne(value)` - Not equal
- `.gt(value)` - Greater than
- `.ge(value)` - Greater than or equal
- `.lt(value)` - Less than
- `.le(value)` - Less than or equal

### Filter Combinators
- `filter1.and(filter2)` - AND combination
- `filter1.or(filter2)` - OR combination

### Core Types (text-search-core/src/search_query.rs)
- `FieldRef` - Reference to a searchable field
- `SearchQuery` - Builder for search queries
- `Filter` - Filter conditions with AND/OR combinators
- `FilterValue` - Typed filter values (Str, I64, U64, F64, Bool, Date)
- `FilterOp` - Filter operators (Eq, Ne, Gt, Ge, Lt, Le)

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
- `text-search-core/src/field_info.rs` - Field metadata
- `text-search-core/src/search_query.rs` - SearchQuery, Filter, FieldRef types
- `text-search-derive/src/field_info.rs` - Derive-time field parsing (uses `darling` 0.20 for `#[text_search(...)]` attribute parsing)
- `text-search-derive/src/indexable.rs` - Indexable derive macro (generates FieldRef constants). **Note**: Uses fully-qualified syntax `<Self as text_search::Indexable>::generate_schema()` to avoid requiring users to import the `Indexable` trait.
- `text-search/src/index_writer.rs` - IndexWriter with `delete_by_filter()`
- `text-search/src/index_reader.rs` - IndexReader with search/hybrid_search/fuzzy_search/regex_search

## Examples Directory
17+ comprehensive examples in `/examples/`:
- `basic_search.rs` - Basic search and pagination
- `fuzzy_search.rs` - Fuzzy matching with typos
- `regex_search.rs` - Regular expression searches
- `filter_equality.rs` - Equality/inequality filters
- `filter_numeric.rs` - Numeric range filters (gt, ge, lt, le)
- `filter_range.rs` - Combined range filters (between ranges)
- `filter_combinators.rs` - AND/OR filter combinations
- `hybrid_search.rs` - Hybrid search strategy
- `delete_by_filter.rs` - Delete with filter API
- `delete_by_term.rs` - Delete by exact term
- `add_to_existing_index.rs` - Incremental indexing
- `empty_index.rs` - Handle empty indexes
- `schema_types.rs` - Different field types demo
- `memory_management.rs` - Index lifecycle
- `large_index.rs` - Performance with 1000+ docs
- `error_handling.rs` - Graceful error handling
- `compare_search_types.rs` - Compare all search methods
- `filter.rs`, `index_and_filter_delete.rs`, `index_and_term_delete.rs` - Original examples

Run examples: `cargo run --example basic_search`
