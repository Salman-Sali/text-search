use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-memory";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();

    // Add books in batches to demonstrate memory management
    println!("=== Adding books in batches ===");

    for batch in 0..3 {
        println!("Batch {}:", batch + 1);
        let batch_books: Vec<Book> = (1..=10)
            .map(|i| {
                let id = batch * 10 + i;
                Book {
                    id,
                    name: format!("Rust Book {}", id),
                    author: format!("Author {}", id % 3),
                    description: format!("Description for book {}", id),
                    published_on: 2020 + (id % 5),
                    tags: vec![format!("tag {}", id % 4)],
                    isbn: if id % 2 == 0 {
                        Some(format!("978-{:010}", id))
                    } else {
                        None
                    },
                }
            })
            .collect();

        for book in &batch_books {
            index_writer.add(book.clone());
            println!("  Added: {}", book.name);
        }
    }

    index_writer.commit().unwrap();
    println!("\nCommitted all batches");

    let index_reader = index_writer.create_index_reader().unwrap();

    // Search across all batches
    println!("\n=== Searching across all batches ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(50);
    let result = index_reader.search(&query).unwrap();
    println!("Total books: {}", result.total_items);
    println!("Page {} of {}", result.page, result.total_pages);

    // Filter by author from different batches
    println!("\n=== Filter by author 'Author 0' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("Author 0"))
        .page(1)
        .per_page(50);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books by Author 0:", result.total_items);
    for book in &result.data {
        println!("  - {} (year: {})", book.name, book.published_on);
    }

    // Reload index multiple times (demonstrates reader lifecycle)
    println!("\n=== Reloading index ===");
    for i in 1..=3 {
        index_reader.reload().unwrap();
        println!("Reload #{} successful", i);
    }

    // Verify data still accessible after reloads
    println!("\n=== Data after reloads ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Still found {} books", result.total_items);
}
