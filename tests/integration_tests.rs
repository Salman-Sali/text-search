use text_search::{FieldRef, Filter, FilterOp, FilterValue, IndexWriter, SearchQuery};

/// Test Book struct for integration tests
#[derive(text_search::Indexed, Clone, Debug, PartialEq)]
pub struct Book {
    #[text_search(id)]
    pub id: i32,
    #[text_search(indexed_text, stored)]
    pub name: String,
    #[text_search(indexed_string, stored)]
    pub author: String,
    #[text_search(indexed_text, stored)]
    pub description: String,
    #[text_search(indexed, stored)]
    pub published_on: i32,
    #[text_search(indexed_string, stored)]
    pub tags: Vec<String>,
}

impl Book {
    pub fn get_sample_books() -> Vec<Self> {
        vec![
            Self {
                id: 1,
                name: "Let's Get Rusty Vol 1".to_string(),
                author: "Bogdan".to_string(),
                description: "An introduction to Rust programming.".to_string(),
                published_on: 2021,
                tags: vec!["abc".into(), "xyz".into(), "123".into(), "234".into()],
            },
            Self {
                id: 2,
                name: "Let's Get Rusty Vol 2".to_string(),
                author: "Bogdan".to_string(),
                description: "An introduction to Rust programming.".to_string(),
                published_on: 2021,
                tags: vec!["abc".into()],
            },
            Self {
                id: 3,
                name: "The Iron Path".to_string(),
                author: "Bogdan".to_string(),
                description: "An introduction to Rust programming.".to_string(),
                published_on: 2021,
                tags: vec!["xyz".into()],
            },
            Self {
                id: 4,
                name: "Fearless Concurrency".to_string(),
                author: "Bogdan".to_string(),
                description: "An introduction to Rust programming.".to_string(),
                published_on: 2021,
                tags: vec!["abc".into()],
            },
            Self {
                id: 5,
                name: "Rust in Action".to_string(),
                author: "Tim McNamara".to_string(),
                description: "A hands-on guide to systems programming with Rust.".to_string(),
                published_on: 2020,
                tags: vec!["abc".into()],
            },
            Self {
                id: 6,
                name: "The Rust Programming Language".to_string(),
                author: "Steve Klabnik and Carol Nichols".to_string(),
                description: "The official book on Rust programming.".to_string(),
                published_on: 2019,
                tags: vec!["abc".into()],
            },
            Self {
                id: 7,
                name: "Programming Rust".to_string(),
                author: "Jim Blandy and Jason Orendorff".to_string(),
                description: "Comprehensive Rust programming coverage.".to_string(),
                published_on: 2018,
                tags: vec!["abc".into()],
            },
            Self {
                id: 8,
                name: "Rust for Rustaceans".to_string(),
                author: "Jon Gjengset".to_string(),
                description: "Intermediate to advanced concepts in Rust.".to_string(),
                published_on: 2021,
                tags: vec!["xyz".into()],
            },
        ]
    }
}

fn setup_index() -> (
    tempfile::TempDir,
    IndexWriter<Book>,
    text_search::IndexReader<Book>,
) {
    let dir = tempfile::tempdir().unwrap();
    let mut writer = IndexWriter::<Book>::new(dir.path(), 50_000_000).unwrap();
    for book in Book::get_sample_books() {
        writer.add(book);
    }
    writer.commit().unwrap();
    let reader = writer.create_index_reader().unwrap();
    (dir, writer, reader)
}

#[test]
fn test_search_query_builder() {
    let field = FieldRef::new("test_field");
    let query = SearchQuery::new(field, "test query")
        .with_filter(Book::author.eq("Test Author"))
        .page(2)
        .per_page(5);

    assert_eq!(query.field().name(), "test_field");
    assert_eq!(query.query(), "test query");
    assert_eq!(query.get_page(), 2);
    assert_eq!(query.get_per_page(), 5);
    assert!(query.filter().is_some());
}

