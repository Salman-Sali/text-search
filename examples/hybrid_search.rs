use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-hybrid";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Hybrid search combines the power of multiple search strategies
    println!("=== Hybrid Search: 'Rust' ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {}", book.name);
    }

    // Hybrid search with filter
    println!("\n=== Hybrid Search with filter: 'Rust' where author = 'Bogdan' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("Bogdan"))
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {}", book.name);
    }

    // Hybrid search in description field
    println!("\n=== Hybrid Search in description: 'programming' ===");
    let query = SearchQuery::new(Book::description, "programming")
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {} (desc: {})", book.name, book.description);
    }

    // Hybrid search with complex filter
    println!("\n=== Hybrid Search: 'Rust' where (author = 'Bogdan' OR tags = 'xyz') ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("Bogdan").or(Book::tags.eq("xyz")))
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!(
            "  - {} by {} (tags: {:?})",
            book.name, book.author, book.tags
        );
    }
}
