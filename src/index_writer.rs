use std::{fs, marker::PhantomData, path::Path};

use tantivy::{Index, IndexWriter as TantivyIndexWriter, directory::MmapDirectory};
use text_search_core::{Filter, Indexable};

use crate::{IndexReader, error::Error, index_reader::filter_to_query};

pub struct IndexWriter<T: Indexable> {
    index: Index,
    schema: tantivy::schema::Schema,
    index_writer: TantivyIndexWriter,
    _marker: PhantomData<T>,
}

impl<T: Indexable> IndexWriter<T> {
    /// Minimum memory_budget_in_bytes = 15_000_000
    pub fn new(path: &Path, memory_budget_in_bytes: usize) -> Result<Self, Error> {
        if !path.exists() {
            let _ = fs::create_dir(path);
        }

        let dir = MmapDirectory::open(&path)?;
        let schema = T::generate_schema();
        let index = Index::open_or_create(dir, schema.clone())?;

        let writer = index.writer(memory_budget_in_bytes)?;
        Ok(Self {
            index,
            schema,
            index_writer: writer,
            _marker: PhantomData,
        })
    }

    pub fn add(&self, data: T) {
        let doc = data.as_document();
        self.index_writer
            .add_document(doc)
            .expect("Error while adding document.");
    }

    pub fn delete(&self, data: T) {
        self.index_writer.delete_term(data.get_id_term());
    }

    pub fn delete_using_term(&self, term: tantivy::Term) {
        self.index_writer.delete_term(term);
    }

    pub fn delete_by_filter(&self, filter: &Filter) {
        let query = filter_to_query(filter, &self.schema, &self.index);
        let _ = self.index_writer.delete_query(query);
    }

    pub fn put(&self, data: T) {
        self.delete(data.clone());
        self.add(data);
    }

    pub fn commit(&mut self) -> Result<(), Error> {
        self.index_writer.commit()?;
        Ok(())
    }

    pub fn create_index_reader(&mut self) -> Result<IndexReader<T>, Error> {
        Ok(IndexReader::from_index_writer(
            self.index.clone(),
            self.schema.clone(),
        )?)
    }
}
