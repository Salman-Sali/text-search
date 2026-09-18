use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-filter-eq";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Filter by exact author match
    println!("=== Filter: author = 'Bogdan' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("Bogdan"))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books by Bogdan:", result.total_items);
    for book in &result.data {
        println!("  - {}", book.name);
    }

    // Filter by tag
    println!("\n=== Filter: tags = 'xyz' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::tags.eq("xyz"))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books with 'xyz' tag:", result.total_items);
    for book in &result.data {
        println!("  - {} (tags: {:?})", book.name, book.tags);
    }

    // Filter by not equal
    println!("\n=== Filter: author != 'Bogdan' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.ne("Bogdan"))
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books not by Bogdan:", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }
}
