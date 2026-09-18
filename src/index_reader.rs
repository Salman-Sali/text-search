use tantivy::{IndexReader as TanvityIndexReader, collector::Count, directory::MmapDirectory};

use std::{marker::PhantomData, ops::Bound, path::Path};

use tantivy::{
    DocAddress, Index, ReloadPolicy, Searcher, TantivyDocument,
    collector::TopDocs,
    query::{
        AllQuery, BooleanQuery, FuzzyTermQuery, Occur, PhrasePrefixQuery, Query, QueryParser,
        RangeQuery, RegexQuery, TermQuery,
    },
    schema::{FieldType, Schema},
};
use text_search_core::{Filter, FilterOp, FilterValue, Indexable, SearchQuery};

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

        let schema = T::generate_schema();
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

    pub fn reload(&self) -> Result<(), Error> {
        Ok(self.reader.reload()?)
    }

    pub fn search(&self, query: &SearchQuery) -> Result<PaginatedResult<T>, Error> {
        let field = self
            .schema
            .get_field(query.field().name())
            .expect("Field with provided field name does not exist in schema.");

        let search_query = QueryParser::for_index(&self.index, vec![field])
            .parse_query(query.query())
            .expect("Error while parsing query.");

        self._search(
            query.filter(),
            search_query,
            query.get_page(),
            query.get_per_page(),
        )
    }

    pub fn fuzzy_search(&self, query: &SearchQuery) -> Result<PaginatedResult<T>, Error> {
        let field = self
            .schema
            .get_field(query.field().name())
            .expect("Field with provided field name does not exist in schema.");

        let term: tantivy::Term = tantivy::Term::from_field_text(field, query.query());
        let search_query = FuzzyTermQuery::new(term, 2, true);

        self._search(
            query.filter(),
            Box::new(search_query),
            query.get_page(),
            query.get_per_page(),
        )
    }

    pub fn regex_search(&self, query: &SearchQuery) -> Result<PaginatedResult<T>, Error> {
        let field = self
            .schema
            .get_field(query.field().name())
            .expect("Field with provided field name does not exist in schema.");

        let search_query = RegexQuery::from_pattern(query.query(), field)
            .expect("Error while building regex query.");

        self._search(
            query.filter(),
            Box::new(search_query),
            query.get_page(),
            query.get_per_page(),
        )
    }

    ///Uses regex pattern matching query along with fuzzy search.
    ///Maybe slow.
    pub fn hybrid_search(&self, query: &SearchQuery) -> Result<PaginatedResult<T>, Error> {
        let field = self
            .schema
            .get_field(query.field().name())
            .expect("Field with provided field name does not exist in schema.");

        let terms: Vec<tantivy::Term> = query
            .query()
            .to_lowercase()
            .split(' ')
            .map(|term| tantivy::Term::from_field_text(field, term))
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

        let search_query = BooleanQuery::new(boolean_quries);
        self._search(
            query.filter(),
            Box::new(search_query),
            query.get_page(),
            query.get_per_page(),
        )
    }

    fn apply_filter(
        &self,
        filter: Option<&Filter>,
        search_query: Box<dyn Query>,
    ) -> Box<dyn Query> {
        let filter_query = filter.map(|f| self.filter_to_query(f));

        match filter_query {
            Some(mut queries) => {
                queries.push((Occur::Must, search_query));
                Box::new(BooleanQuery::from(queries))
            }
            None => search_query,
        }
    }

    fn filter_to_query(&self, filter: &Filter) -> Vec<(Occur, Box<dyn Query>)> {
        match filter {
            Filter::Condition {
                field_name,
                op,
                value,
            } => {
                let field = self.schema.get_field(field_name).expect(&format!(
                    "Field with provided field name `{}` does not exist in schema.",
                    field_name
                ));

                // Get the field type from schema to determine the correct query type
                let field_entry = self.schema.get_field_entry(field);
                let tantivy_field_type = field_entry.field_type();

                match op {
                    FilterOp::Eq => {
                        match value {
                            FilterValue::Str(s) => {
                                // For strings, use QueryParser with quoted string
                                let phrase = format!("\"{}\"", s);
                                let query = QueryParser::for_index(&self.index, vec![field])
                                    .parse_query(&phrase)
                                    .expect("Error while parsing query.");
                                vec![(Occur::Must, query)]
                            }
                            _ => {
                                // For non-string values, use TermQuery
                                let term = value.to_term(field);
                                let query =
                                    TermQuery::new(term, tantivy::schema::IndexRecordOption::Basic);
                                vec![(Occur::Must, Box::new(query))]
                            }
                        }
                    }
                    FilterOp::Ne => {
                        let eq_query = self.filter_to_query(&Filter::Condition {
                            field_name: field_name.clone(),
                            op: FilterOp::Eq,
                            value: value.clone(),
                        });
                        // For Ne, we need to use BooleanQuery with MustNot
                        // Since filter_to_query returns a Vec, we take the first query
                        let eq_query_boxed = eq_query
                            .into_iter()
                            .next()
                            .map(|(_, q)| q)
                            .unwrap_or_else(|| Box::new(AllQuery));
                        vec![
                            (Occur::MustNot, eq_query_boxed),
                            (Occur::Must, Box::new(AllQuery)),
                        ]
                    }
                    FilterOp::Ge => {
                        let term = value.to_term(field);
                        let value_type = get_field_value_type(tantivy_field_type);
                        let query = RangeQuery::new_term_bounds(
                            field_name.clone(),
                            value_type,
                            &Bound::Included(term),
                            &Bound::Unbounded,
                        );
                        vec![(Occur::Must, Box::new(query))]
                    }
                    FilterOp::Gt => {
                        let term = value.to_term(field);
                        let value_type = get_field_value_type(tantivy_field_type);
                        let query = RangeQuery::new_term_bounds(
                            field_name.clone(),
                            value_type,
                            &Bound::Excluded(term),
                            &Bound::Unbounded,
                        );
                        vec![(Occur::Must, Box::new(query))]
                    }
                    FilterOp::Le => {
                        let term = value.to_term(field);
                        let value_type = get_field_value_type(tantivy_field_type);
                        let query = RangeQuery::new_term_bounds(
                            field_name.clone(),
                            value_type,
                            &Bound::Unbounded,
                            &Bound::Included(term),
                        );
                        vec![(Occur::Must, Box::new(query))]
                    }
                    FilterOp::Lt => {
                        let term = value.to_term(field);
                        let value_type = get_field_value_type(tantivy_field_type);
                        let query = RangeQuery::new_term_bounds(
                            field_name.clone(),
                            value_type,
                            &Bound::Unbounded,
                            &Bound::Excluded(term),
                        );
                        vec![(Occur::Must, Box::new(query))]
                    }
                }
            }
            Filter::And(filters) => {
                let mut all_queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
                for f in filters {
                    let sub_queries = self.filter_to_query(f);
                    for (_, q) in sub_queries {
                        all_queries.push((Occur::Must, q));
                    }
                }
                all_queries
            }
            Filter::Or(filters) => {
                let mut all_queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
                for f in filters {
                    let sub_queries = self.filter_to_query(f);
                    for (_, q) in sub_queries {
                        all_queries.push((Occur::Should, q));
                    }
                }
                all_queries
            }
        }
    }

    fn _search(
        &self,
        filter: Option<&Filter>,
        query: Box<dyn Query>,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResult<T>, Error> {
        let searcher = self.reader.searcher();

        let query = self.apply_filter(filter, query);

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
        Ok(PaginatedResult::new(
            data,
            page,
            total_items as i64,
            total_pages,
        ))
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

/// Get the value type for a field from its FieldType
fn get_field_value_type(field_type: &FieldType) -> tantivy::schema::Type {
    use tantivy::schema::Type;

    match field_type {
        FieldType::Str(_) => Type::Str,
        FieldType::U64(_) => Type::U64,
        FieldType::I64(_) => Type::I64,
        FieldType::F64(_) => Type::F64,
        FieldType::Bool(_) => Type::Bool,
        FieldType::Date(_) => Type::Date,
        _ => Type::Str, // Default to string for unknown types
    }
}

/// Convert a Filter to a tantivy query. This is a helper function that can be used
/// by both IndexReader and IndexWriter.
pub(crate) fn filter_to_query(filter: &Filter, schema: &Schema, index: &Index) -> Box<dyn Query> {
    match filter {
        Filter::Condition {
            field_name,
            op,
            value,
        } => {
            let field = schema.get_field(field_name).expect(&format!(
                "Field with provided field name `{}` does not exist in schema.",
                field_name
            ));

            // Get the field type from schema to determine the correct query type
            let field_entry = schema.get_field_entry(field);
            let tantivy_field_type = field_entry.field_type();
            let value_type = get_field_value_type(tantivy_field_type);

            match op {
                FilterOp::Eq => {
                    match value {
                        FilterValue::Str(s) => {
                            // For strings, use QueryParser with quoted string
                            let phrase = format!("\"{}\"", s);
                            let query = QueryParser::for_index(index, vec![field])
                                .parse_query(&phrase)
                                .expect("Error while parsing query.");
                            query
                        }
                        _ => {
                            // For non-string values, use TermQuery
                            let term = value.to_term(field);
                            Box::new(TermQuery::new(
                                term,
                                tantivy::schema::IndexRecordOption::Basic,
                            ))
                        }
                    }
                }
                FilterOp::Ne => {
                    // Get the eq query and wrap in BooleanQuery with MustNot
                    let eq_query = filter_to_query(
                        &Filter::Condition {
                            field_name: field_name.clone(),
                            op: FilterOp::Eq,
                            value: value.clone(),
                        },
                        schema,
                        index,
                    );
                    Box::new(BooleanQuery::new(vec![
                        (Occur::MustNot, eq_query),
                        (Occur::Must, Box::new(AllQuery)),
                    ]))
                }
                FilterOp::Ge => {
                    let term = value.to_term(field);
                    Box::new(RangeQuery::new_term_bounds(
                        field_name.clone(),
                        value_type,
                        &Bound::Included(term),
                        &Bound::Unbounded,
                    ))
                }
                FilterOp::Gt => {
                    let term = value.to_term(field);
                    Box::new(RangeQuery::new_term_bounds(
                        field_name.clone(),
                        value_type,
                        &Bound::Excluded(term),
                        &Bound::Unbounded,
                    ))
                }
                FilterOp::Le => {
                    let term = value.to_term(field);
                    Box::new(RangeQuery::new_term_bounds(
                        field_name.clone(),
                        value_type,
                        &Bound::Unbounded,
                        &Bound::Included(term),
                    ))
                }
                FilterOp::Lt => {
                    let term = value.to_term(field);
                    Box::new(RangeQuery::new_term_bounds(
                        field_name.clone(),
                        value_type,
                        &Bound::Unbounded,
                        &Bound::Excluded(term),
                    ))
                }
            }
        }
        Filter::And(filters) => {
            let mut queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
            for f in filters {
                queries.push((Occur::Must, filter_to_query(f, schema, index)));
            }
            Box::new(BooleanQuery::new(queries))
        }
        Filter::Or(filters) => {
            let mut queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
            for f in filters {
                queries.push((Occur::Should, filter_to_query(f, schema, index)));
            }
            Box::new(BooleanQuery::new(queries))
        }
    }
}
