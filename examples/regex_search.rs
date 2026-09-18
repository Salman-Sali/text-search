use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-regex";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Regex search - match any term starting with "rust"
    println!("=== Regex Search: 'rust.*' ===");
    let query = SearchQuery::new(Book::name, "rust.*").page(1).per_page(10);
    let result = index_reader.regex_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {}", book.name);
    }

    // Regex search - match terms containing "program"
    println!("\n=== Regex Search: '.*program.*' ===");
    let query = SearchQuery::new(Book::description, ".*program.*")
        .page(1)
        .per_page(10);
    let result = index_reader.regex_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {} (desc: {})", book.name, book.description);
    }

    // Regex search on author field (indexed_string)
    println!("\n=== Regex Search on author: 'bog.*' ===");
    let query = SearchQuery::new(Book::author, "bog.*").page(1).per_page(10);
    let result = index_reader.regex_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }
}
