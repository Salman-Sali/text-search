use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-fuzzy";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Fuzzy search with misspelling - "Rsut" instead of "Rust"
    println!("=== Fuzzy Search: 'Rsut' (misspelled) ===");
    let query = SearchQuery::new(Book::name, "Rsut").page(1).per_page(10);
    let result = index_reader.fuzzy_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {}", book.name);
    }

    // Fuzzy search with another misspelling - "Ruty" instead of "Rusty"
    println!("\n=== Fuzzy Search: 'Ruty' (misspelled) ===");
    let query = SearchQuery::new(Book::name, "Ruty").page(1).per_page(10);
    let result = index_reader.fuzzy_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {}", book.name);
    }

    // Fuzzy search with description field
    println!("\n=== Fuzzy Search in description: 'oficial' (misspelled) ===");
    let query = SearchQuery::new(Book::description, "oficial")
        .page(1)
        .per_page(10);
    let result = index_reader.fuzzy_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {} (desc: {})", book.name, book.description);
    }
}
