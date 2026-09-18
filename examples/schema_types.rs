use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-schema";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Search by ID field
    println!("=== Search by ID field ===");
    let query = SearchQuery::new(Book::id, "5").page(1).per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books with id=5:", result.total_items);
    for book in &result.data {
        println!("  - {} (id: {})", book.name, book.id);
    }

    // Search on indexed_string field (exact match behavior differs)
    println!("\n=== Search on indexed_string field (author) ===");
    // Note: indexed_string treats the whole value as one token
    // So "Steve Klabnik and Carol Nichols" is one token, not separate words
    let query = SearchQuery::new(Book::author, "Bogdan")
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books by Bogdan:", result.total_items);
    for book in &result.data {
        println!("  - {}", book.name);
    }

    // Search on indexed_string field with full author name
    println!("\n=== Search on indexed_string with full author name ===");
    let query = SearchQuery::new(Book::author, "Steve Klabnik and Carol Nichols")
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }

    // Search on Vec<String> field (tags)
    println!("\n=== Search on Vec<String> field (tags) ===");
    let query = SearchQuery::new(Book::tags, "xyz").page(1).per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books with 'xyz' tag:", result.total_items);
    for book in &result.data {
        println!("  - {} (tags: {:?})", book.name, book.tags);
    }
}
