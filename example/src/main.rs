use std::path::Path;

use book::Book;
use text_search::{indexer::Indexer, Indexable};

mod book;

fn main() {
    let indexer = Indexer::new(Path::new("D:\\playground\\search_data"));
    let books = Book::get_sample_books();    
    for book in books {
        indexer.index(book);        
    }

    let basic_search_result = indexer.search("name", "Rust", 10);
    println!("BASIC SEARCH");
    for book in basic_search_result {
        println!("{}", book.name);
    }

    println!("\n\nFUZZY SEARCH");
    let fuzzy_search_result = indexer.fuzzy_search("name", "Rosty", 10);
    for book in fuzzy_search_result {
        println!("{}", book.name);
    }

    println!("\n\nRegex SEARCH");
    let regex_search_result = indexer.regex_query("name", "rustacea.*", 10);
    for book in regex_search_result {
        println!("{}", book.name);
    }
}


