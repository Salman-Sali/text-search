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
    let result = indexer.search("name", "Let's Get Rusty Vol 1", 10);
    for ele in result {
        print!("{}", ele.name);
    }
}


