use std::{collections::HashMap, fs, marker::PhantomData, path::Path};

use tantivy::{
    Index, IndexWriter as TantivyIndexWriter,
    directory::MmapDirectory,
    query::{BooleanQuery, Occur, Query, QueryParser},
    schema::Schema,
};
use text_search_core::Indexable;

use crate::{IndexReader, error::Error};

pub struct IndexWriter<T: Indexable> {
    index: Index,
    schema: Schema,
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
        let schema = T::get_struct_info().generate_schema();
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

    fn new_boolean_query_filters(
        &self,
        filters: HashMap<&str, &str>,
    ) -> Vec<(Occur, Box<dyn Query>)> {
        filters
            .iter()
            .map(|x| {
                let field = self.schema.get_field(x.0).expect(&format!(
                    "Field with provided field name `{}` does not exists in schema.",
                    x.0
                ));
                let phrase = format!("\"{}\"", x.1);

                let filter_query = QueryParser::for_index(&self.index, vec![field])
                    .parse_query(&phrase)
                    .expect("Error while parsing query.");
                (Occur::Must, filter_query)
            })
            .collect()
    }

    pub fn delete_using_filters(&self, filters: HashMap<&str, &str>) {
        let query = BooleanQuery::from(self.new_boolean_query_filters(filters));
        let _ = self.index_writer.delete_query(Box::new(query));
    }

    pub fn put(&self, data: T) {
        self.delete(data.clone());
        self.add(data);
    }

    pub fn commit(&mut self) -> Result<(), Error> {
        self.index_writer.commit()?;
        return Ok(());
    }

    pub fn create_index_reader(&mut self) -> Result<IndexReader<T>, Error> {
        Ok(IndexReader::from_index_writer(
            self.index.clone(),
            self.schema.clone(),
        )?)
    }
}
