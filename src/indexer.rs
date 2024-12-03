use std::{
    fs,
    path::{Path, PathBuf},
};

use template::Indexable;

pub struct Indexer {
    path: PathBuf,
}

impl Indexer {
    pub fn new(path: &Path) -> Self {
        if !path.exists() {
            let _ = fs::create_dir(path);
        }
        Self { path: path.into() }
    }

    pub fn index<T: Indexable>(&self, data: T) {
        
    }
}
