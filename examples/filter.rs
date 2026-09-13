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

    let index_reader = index_writer.create_index_reader().unwrap();

    let filter = HashMap::from([("tags", "xyz")]);
    let regex_search_result = index_reader
        .hybrid_search(filter, "name", "Rust", 1, 10)
        .unwrap();
    for book in regex_search_result.data {
        println!("{:?}", book);
    }
}
