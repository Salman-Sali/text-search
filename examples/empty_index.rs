use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-empty";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();

    // Don't add any books - create an empty index
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Search on empty index
    println!("=== Search on empty index ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Total items: {}", result.total_items);
    println!("Data length: {}", result.data.len());
    println!("Page: {} of {}", result.page, result.total_pages);

    // Hybrid search on empty index
    println!("\n=== Hybrid search on empty index ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Total items: {}", result.total_items);
    println!("Data length: {}", result.data.len());
    println!("Page: {} of {}", result.page, result.total_pages);

    // Search with filter on empty index
    println!("\n=== Search with filter on empty index ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("Bogdan"))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Total items: {}", result.total_items);
    println!("Data length: {}", result.data.len());

    // Now add some books
    println!("\n=== Adding books to previously empty index ===");
    let books = Book::get_sample_books();
    for book in &books[..3] {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();
    index_reader.reload().unwrap();

    // Search again
    println!("\n=== Search after adding books ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Total items: {}", result.total_items);
    for book in &result.data {
        println!("  - {}", book.name);
    }
}
