use book::Book;
use std::{fs, path::Path};
use text_search::{IndexWriter, Indexable, SearchQuery, tantivy::Term};

mod book;

fn main() {
    let path = "/tmp/text-search-test";
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
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let regex_search_result = index_reader.hybrid_search(&query).unwrap();
    for book in regex_search_result.data {
        println!("{:?}", book);
    }

    let field = Book::generate_schema().get_field("author").unwrap();

    let term = Term::from_field_text(field, "Steve Klabnik and Carol Nichols");
    index_writer.delete_using_term(term);
    index_writer.commit().unwrap();

    let _ = index_reader.reload();

    println!("After deleting");
    let regex_search_result = index_reader.hybrid_search(&query).unwrap();
    for book in regex_search_result.data {
        println!("{:?}", book);
    }
}
