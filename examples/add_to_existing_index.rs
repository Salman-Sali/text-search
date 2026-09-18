use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-reindex";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();

    // Add initial set of books
    let initial_books = vec![
        Book {
            id: 1,
            name: "Book One".to_string(),
            author: "Author A".to_string(),
            description: "First book".to_string(),
            published_on: 2021,
            tags: vec!["initial".to_string()],
        },
        Book {
            id: 2,
            name: "Book Two".to_string(),
            author: "Author B".to_string(),
            description: "Second book".to_string(),
            published_on: 2022,
            tags: vec!["initial".to_string()],
        },
    ];

    for book in &initial_books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Show initial books
    println!("=== Initial books ===");
    let query = SearchQuery::new(Book::name, "Book").page(1).per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Total: {} books", result.total_items);
    for book in &result.data {
        println!(
            "  - {} by {} (tags: {:?})",
            book.name, book.author, book.tags
        );
    }

    // Add more books to existing index
    println!("\n=== Adding more books... ===");
    let more_books = vec![
        Book {
            id: 3,
            name: "Book Three".to_string(),
            author: "Author C".to_string(),
            description: "Third book".to_string(),
            published_on: 2023,
            tags: vec!["added".to_string()],
        },
        Book {
            id: 4,
            name: "Book Four".to_string(),
            author: "Author D".to_string(),
            description: "Fourth book".to_string(),
            published_on: 2024,
            tags: vec!["added".to_string()],
        },
    ];

    for book in &more_books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();
    index_reader.reload().unwrap();

    // Show all books after adding
    println!("\n=== All books after adding ===");
    let query = SearchQuery::new(Book::name, "Book").page(1).per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Total: {} books", result.total_items);
    for book in &result.data {
        println!(
            "  - {} by {} (tags: {:?})",
            book.name, book.author, book.tags
        );
    }

    // Filter to only show newly added books
    println!("\n=== Filtered to 'added' tag ===");
    let query = SearchQuery::new(Book::name, "Book")
        .with_filter(Book::tags.eq("added"))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books with 'added' tag:", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }
}
