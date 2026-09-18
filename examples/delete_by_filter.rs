use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-delete";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Show all books before deletion
    println!("=== All books before deletion ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Total: {} books", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }

    // Delete books by a specific author
    println!("\n=== Deleting books by 'Bogdan' ===");
    index_writer.delete_by_filter(&Book::author.eq("Bogdan"));
    index_writer.commit().unwrap();
    index_reader.reload().unwrap();

    // Show remaining books
    println!("\n=== Books after deleting Bogdan's books ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Total: {} books", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }

    // Delete books with specific tag and year
    println!("\n=== Deleting books with tags = 'abc' AND year < 2020 ===");
    index_writer.delete_by_filter(&Book::tags.eq("abc").and(Book::published_on.lt(2020)));
    index_writer.commit().unwrap();
    index_reader.reload().unwrap();

    // Show remaining books
    println!("\n=== Books after second deletion ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Total: {} books", result.total_items);
    for book in &result.data {
        println!(
            "  - {} by {} (tags: {:?}, {})",
            book.name, book.author, book.tags, book.published_on
        );
    }
}
