use std::{
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use tantivy::{
    collector::TopDocs, directory::MmapDirectory, query::QueryParser, schema::Schema, Index,
    IndexWriter, TantivyDocument,
};
use template::Indexable;

pub struct Indexer<T: Indexable> {
    path: PathBuf,
    index: Index,
    schema: Schema,
    _marker: PhantomData<T>,
}

impl<T: Indexable> Indexer<T> {
    pub fn new(path: &Path) -> Self {
        if !path.exists() {
            let _ = fs::create_dir(path);
        }

        let dir = MmapDirectory::open(&path).expect("Error while opening directory");
        let schema = T::get_struct_info().generate_schema();
        let index = Index::open_or_create(dir, schema.clone())
            .expect("Error while opening or creating index. If schema has been updated, remove the old data.");

        Self {
            path: path.into(),
            index,
            schema,
            _marker: PhantomData,
        }
    }

    pub fn index(&self, data: T) {
        let doc = data.as_document();
        let mut index_writer: IndexWriter = self
            .index
            .writer(50_000_000)
            .expect("Error while creating index writer.");

        index_writer
            .add_document(doc)
            .expect("Error while adding document.");
        index_writer
            .commit()
            .expect("Error while commiting data to index.");
    }

    pub fn search(&self, field_name: &str, query: &str, result_count: usize) -> Vec<T> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(tantivy::ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .expect("Error while constructing reader for search operation.");

        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let searcher = reader.searcher();
        let _query = QueryParser::for_index(&self.index, vec![field])
            .parse_query(query)
            .expect("Error while parsing query.");

        let top_docs = searcher
            .search(&_query, &TopDocs::with_limit(result_count))
            .expect("Error while performing search operation.");

        let mut result: Vec<T> = vec![];
        for (_score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher
                .doc(doc_address)
                .expect("Error while trying to find search document.");
            result.push(T::from_doc(doc));
        }
        result
    }
}
