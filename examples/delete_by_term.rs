use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, Indexable, SearchQuery, tantivy::Term};

mod book;

fn main() {
    let path = "/tmp/text-search-term-del";
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

    // Delete by exact term match on author field
    println!("\n=== Deleting by term: author = 'Jon Gjengset' ===");
    let schema = Book::generate_schema();
    let author_field = schema.get_field("author").unwrap();
    let term = Term::from_field_text(author_field, "Jon Gjengset");
    index_writer.delete_using_term(term);
    index_writer.commit().unwrap();
    index_reader.reload().unwrap();

    // Show remaining books
    println!("\n=== Books after term deletion ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Total: {} books", result.total_items);
    for book in &result.data {
        println!("  - {} by {}", book.name, book.author);
    }

    // Delete by ID
    println!("\n=== Deleting by ID: id = 5 ===");
    let id_field = schema.get_field("id").unwrap();
    let term = Term::from_field_i64(id_field, 5);
    index_writer.delete_using_term(term);
    index_writer.commit().unwrap();
    index_reader.reload().unwrap();

    // Show remaining books
    println!("\n=== Books after ID deletion ===");
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = index_reader.hybrid_search(&query).unwrap();
    println!("Total: {} books", result.total_items);
    for book in &result.data {
        println!("  - {} (id: {})", book.name, book.id);
    }
}
