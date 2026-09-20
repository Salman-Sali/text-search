use text_search::Indexed;

#[derive(Indexed, Clone, Debug)]
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
    #[text_search(indexed_string, stored)]
    pub isbn: Option<String>,
}

impl Book {
    #[allow(unused)]
    pub fn get_sample_books() -> Vec<Self> {
        vec![
            Self {
                id: 1,
                name: "Let's Get Rusty Vol 1".to_string(),
                author: "Bogdan".to_string(),
                description: "An introduction to Rust programming.".to_string(),
                published_on: 2021,
                tags: vec!["abc".into(), "xyz".into(), "123".into(), "234".into()],
                isbn: Some("978-1234567890".to_string()),
            },
            Self {
                id: 2,
                name: "Let's Get Rusty Vol 2".to_string(),
                author: "Bogdan".to_string(),
                description: "An introduction to Rust programming.".to_string(),
                published_on: 2021,
                tags: vec!["abc".into()],
                isbn: Some("978-0987654321".to_string()),
            },
            Self {
                id: 3,
                name: "The Iron Path".to_string(),
                author: "Bogdan".to_string(),
                description: "An introduction to Rust programming.".to_string(),
                published_on: 2021,
                tags: vec!["xyz".into()],
                isbn: None,
            },
            Self {
                id: 4,
                name: "Fearless Concurrency".to_string(),
                author: "Bogdan".to_string(),
                description: "An introduction to Rust programming.".to_string(),
                published_on: 2021,
                tags: vec!["abc".into()],
                isbn: None,
            },
            Self {
                id: 5,
                name: "Rust in Action".to_string(),
                author: "Tim McNamara".to_string(),
                description: "A hands-on guide to systems programming with Rust.".to_string(),
                published_on: 2020,
                tags: vec!["abc".into()],
                isbn: Some("978-1617294556".to_string()),
            },
            Self {
                id: 6,
                name: "The Rust Programming Language".to_string(),
                author: "Steve Klabnik and Carol Nichols".to_string(),
                description: "The official book on Rust programming.".to_string(),
                published_on: 2019,
                tags: vec!["abc".into()],
                isbn: Some("978-1718500440".to_string()),
            },
            Self {
                id: 7,
                name: "Programming Rust".to_string(),
                author: "Jim Blandy and Jason Orendorff".to_string(),
                description: "Comprehensive Rust programming coverage.".to_string(),
                published_on: 2018,
                tags: vec!["abc".into()],
                isbn: Some("978-1491927281".to_string()),
            },
            Self {
                id: 8,
                name: "Rust for Rustaceans".to_string(),
                author: "Jon Gjengset".to_string(),
                description: "Intermediate to advanced concepts in Rust.".to_string(),
                published_on: 2021,
                tags: vec!["xyz".into()],
                isbn: Some("978-1718501850".to_string()),
            },
        ]
    }
}
