A plug and play wrapper around tantivy.

Diesel is to SQL as text-search is to tantivy.

UNDER DEVELOPMENT

Currently will only function if:  
  - all fields are "stored"
  - fields must be String or i32 
  - refer example 

```rust
use text_search::{Indexer, Indexed};

#[derive(Indexed)]
pub struct Book {
    //default is #[text_search(not_indexed, stored)]

    //id behaves like #[text_search(indexed, stored)]
    #[text_search(id)]
    pub id: i32,
    #[text_search(indexed_text, stored)]
    pub name: String,
    #[text_search(indexed_text, stored)]
    pub author: String,
    #[text_search(indexed_text, stored)]
    pub description: String,
    pub published_on: i32
}

fn main() {
  let mut index_writer = IndexWriter::<Book>::new(Path::new("/path/to/your/dir"), 50_000_000).unwrap();
  let books = Book::get_sample_books();
  
  for book in books {
      index_writer.index(book);        
  }
  let _ = index_writer.commit();

  let index_reader = index_writer.create_index_reader().unwrap();

  let basic_search_result: Vec<Book> = index_reader.search(HashMap::new(), "name", "Rust", 10);

  let fuzzy_search_result: Vec<Book> = index_reader.fuzzy_search(HashMap::new(), "name", "Rosty", 10);

  let regex_search_result: Vec<Book> = index_reader.regex_query(HashMap::new(), "name", "rustacea.*", 10);
}
```
