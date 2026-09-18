use book::Book;
use std::{fs, path::Path, time::Instant};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-large";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();

    // Generate many books
    let num_books = 1000;
    println!("=== Adding {} books ===", num_books);
    let start = Instant::now();

    for i in 1..=num_books {
        let book = Book {
            id: i,
            name: format!("Rust Programming Volume {}", i),
            author: format!("Author {}", i % 100),
            description: format!("This is the description for book number {} about Rust.", i),
            published_on: 2000 + (i % 25),
            tags: vec![format!("tag{}", i % 10), format!("category{}", i % 5)],
        };
        index_writer.add(book);
    }

    index_writer.commit().unwrap();
    let elapsed = start.elapsed();
    println!("Indexed {} books in {:?}", num_books, elapsed);

    let index_reader = index_writer.create_index_reader().unwrap();

    // Simple search
    println!("\n=== Simple search ===");
    let start = Instant::now();
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Found {} books in {:?}",
        result.total_items,
        start.elapsed()
    );

    // Filtered search
    println!("\n=== Filtered search: year >= 2015 ===");
    let start = Instant::now();
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.ge(2015))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Found {} books in {:?}",
        result.total_items,
        start.elapsed()
    );

    // Search by specific tag
    println!("\n=== Search by tag 'tag5' ===");
    let start = Instant::now();
    let query = SearchQuery::new(Book::tags, "tag5").page(1).per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!(
        "Found {} books in {:?}",
        result.total_items,
        start.elapsed()
    );

    // Pagination test
    println!("\n=== Pagination test ===");
    for page in 1..=5 {
        let start = Instant::now();
        let query = SearchQuery::new(Book::name, "Rust").page(page).per_page(20);
        let result = index_reader.search(&query).unwrap();
        println!(
            "Page {}: {} results in {:?}",
            page,
            result.data.len(),
            start.elapsed()
        );
    }

    // Complex filter
    println!("\n=== Complex filter: author = 'Author 0' AND year >= 2010 ===");
    let start = Instant::now();
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("Author 0").and(Book::published_on.ge(2010)))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Found {} books in {:?}",
        result.total_items,
        start.elapsed()
    );
}
