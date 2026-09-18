use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, SearchQuery};

mod book;

fn main() {
    let path = "/tmp/text-search-compare";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);

    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    let search_term = "Rust";
    println!(
        "=== Comparing search types for term: '{}' ===\n",
        search_term
    );

    // Standard search
    println!("1. Standard search (search):");
    let query = SearchQuery::new(Book::name, search_term)
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("   Found {} books:", result.total_items);
    for book in &result.data {
        println!("      - {}", book.name);
    }

    // Hybrid search
    println!("\n2. Hybrid search (hybrid_search):");
    let query = SearchQuery::new(Book::name, search_term)
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("   Found {} books:", result.total_items);
    for book in &result.data {
        println!("      - {}", book.name);
    }

    // Fuzzy search
    println!("\n3. Fuzzy search (fuzzy_search) with 'Rsut':");
    let query = SearchQuery::new(Book::name, "Rsut").page(1).per_page(10);
    let result = index_reader.fuzzy_search(&query).unwrap();
    println!("   Found {} books:", result.total_items);
    for book in &result.data {
        println!("      - {}", book.name);
    }

    // Regex search
    println!("\n4. Regex search (regex_search) with 'rust.*':");
    let query = SearchQuery::new(Book::name, "rust.*").page(1).per_page(10);
    let result = index_reader.regex_search(&query).unwrap();
    println!("   Found {} books:", result.total_items);
    for book in &result.data {
        println!("      - {}", book.name);
    }

    // Search in description field
    let desc_term = "programming";
    println!(
        "\n=== Comparing search in description field for term: '{}' ===\n",
        desc_term
    );

    println!("1. Standard search:");
    let query = SearchQuery::new(Book::description, desc_term)
        .page(1)
        .per_page(10);
    let result = index_reader.search(&query).unwrap();
    println!("   Found {} books", result.total_items);

    println!("\n2. Hybrid search:");
    let query = SearchQuery::new(Book::description, desc_term)
        .page(1)
        .per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("   Found {} books", result.total_items);

    println!("\n3. Fuzzy search with 'programing' (misspelled):");
    let query = SearchQuery::new(Book::description, "programing")
        .page(1)
        .per_page(10);
    let result = index_reader.fuzzy_search(&query).unwrap();
    println!("   Found {} books", result.total_items);

    // All with filter
    println!("\n=== All search types with author filter ===");
    let filter = Book::author.eq("Bogdan");

    println!(
        "Standard search with filter: {} books",
        index_reader
            .search(
                &SearchQuery::new(Book::name, "Rust")
                    .with_filter(filter.clone())
                    .page(1)
                    .per_page(10)
            )
            .unwrap()
            .total_items
    );
    println!(
        "Hybrid search with filter: {} books",
        index_reader
            .hybrid_search(
                &SearchQuery::new(Book::name, "Rust")
                    .with_filter(filter.clone())
                    .page(1)
                    .per_page(10)
            )
            .unwrap()
            .total_items
    );
    println!(
        "Fuzzy search with filter: {} books",
        index_reader
            .fuzzy_search(
                &SearchQuery::new(Book::name, "Rsut")
                    .with_filter(filter.clone())
                    .page(1)
                    .per_page(10)
            )
            .unwrap()
            .total_items
    );
    println!(
        "Regex search with filter: {} books",
        index_reader
            .regex_search(
                &SearchQuery::new(Book::name, "rust.*")
                    .with_filter(filter.clone())
                    .page(1)
                    .per_page(10)
            )
            .unwrap()
            .total_items
    );
}