#[test]
fn test_field_ref_eq_filter() {
    let filter = Book::author.eq("Bogdan");

    match filter {
        Filter::Condition {
            field_name,
            op,
            value,
        } => {
            assert_eq!(field_name, "author");
            assert_eq!(op, FilterOp::Eq);
            match value {
                FilterValue::Str(s) => assert_eq!(s, "Bogdan"),
                _ => panic!("Expected Str filter value"),
            }
        }
        _ => panic!("Expected Condition filter"),
    }
}

#[test]
fn test_filter_and_combinator() {
    let filter = Book::author.eq("Bogdan").and(Book::tags.eq("xyz"));

    match filter {
        Filter::And(filters) => {
            assert_eq!(filters.len(), 2);
            // Check first condition
            match &filters[0] {
                Filter::Condition {
                    field_name,
                    op,
                    value,
                } => {
                    assert_eq!(field_name, "author");
                    assert_eq!(*op, FilterOp::Eq);
                    match value {
                        FilterValue::Str(s) => assert_eq!(s, "Bogdan"),
                        _ => panic!("Expected Str filter value"),
                    }
                }
                _ => panic!("Expected Condition filter"),
            }
            // Check second condition
            match &filters[1] {
                Filter::Condition {
                    field_name,
                    op,
                    value,
                } => {
                    assert_eq!(field_name, "tags");
                    assert_eq!(*op, FilterOp::Eq);
                    match value {
                        FilterValue::Str(s) => assert_eq!(s, "xyz"),
                        _ => panic!("Expected Str filter value"),
                    }
                }
                _ => panic!("Expected Condition filter"),
            }
        }
        _ => panic!("Expected And filter"),
    }
}

#[test]
fn test_filter_or_combinator() {
    let filter = Book::author
        .eq("Bogdan")
        .or(Book::author.eq("Tim McNamara"));

    match filter {
        Filter::Or(filters) => {
            assert_eq!(filters.len(), 2);
        }
        _ => panic!("Expected Or filter"),
    }
}

#[test]
fn test_search_no_filter() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = reader.search(&query).unwrap();

    // Should find some books with "Rust" in the title
    assert!(result.total_items > 0);
    assert!(!result.data.is_empty());
}

#[test]
fn test_search_with_eq_filter() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("Bogdan"))
        .page(1)
        .per_page(10);
    let result = reader.hybrid_search(&query).unwrap();

    // All results should be by Bogdan
    assert!(!result.data.is_empty());
    for book in &result.data {
        assert_eq!(book.author, "Bogdan");
    }
}

#[test]
fn test_search_with_and_filter() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.eq("Bogdan").and(Book::tags.eq("xyz")))
        .page(1)
        .per_page(10);
    let result = reader.hybrid_search(&query).unwrap();

    // Should find books by Bogdan with xyz tag
    assert!(!result.data.is_empty());
    for book in &result.data {
        assert_eq!(book.author, "Bogdan");
        assert!(book.tags.contains(&"xyz".to_string()));
    }
}

#[test]
fn test_fuzzy_search() {
    let (_dir, _writer, reader) = setup_index();

    // Search for misspelled term "Rsut" instead of "Rust"
    let query = SearchQuery::new(Book::name, "Rsut").page(1).per_page(10);
    let result = reader.fuzzy_search(&query).unwrap();

    // Should still find results due to fuzzy matching
    assert!(result.total_items > 0);
}

#[test]
fn test_regex_search() {
    let (_dir, _writer, reader) = setup_index();

    // Search with regex pattern - matches any term starting with 'rust'
    // Note: regex queries work on individual tokens, so "rust.*" matches "rust" and "rusty"
    let query = SearchQuery::new(Book::name, "rust.*").page(1).per_page(10);
    let result = reader.regex_search(&query).unwrap();

    // Should find books with "Rust" or "Rusty" in the name (books 1, 2, 5, 6, 7, 8)
    assert!(result.total_items >= 4);
}

