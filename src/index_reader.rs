use tantivy::{IndexReader as TanvityIndexReader, collector::Count, directory::MmapDirectory};

use std::{collections::HashMap, marker::PhantomData, path::Path};

use tantivy::{
    DocAddress, Index, ReloadPolicy, Searcher, TantivyDocument, Term,
    collector::TopDocs,
    query::{
        BooleanQuery, FuzzyTermQuery, Occur, PhrasePrefixQuery, Query, QueryParser, RegexQuery,
    },
    schema::Schema,
};
use text_search_core::Indexable;

use crate::{error::Error, paginated_result::PaginatedResult};

#[derive(Clone)]
pub struct IndexReader<T: Indexable> {
    index: Index,
    schema: Schema,
    reader: TanvityIndexReader,
    _marker: PhantomData<T>,
}

impl<T: Indexable> IndexReader<T> {
    pub fn new(path: &Path) -> Result<Self, Error> {
        if !path.exists() {
            let _ = std::fs::create_dir(path);
        }
        let dir = MmapDirectory::open(&path)?;

        let schema = T::get_struct_info().generate_schema();
        let index = Index::open_or_create(dir, schema.clone())?;
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        Ok(Self {
            reader,
            index,
            schema,
            _marker: PhantomData,
        })
    }

    pub(crate) fn from_index_writer(index: Index, schema: Schema) -> Result<Self, Error> {
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        Ok(Self {
            reader,
            index,
            schema,
            _marker: PhantomData,
        })
    }

    pub fn search(
        &self,
        filter: HashMap<&str, &str>,
        field_name: &str,
        query: &str,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResult<T>, Error> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let search_query = QueryParser::for_index(&self.index, vec![field])
            .parse_query(query)
            .expect("Error while parsing query.");

        self._search(filter, search_query, page, per_page)
    }

    pub fn fuzzy_search(
        &self,
        filter: HashMap<&str, &str>,
        field_name: &str,
        query: &str,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResult<T>, Error> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let term: Term = Term::from_field_text(field, query);
        let query = FuzzyTermQuery::new(term, 2, true);

        self._search(filter, Box::new(query), page, per_page)
    }

    pub fn regex_search(
        &self,
        filter: HashMap<&str, &str>,
        field_name: &str,
        query: &str,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResult<T>, Error> {
        let field = self
            .schema
            .get_field(field_name)
            .expect("Field with provided field name does not exsit in schema.");

        let query =
            RegexQuery::from_pattern(query, field).expect("Error while building regex query.");

        self._search(filter, Box::new(query), page, per_page)
    }

    ///Uses regex pattern matching query along with fuzzy search.
    ///Maybe slow.
    pub fn hybrid_search(
        &self,
        filter: HashMap<&str, &str>,
        field_name: &str,
        query: &str,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResult<T>, Error> {
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
        self._search(filter, Box::new(query), page, per_page)
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
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResult<T>, Error> {
        let searcher = self.reader.searcher();

        let query = self.filter_query(filter, query);

        let offset = (page - 1) * per_page;

        let top_docs = searcher
            .search(
                &query,
                &TopDocs::with_limit(per_page as usize).and_offset(offset as usize),
            )
            .expect("Error while performing search operation.");

        let total_items = searcher.search(&query, &Count)?;

        let total_pages: i64 = if total_items == 0 {
            0
        } else {
            (total_items as i64 + per_page - 1) / per_page
        };

        let data = Self::docs_to_t(top_docs, &searcher);
        return Ok(PaginatedResult::new(
            data,
            page,
            total_items as i64,
            total_pages,
        ));
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
