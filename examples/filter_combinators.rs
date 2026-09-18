use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-filter-comb";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    // AND filter: author = 'Bogdan' AND tags = 'xyz'
    println!("=== Filter: author = 'Bogdan' AND tags = 'xyz' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("Bogdan").and(Book::tags.eq("xyz")))
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!(
            "  - {} by {} (tags: {:?})",
            book.name, book.author, book.tags
        );
    }

    // OR filter: author = 'Bogdan' OR author = 'Jon Gjengset'
    println!("\n=== Filter: author = 'Bogdan' OR author = 'Jon Gjengset' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(
            Book::author
                .eq("Bogdan")
                .or(Book::author.eq("Jon Gjengset")),
        )
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }

    // Complex filter: (author = 'Bogdan' AND published_on >= 2021) OR (author = 'Tim McNamara')
    println!("\n=== Filter: (author = 'Bogdan' AND year >= 2021) OR author = 'Tim McNamara' ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(
            Book::author
                .eq("Bogdan")
                .and(Book::published_on.ge(2021))
                .or(Book::author.eq("Tim McNamara")),
        )
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!(
            "  - {} by {} ({})",
            book.name, book.author, book.published_on
        );
    }

    // Triple AND filter
    println!("\n=== Filter: author = 'Bogdan' AND tags = 'abc' AND year >= 2021 ===");
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(
            Book::author
                .eq("Bogdan")
                .and(Book::tags.eq("abc"))
                .and(Book::published_on.ge(2021)),
        )
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!(
            "  - {} by {} (tags: {:?}, {})",
            book.name, book.author, book.tags, book.published_on
        );
    }

    // Triple OR filter
    println!(
        "\n=== Filter: author = 'Bogdan' OR author = 'Jon Gjengset' OR author = 'Tim McNamara' ==="
    );
    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(
            Book::author
                .eq("Bogdan")
                .or(Book::author.eq("Jon Gjengset"))
                .or(Book::author.eq("Tim McNamara")),
        )
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Found {} books:", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }
}
