A plug and play wrapper around tantivy.

Diesel is to SQL as text-search is to tantivy.

UNDER DEVELOPMENT

## Quick Start

```rust
use text_search::{IndexWriter, Indexable};

#[derive(Indexable)]
pub struct Book {
    #[text_search(id)]
    pub id: i32,
    #[text_search(indexed_text, stored)]
    pub name: String,
    #[text_search(indexed_string, stored)]
    pub author: String,
    #[text_search(indexed_text, stored)]
    pub description: String,
    pub published_on: i32
}

fn main() {
    let mut index_writer = IndexWriter::<Book>::new(Path::new("/path/to/your/dir"), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    
    for book in books {
        index_writer.add(book);        
    }
    let _ = index_writer.commit();

    let index_reader = index_writer.create_index_reader().unwrap();

    let basic_search_result = index_reader.search(HashMap::new(), "name", "Rust", 1, 10).unwrap().data;

    let fuzzy_search_result = index_reader.fuzzy_search(HashMap::new(), "name", "Rosty", 1, 10).unwrap().data;

    let regex_search_result = index_reader.regex_search(HashMap::new(), "name", "rustacea.*", 1, 10).unwrap().data;
}
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
#[derive(Indexable)]
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

### Conflicts

These combinations will cause a compile-time panic:
- Multiple `index_type` attributes on the same field
- Multiple `stored`/`not_stored` on the same field  
- `id` combined with any other attribute
