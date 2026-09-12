use std::{collections::HashMap, fs, path::Path};

use book::Book;
use text_search::{IndexWriter, Indexable, tantivy::Term};

mod book;

fn main() {
    let path = "/home/salman/text-search-test";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);
    let mut index_writer = IndexWriter::<Book>::new(Path::new(path), 50_000_000).unwrap();
    let books = Book::get_sample_books();
    for book in &books {
        index_writer.add(book.clone());
    }
    index_writer.commit().unwrap();

    let index_reader = index_writer.create_index_reader().unwrap();

    println!("Before deleting");
    let regex_search_result = index_reader.hybrid_search(HashMap::new(), "name", "Rust", 10);
    for book in regex_search_result {
        println!("{:?}", book);
    }

    let field = Book::get_struct_info()
        .generate_schema()
        .get_field("author")
        .unwrap();

    let term = Term::from_field_text(field, "Steve Klabnik and Carol Nichols");
    index_writer.delete_using_term(term);
    index_writer.commit().unwrap();

    println!("After deleting");
    let regex_search_result = index_reader.hybrid_search(HashMap::new(), "name", "Rust", 10);
    for book in regex_search_result {
        println!("{:?}", book);
    }
}
