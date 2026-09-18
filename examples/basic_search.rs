use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-basic";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Basic search - find books with "Rust" in the name
    println!("=== Basic Search ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books with 'Rust' in name:", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }

    // Search with pagination - page 1
    println!("\n=== Pagination (page 1, 3 per page) ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(3);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Page {} of {} (total: {})",
        result.page, result.total_pages, result.total_items
    );
    for book in &result.data {
        println!("  - {}", book.name);
    }

    // Search with pagination - page 2
    println!("\n=== Pagination (page 2, 3 per page) ===");
    let query = SearchQuery::new(Book::name, "Rust").page(2).per_page(3);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Page {} of {} (total: {})",
        result.page, result.total_pages, result.total_items
    );
    for book in &result.data {
        println!("  - {}", book.name);
    }
}
