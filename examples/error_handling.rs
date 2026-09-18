use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-errors";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Handle empty search results gracefully
    println!("=== Search for non-existent term ===");
    let query = SearchQuery::new(Book::name, "NonExistentBookXYZ")
        .page(1)
        .per_page(10);
    match index_reader.search(&query) {
        Ok(result) => {
            println!("Search succeeded (no panic for empty results)");
            println!("Total items: {}", result.total_items);
            println!("Data is empty: {}", result.data.is_empty());
        }
        Err(e) => {
            println!("Search failed: {:?}", e);
        }
    }

    // Search with no matches in filter
    println!("\n=== Filter that matches nothing ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("NonExistentAuthor"))
        .page(1)
        .per_page(10);
    match index_reader.search(&query) {
        Ok(result) => {
            println!("Search succeeded with filter");
            println!("Total items: {}", result.total_items);
        }
        Err(e) => {
            println!("Search failed: {:?}", e);
        }
    }

    // Page beyond results
    println!("\n=== Page beyond available results ===");
    let query = SearchQuery::new(Book::name, "Rust").page(100).per_page(10);
    match index_reader.search(&query) {
        Ok(result) => {
            println!("Search succeeded");
            println!("Total items: {}", result.total_items);
            println!("Page: {} of {}", result.page, result.total_pages);
            println!("Data length: {}", result.data.len());
        }
        Err(e) => {
            println!("Search failed: {:?}", e);
        }
    }

    // Valid searches still work
    println!("\n=== Valid search ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(5);
    match index_reader.search(&query) {
        Ok(result) => {
            println!("Search succeeded");
            println!("Found {} books:", result.total_items);
            for book in &result.data {
                println!("  - {}", book.name);
            }
        }
        Err(e) => {
            println!("Search failed: {:?}", e);
        }
    }
}
