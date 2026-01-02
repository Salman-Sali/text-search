use std::{collections::HashMap, fs, marker::PhantomData, path::Path};

use tantivy::{
    DocAddress, Index, IndexWriter, ReloadPolicy, Searcher, TantivyDocument, Term,
    collector::TopDocs,
    directory::MmapDirectory,
    query::{
        BooleanQuery, FuzzyTermQuery, Occur, PhrasePrefixQuery, Query, QueryParser, RegexQuery,
    },
    schema::Schema,
};
use text_search_core::Indexable;

pub struct Indexer<T: Indexable> {
    index: Index,
    schema: Schema,
    index_writer: Option<IndexWriter>,
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
            index,
            schema,
            index_writer: None,
            _marker: PhantomData,
        }
    }

    fn create_index_writer(&mut self) {
        if self.index_writer.is_none() {
            self.index_writer = Some(
                self.index
                    .writer(50_000_000)
                    .expect("Error while creating index writer."),
            );
        }
    }

    pub fn index(&mut self, data: T) {
        self.create_index_writer();

        let doc = data.as_document();
        self.index_writer
            .as_ref()
            .unwrap()
            .add_document(doc)
            .expect("Error while adding document.");
    }

    pub fn delete(&mut self, data: T) {
        self.create_index_writer();
        self.index_writer
            .as_ref()
            .unwrap()
            .delete_term(data.get_id_term());
    }

    pub fn delete_using_term(&mut self, term: tantivy::Term) {
        self.create_index_writer();
        self.index_writer.as_ref().unwrap().delete_term(term);
    }

    pub fn delete_using_filters(&mut self, filters: HashMap<&str, &str>) {
        self.create_index_writer();
        let query = BooleanQuery::from(self.new_boolean_query_filters(filters));
        let _ = self
            .index_writer
            .as_ref()
            .unwrap()
            .delete_query(Box::new(query));
    }

    pub fn update(&mut self, data: T) {
        self.delete(data.clone());
        self.index(data);
    }

    pub fn commit(&mut self) {
        if self.index_writer.is_some() {
            self.index_writer
                .as_mut()
                .unwrap()
                .commit()
                .expect("Error while commiting index data.");
        }

        self.index_writer = None;
    }

    pub fn search(
        &self,
        filter: HashMap<&str, &str>,
        field_name: &str,
        query: &str,
        result_count: usize,
    ) -> Vec<T> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let search_query = QueryParser::for_index(&self.index, vec![field])
            .parse_query(query)
            .expect("Error while parsing query.");

        self._search(filter, search_query, result_count)
    }

    pub fn fuzzy_search(
        &self,
        filter: HashMap<&str, &str>,
        field_name: &str,
        query: &str,
        result_count: usize,
    ) -> Vec<T> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let term: Term = Term::from_field_text(field, query);
        let query = FuzzyTermQuery::new(term, 2, true);

        self._search(filter, Box::new(query), result_count)
    }

    pub fn regex_search(
        &self,
        filter: HashMap<&str, &str>,
        field_name: &str,
        query: &str,
        result_count: usize,
    ) -> Vec<T> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let query =
            RegexQuery::from_pattern(query, field).expect("Error while building regex query.");

        self._search(filter, Box::new(query), result_count)
    }

    ///Uses regex pattern matching query along with fuzzy search.
    ///Maybe slow.
    pub fn hybrid_search(
        &self,
        filter: HashMap<&str, &str>,
        field_name: &str,
        query: &str,
        result_count: usize,
    ) -> Vec<T> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let terms: Vec<Term> = query
            .to_lowercase()
            .split(" ")
            .map(|term| Term::from_field_text(field, term))
            .collect();

        let fuzzy_queries: Vec<(Occur, Box<dyn Query>)> = terms
            .iter()
            .map(|term| {
                (
                    Occur::Should,
                    Box::new(FuzzyTermQuery::new(term.clone(), 2, true)) as Box<dyn Query>,
                )
            })
            .collect();

        let phrase_prefix_query: (Occur, Box<dyn Query>) = (
            Occur::Should,
            Box::new(PhrasePrefixQuery::new(terms)) as Box<dyn Query>,
        );

        let mut boolean_quries: Vec<(Occur, Box<dyn Query>)> = vec![phrase_prefix_query];
        boolean_quries.extend(fuzzy_queries);

        let query = BooleanQuery::new(boolean_quries);
        self._search(filter, Box::new(query), result_count)
    }

    fn filter_query(&self, filters: HashMap<&str, &str>, query: Box<dyn Query>) -> Box<dyn Query> {
        let filter_query = if filters.is_empty() {
            None
        } else {
            Some(self.new_boolean_query_filters(filters))
        };

        match filter_query {
            Some(mut x) => {
                x.push((Occur::Must, Box::new(query)));
                Box::new(BooleanQuery::from(x))
            }
            None => query,
        }
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

    fn _search(
        &self,
        filter: HashMap<&str, &str>,
        query: Box<dyn Query>,
        result_count: usize,
    ) -> Vec<T> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .expect("Error while constructing reader for search operation.");
        let searcher = reader.searcher();

        let query = self.filter_query(filter, query);

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(result_count))
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
