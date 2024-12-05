use std::{
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use tantivy::{
    collector::{Count, TopDocs},
    directory::MmapDirectory,
    query::{FuzzyTermQuery, PhrasePrefixQuery, Query, QueryParser, RegexQuery},
    schema::Schema,
    DocAddress, Index, IndexWriter, ReloadPolicy, Searcher, TantivyDocument, Term,
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
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let query = QueryParser::for_index(&self.index, vec![field])
            .parse_query(query)
            .expect("Error while parsing query.");

        self._search(&query, result_count)
    }

    pub fn fuzzy_search(&self, field_name: &str, query: &str, result_count: usize) -> Vec<T> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let term: Term = Term::from_field_text(field, query);
        let query = FuzzyTermQuery::new(term, 2, true);

        self._search(&query, result_count)
    }

    pub fn regex_query(&self, field_name: &str, query: &str, result_count: usize) -> Vec<T> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let query =
            RegexQuery::from_pattern(query, field).expect("Error while building regex query.");

        self._search(&query, result_count)
    }

    pub fn prefix_query(&self, field_name: &str, query: &str, result_count: usize) -> Vec<T> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let t = Term::from_field_text(field, query);
        let query = PhrasePrefixQuery::new(vec![t]);
        self._search(&query, result_count)
    }

    fn _search(&self, query: &dyn Query, result_count: usize) -> Vec<T> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .expect("Error while constructing reader for search operation.");
        let searcher = reader.searcher();

        let top_docs = searcher
            .search(query, &TopDocs::with_limit(result_count))
            .expect("Error while performing search operation.");

        Self::docs_to_t(top_docs, &searcher)
    }

    fn docs_to_t(top_docs: Vec<(f32, DocAddress)>, searcher: &Searcher) -> Vec<T> {
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
