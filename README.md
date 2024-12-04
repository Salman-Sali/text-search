A plug and play wrapper around tantivy.

Diesel is to SQL as text-search is to tantivy.

UNDER DEVELOPMENT

Currently will only function if:  
  - all fields are "stored"
  - fields must be String or i32 
  - refer example 

```rust
fn main() {
  let indexer = Indexer::new(Path::new("/path/to/your/dir"));
  let books = Book::get_sample_books();
  
  for book in books {
      indexer.index(book);        
  }
  let result = indexer.search("name", "Let's Get Rusty Vol 1", 10);
}
```
