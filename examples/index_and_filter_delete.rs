use std::{collections::HashMap, fs, path::Path};

use book::Book;
use text_search::IndexWriter;

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

    println!("Before deleting");

    let index_reader = index_writer.create_index_reader().unwrap();

    let regex_search_result = index_reader
        .hybrid_search(HashMap::new(), "name", "Rust", 1, 10)
        .unwrap();
    for book in regex_search_result.data {
        println!("{:?}", book);
    }

    index_writer.delete_using_filters(HashMap::from([
        ("author", "Steve Klabnik and Carol Nichols"),
        ("name", "The Rust Programming Language"),
    ]));
    index_writer.commit().unwrap();

    println!("After deleting");
    let regex_search_result = index_reader
        .hybrid_search(HashMap::new(), "name", "Rust", 1, 10)
        .unwrap();
    for book in regex_search_result.data {
        println!("{:?}", book);
    }
}
