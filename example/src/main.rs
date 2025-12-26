use std::{collections::HashMap, fs, path::Path};

use book::Book;
use text_search::Indexer;

mod book;

fn main() {
    let path = "/home/salman/text-search-test";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);
    let mut indexer = Indexer::new(Path::new(path));
    let books = Book::get_sample_books();
    for book in &books {
        indexer.index(book.clone());
    }
    indexer.commit();

    println!("Filtered Search");
    let regex_search_result =
        indexer.fuzzy_search(HashMap::from([("author", "Bogdan")]), "name", "Rust", 10);
    for book in regex_search_result {
        println!("{}", book.id);
    }

    println!("Before deleting");
    let regex_search_result = indexer.hybrid_search(HashMap::new(), "name", "Rust", 10);
    for book in regex_search_result {
        println!("{}", book.id);
    }

    indexer.delete(books.first().cloned().unwrap());
    indexer.commit();

    println!("After deleting");
    let regex_search_result = indexer.hybrid_search(HashMap::new(), "name", "Rust", 10);
    for book in regex_search_result {
        println!("{}", book.id);
    }
}