#[test]
fn test_hybrid_search() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result = reader.hybrid_search(&query).unwrap();

    // Should find results
    assert!(result.total_items > 0);
    assert!(!result.data.is_empty());
}

#[test]
fn test_pagination() {
    let (_dir, _writer, reader) = setup_index();

    // Search with per_page=3, page=1
    // Note: "Rust" query matches books with "Rust" token in name (books 5, 6, 7, 8)
    // Books 1-2 have "Rusty" (different token), books 3-4 don't contain "Rust"
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(3);
    let result = reader.search(&query).unwrap();

    assert_eq!(result.data.len(), 3);
    assert_eq!(result.page, 1);
    assert_eq!(result.total_items, 4); // Books with "Rust" token: 5, 6, 7, 8
    assert_eq!(result.total_pages, 2); // ceil(4/3) = 2

    // Search with per_page=3, page=2
    let query = SearchQuery::new(Book::name, "Rust").page(2).per_page(3);
    let result = reader.search(&query).unwrap();

    assert_eq!(result.data.len(), 1); // Last page has 1 item (4 total, 3 per page)
    assert_eq!(result.page, 2);
}

#[test]
fn test_delete_by_filter() {
    let (_dir, mut writer, reader) = setup_index();

    // Count all books before deletion
    let query = SearchQuery::new(Book::name, "Rust").page(1).per_page(10);
    let result_before = reader.hybrid_search(&query).unwrap();
    let count_before = result_before.total_items;

    // Delete books by a specific author
    writer.delete_by_filter(&Book::author.eq("Bogdan"));
    writer.commit().unwrap();

    // Reload the reader to see the changes
    let _ = reader.reload();

    // Count all books after deletion
    let result_after = reader.hybrid_search(&query).unwrap();
    let count_after = result_after.total_items;

    // Should have fewer books now
    assert!(count_after < count_before);
    assert!(!result_after.data.iter().any(|b| b.author == "Bogdan"));
}

#[test]
fn test_numeric_gt_filter() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.gt(2019))
        .page(1)
        .per_page(10);
    let result = reader.search(&query).unwrap();

    // All results should have published_on > 2019
    for book in &result.data {
        assert!(book.published_on > 2019);
    }
}

#[test]
fn test_numeric_ge_filter() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.ge(2020))
        .page(1)
        .per_page(10);
    let result = reader.search(&query).unwrap();

    // All results should have published_on >= 2020
    for book in &result.data {
        assert!(book.published_on >= 2020);
    }
}

#[test]
fn test_numeric_lt_filter() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.lt(2020))
        .page(1)
        .per_page(10);
    let result = reader.search(&query).unwrap();

    // All results should have published_on < 2020
    for book in &result.data {
        assert!(book.published_on < 2020);
    }
}

#[test]
fn test_numeric_le_filter() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::published_on.le(2020))
        .page(1)
        .per_page(10);
    let result = reader.search(&query).unwrap();

    // All results should have published_on <= 2020
    for book in &result.data {
        assert!(book.published_on <= 2020);
    }
}

#[test]
fn test_chained_filters() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(
            Book::author
                .eq("Bogdan")
                .and(Book::tags.eq("xyz"))
                .and(Book::published_on.ge(2020)),
        )
        .page(1)
        .per_page(10);
    let result = reader.search(&query).unwrap();

    // All results should match all conditions
    for book in &result.data {
        assert_eq!(book.author, "Bogdan");
        assert!(book.tags.contains(&"xyz".to_string()));
        assert!(book.published_on >= 2020);
    }
}

#[test]
fn test_filter_ne() {
    let (_dir, _writer, reader) = setup_index();

    let query = SearchQuery::new(Book::name, "Rust")
        .with_filter(Book::author.ne("Bogdan"))
        .page(1)
        .per_page(10);
    let result = reader.search(&query).unwrap();

    // All results should NOT be Bogdan
    assert!(!result.data.is_empty());
    for book in &result.data {
        assert_ne!(book.author, "Bogdan");
    }
}
