pub struct Book {
    pub id: u32,
    pub name: String,
    pub author: String,
    pub description: String,
    pub published_on: u16
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
            },
            Self {
                id: 2,
                name: "Rust in Action".to_string(),
                author: "Tim McNamara".to_string(),
                description: "A hands-on guide to systems programming with Rust.".to_string(),
                published_on: 2020,
            },
            Self {
                id: 3,
                name: "The Rust Programming Language".to_string(),
                author: "Steve Klabnik and Carol Nichols".to_string(),
                description: "The official book on Rust programming.".to_string(),
                published_on: 2019,
            },
            Self {
                id: 4,
                name: "Programming Rust".to_string(),
                author: "Jim Blandy and Jason Orendorff".to_string(),
                description: "Comprehensive Rust programming coverage.".to_string(),
                published_on: 2018,
            },
            Self {
                id: 5,
                name: "Rust for Rustaceans".to_string(),
                author: "Jon Gjengset".to_string(),
                description: "Intermediate to advanced concepts in Rust.".to_string(),
                published_on: 2021,
            },
        ]
    }
}