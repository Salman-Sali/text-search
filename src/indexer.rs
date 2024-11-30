pub struct  Indexer {
    path: String
}

impl Indexer {
    pub fn new(path: &str) -> Self {
        Self { path: path.into() }
    }

    
}