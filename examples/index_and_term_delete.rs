use std::{collections::HashMap, fs, path::Path};

use book::Book;
use text_search::{tantivy::Term, Indexable, Indexer};

mod book;

fn main() {
    let path = "/home/salman/text-search-test";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);
    let mut indexer = Indexer::<Book>::new(Path::new(path));
    let books = Book::get_sample_books();
    for book in &books {
        indexer.index(book.clone());
    }
    indexer.commit();

    println!("Before deleting");
    let regex_search_result = indexer.hybrid_search(HashMap::new(), "name", "Rust", 10);
    for book in regex_search_result {
        println!("{:?}", book);
    }

    let field = Book::get_struct_info()
        .generate_schema()
        .get_field("author")
        .unwrap();

    let term = Term::from_field_text(field, "Steve Klabnik and Carol Nichols");
    indexer.delete_using_term(term);
    indexer.commit();

    println!("After deleting");
    let regex_search_result = indexer.hybrid_search(HashMap::new(), "name", "Rust", 10);
    for book in regex_search_result {
        println!("{:?}", book);
    }
}