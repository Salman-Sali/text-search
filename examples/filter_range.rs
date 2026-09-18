use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-filter-range";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // Range filter: year between 2018 and 2020 (inclusive)
    println!("=== Range Filter: 2018 <= published_on <= 2020 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.ge(2018).and(Book::published_on.le(2020)))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Found {} books published between 2018-2020:",
        result.total_items
    );
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }

    // Range filter: year > 2018 AND year < 2021 (exclusive range)
    println!("\n=== Range Filter: 2018 < published_on < 2021 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.gt(2018).and(Book::published_on.lt(2021)))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Found {} books published between 2018-2021 (exclusive):",
        result.total_items
    );
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }

    // Open-ended range: year >= 2020
    println!("\n=== Open Range: published_on >= 2020 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.ge(2020))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books from 2020 onwards:", result.total_items);
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }

    // Open-ended range: year < 2020
    println!("\n=== Open Range: published_on < 2020 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.lt(2020))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books before 2020:", result.total_items);
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }

    // Complex range with author filter: year >= 2019 AND author = 'Bogdan'
    println!("\n=== Range + Author: published_on >= 2019 AND author = 'Bogdan' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.ge(2019).and(Book::author.eq("Bogdan")))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Found {} books by Bogdan from 2019 onwards:",
        result.total_items
    );
    for book in &result.data {
        println!("  - {} ({})", book.name, book.published_on);
    }

    // Triple range: year >= 2018 AND year <= 2021 AND tags = 'abc'
    println!("\n=== Triple Filter: 2018-2021 AND tags = 'abc' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(
            Book::published_on
                .ge(2018)
                .and(Book::published_on.le(2021))
                .and(Book::tags.eq("abc")),
        )
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!(
        "Found {} books from 2018-2021 with 'abc' tag:",
        result.total_items
    );
    for book in &result.data {
        println!(
            "  - {} ({}, tags: {:?})",
            book.name, book.published_on, book.tags
        );
    }

    // Range with OR: (year >= 2019) OR (author = 'Jim Blandy and Jason Orendorff')
    println!("\n=== Range OR Author: published_on >= 2019 OR author = 'Jim Blandy...' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(
            Book::published_on
                .ge(2019)
                .or(Book::author.eq("Jim Blandy and Jason Orendorff")),
        )
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!(
        "Found {} books (2019+ OR by Jim Blandy):",
        result.total_items
    );
    for book in &result.data {
        println!(
            "  - {} by {} ({})",
            book.name, book.author, book.published_on
        );
    }
}
