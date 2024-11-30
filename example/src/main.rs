use book::Book;
use text_search::{indexer::Indexer, Indexable};

mod book;

fn main() {
    let indexer = Indexer::new("D:\\playground\\search_data");
    let books = Book::get_sample_books();
    
    for book in books {
        let a = book.get_field_configs();
        let b = 1;
        //indexer.index(book);        
    }
}
