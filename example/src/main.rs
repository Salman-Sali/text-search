use std::{fs, path::Path};

use book::Book;
use text_search::Indexer;

mod book;

fn main() {
    let path = "D:\\playground\\search_data";
    let _ = fs::remove_dir_all(&path);
    let _ = fs::create_dir(&path);
    let mut indexer = Indexer::new(Path::new(path));
    let books = Book::get_sample_books();    
    for book in &books {
        indexer.index(book.clone());        
    }
    indexer.commit();

    // let basic_search_result = indexer.search("name", "Rust", 10);
    // println!("BASIC SEARCH");
    // for book in basic_search_result {
    //     println!("{}", book.name);
    // }

    // println!("\n\nFUZZY SEARCH");
    // let fuzzy_search_result = indexer.fuzzy_search("name", "Rosty", 10);
    // for book in fuzzy_search_result {
    //     println!("{}", book.name);
    // }

    println!("\nSEARCH");
    let regex_search_result = indexer.hybrid_search("name", "Rust", 10);
    for book in regex_search_result {
        println!("{}", book.id);
    }

    indexer.delete(books.first().cloned().unwrap());
    indexer.commit();

    println!("\nSEARCH 2");
    let regex_search_result = indexer.hybrid_search("name", "Rust", 10);
    for book in regex_search_result {
        println!("{}", book.id);
    }

    indexer.delete_using_term(Book::get_term_from_id(5));
    indexer.commit();

    println!("\nSEARCH 3");
    let regex_search_result = indexer.hybrid_search("name", "Rust", 10);
    for book in regex_search_result {
        println!("{}", book.id);
    }
}


