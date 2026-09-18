use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-filter-numeric";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Filter by year greater than
    println!("=== Filter: published_on > 2019 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.gt(2019))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books published after 2019:", result.total_items);
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }

    // Filter by year greater than or equal
    println!("\n=== Filter: published_on >= 2020 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.ge(2020))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Found {} books published in 2020 or later:",
        result.total_items
    );
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }

    // Filter by year less than
    println!("\n=== Filter: published_on < 2020 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.lt(2020))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books published before 2020:", result.total_items);
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }

    // Filter by year less than or equal
    println!("\n=== Filter: published_on <= 2019 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.le(2019))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Found {} books published in 2019 or earlier:",
        result.total_items
    );
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }

    // Filter by exact year
    println!("\n=== Filter: published_on = 2021 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.eq(2021))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books published in 2021:", result.total_items);
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }
}
