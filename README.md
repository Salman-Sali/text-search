A plug and play wrapper around tantivy.

Diesel is to SQL as text-search is to tantivy.

## Quick Start

```rust
use text_search::{IndexWriter, Indexed, SearchQuery};
use std::path::Path;

#[derive(Indexed, Clone, Debug)]
pub struct Book {
    #[text_search(id)]
    pub id: i32,
    #[text_search(indexed_text, stored)]
    pub name: String,
    #[text_search(indexed_string, stored)]
    pub author: String,
    #[text_search(indexed_text, stored)]
    pub description: String,
    #[text_search(indexed, stored)]
    pub published_on: i32,
    #[text_search(indexed_string, stored)]
    pub tags: Vec<String>
}



fn main() {
    // Create an index
    let mut index_writer = IndexWriter::<Book>::new(Path::new("/path/to/your/dir"), 50_000_000).unwrap();
    let books = get_sample_books();
    
    for book in books {
        index_writer.add(book);        
    }
    let _ = index_writer.commit();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Basic search
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let basic_search_result = index_reader.search(&query).unwrap().data;

    // Fuzzy search (tolerates typos)
    let query = SearchQuery::new(Book::name, "Rosty").page(1).per_page(10);
    let fuzzy_search_result = index_reader.fuzzy_search(&query).unwrap().data;

    // Regex search
    let query = SearchQuery::new(Book::name, "rustacea.*").page(1).per_page(10);
    let regex_search_result = index_reader.regex_search(&query).unwrap().data;
}
```

## Filtering

Use typed field references to build filter expressions:

```rust
// Exact match on string field
Book::author.eq("Bogdan")

// Combine with AND
Book::author.eq("Bogdan").and(Book::tags.eq("xyz"))

// Combine with OR
Book::author.eq("Bogdan").or(Book::author.eq("Tim McNamara"))

// Numeric comparisons
Book::published_on.gt(2020)
Book::published_on.ge(2020)  // >=
Book::published_on.lt(2025)
Book::published_on.le(2025)  // <=
Book::published_on.ne(2022)  // !=

// Complex chains
Book::author.eq("Bogdan")
    .and(Book::tags.eq("abc"))
    .and(Book::published_on.ge(2020))
```

Use filters with `SearchQuery`:

```rust
let query = SearchQuery::new(Book::name, "Rust")
    .with_filter(Book::author.eq("Bogdan"))
    .page(1)
    .per_page(10);

let result = index_reader.search(&query).unwrap();
```

## Search Methods

| Method | Description |
|--------|-------------|
| `search(&query)` | Standard full-text search with query parsing |
| `fuzzy_search(&query)` | Tolerates typos (e.g., "Rsut" finds "Rust") |
| `regex_search(&query)` | Pattern matching with regular expressions |
| `hybrid_search(&query)` | Combines fuzzy + phrase prefix (best results) |

## Pagination

All search methods return `PaginatedResult<T>`:

```rust
let query = SearchQuery::new(Book::name, "Rust")
    .page(1)
    .per_page(10);

let result = index_reader.search(&query).unwrap();

println!("Page: {} of {}", result.page, result.total_pages);
println!("Total items: {}", result.total_items);
println!("Data: {:?}", result.data);
```

Methods on `SearchQuery`:
- `new(field: FieldRef, query: impl Into<String>)` - Create a new query
- `with_filter(filter: Filter)` - Add a filter expression
- `page(n: i64)` - Set page number (default: 1)
- `per_page(n: i64)` - Set page size (default: 10)

## Deleting

```rust
// Delete by document instance
let book = /* ... */;
index_writer.delete(book);

// Delete by term (fast, for known ID)
let term = Book::get_term_from_id(42);
index_writer.delete_using_term(term);

// Delete by filter expression
index_writer.delete_by_filter(
    &Book::author.eq("Steve Klabnik and Carol Nichols")
        .and(Book::name.eq("The Rust Programming Language"))
);

index_writer.commit().unwrap();
```

## Field Attributes

Use `#[text_search(...)]` to configure how each field is indexed and stored.

### Default Behavior

If no attribute is specified, the default is `#[text_search(not_indexed, stored)]`.

### Attributes

| Attribute | Applies To | Description |
|-----------|------------|-------------|
| `id` | Any field | Marks the field as the document ID. Automatically applies `indexed` and `stored`. **Cannot be combined with other attributes.** |
| `indexed_string` | `String`, `Vec<String>` | Indexed but **untokenized** - stored as a single term. Good for exact matches, tags, enums. |
| `indexed_text` | `String`, `Vec<String>` | Indexed and **tokenized** - split into individual words. Good for full-text search. |
| `indexed` | Numeric types, `bool`, `DateTime` | Indexed for range/filter queries. For `String`, behaves like `indexed_string`. |
| `not_indexed` | All types | Not indexed - can only be retrieved from stored docs, not searched. |
| `stored` | All types | Field value is stored and retrievable in search results. |
| `not_stored` | All types | Field is indexed but value not stored (searchable but not retrievable). |

### `indexed_string` vs `indexed_text`

The key difference is how text is processed:

| Index Type | Stored as | Search Behavior |
|------------|-----------|-----------------|
| `indexed_string` | Single term: `"Steve Klabnik and Carol Nichols"` | Exact match only. Searching for `"Steve"` won't match. |
| `indexed_text` | Multiple tokens: `"steve"`, `"klabnik"`, `"and"`, `"carol"`, `"nichols"` | Word-by-word match. Searching for `"steve"` or `"klabnik"` matches. |

### Supported Types

| Type | `indexed_string` | `indexed_text` | `indexed` | `not_indexed` |
|------|------------------|----------------|-----------|---------------|
| `String` | ✓ | ✓ | ✓ | ✓ |
| `Vec<String>` | ✓ | ✓ | ✓ | ✓ |
| `i32`, `i64`, `u32`, `u64` | ✗ | ✗ | ✓ | ✓ |
| `f64` | ✗ | ✗ | ✓ | ✓ |
| `bool` | ✗ | ✗ | ✓ | ✓ |
| `DateTime` | ✗ | ✗ | ✓ | ✓ |
| `Vec<T>` (numeric/bool/DateTime) | ✗ | ✗ | ✓ | ✓ |

### Examples

```rust
#[derive(Indexed, Clone, Debug)]
pub struct Book {
    // Document ID - indexed and stored
    #[text_search(id)]
    pub id: i32,
    
    // Tokenized for full-text search
    #[text_search(indexed_text, stored)]
    pub name: String,
    
    // Untokenized - exact match only
    #[text_search(indexed_string, stored)]
    pub author: String,
    
    // Multiple exact-match tags
    #[text_search(indexed_string, stored)]
    pub tags: Vec<String>,
    
    // Filterable but not returned in results
    #[text_search(indexed, not_stored)]
    pub year: i32,
    
    // Retrieved but not searchable
    #[text_search(not_indexed, stored)]
    pub metadata: String,
}
```

### Field References

The derive macro generates typed field references as associated constants:

```rust
#[derive(Indexed)]
pub struct Book {
    #[text_search(id)]
    pub id: i32,
    #[text_search(indexed_text, stored)]
    pub name: String,
    // ...
}

// Generated associated constants:
// Book::id      -> FieldRef
// Book::name    -> FieldRef
// Book::author  -> FieldRef
// ... etc for each field
```

These are used with `SearchQuery::new(field, query)` and filter methods like `.eq()`, `.gt()`, etc.

### Conflicts

These combinations will cause a compile-time error:
- Multiple `index_type` attributes on the same field
- Multiple `stored`/`not_stored` on the same field  
- `id` combined with any other attribute
