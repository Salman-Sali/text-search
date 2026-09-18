#[derive(Debug, Clone, Copy)]
pub struct FieldRef {
    name: &'static str,
}

impl FieldRef {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn eq(self, value: impl Into<FilterValue>) -> Filter {
        Filter::Condition {
            field_name: self.name.to_string(),
            op: FilterOp::Eq,
            value: value.into(),
        }
    }

    pub fn ne(self, value: impl Into<FilterValue>) -> Filter {
        Filter::Condition {
            field_name: self.name.to_string(),
            op: FilterOp::Ne,
            value: value.into(),
        }
    }

    pub fn gt(self, value: impl Into<FilterValue>) -> Filter {
        Filter::Condition {
            field_name: self.name.to_string(),
            op: FilterOp::Gt,
            value: value.into(),
        }
    }

    pub fn ge(self, value: impl Into<FilterValue>) -> Filter {
        Filter::Condition {
            field_name: self.name.to_string(),
            op: FilterOp::Ge,
            value: value.into(),
        }
    }

    pub fn lt(self, value: impl Into<FilterValue>) -> Filter {
        Filter::Condition {
            field_name: self.name.to_string(),
            op: FilterOp::Lt,
            value: value.into(),
        }
    }

    pub fn le(self, value: impl Into<FilterValue>) -> Filter {
        Filter::Condition {
            field_name: self.name.to_string(),
            op: FilterOp::Le,
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FilterValue {
    Str(String),
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
    Date(tantivy::DateTime),
}

impl FilterValue {
    pub fn to_term(&self, field: tantivy::schema::Field) -> tantivy::Term {
        match self {
            FilterValue::Str(s) => tantivy::Term::from_field_text(field, s),
            FilterValue::I64(i) => tantivy::Term::from_field_i64(field, *i),
            FilterValue::U64(u) => tantivy::Term::from_field_u64(field, *u),
            FilterValue::F64(f) => tantivy::Term::from_field_f64(field, *f),
            FilterValue::Bool(b) => tantivy::Term::from_field_bool(field, *b),
            FilterValue::Date(d) => tantivy::Term::from_field_date(field, *d),
        }
    }
}

impl From<&str> for FilterValue {
    fn from(value: &str) -> Self {
        FilterValue::Str(value.to_string())
    }
}

impl From<String> for FilterValue {
    fn from(value: String) -> Self {
        FilterValue::Str(value)
    }
}

impl From<i32> for FilterValue {
    fn from(value: i32) -> Self {
        FilterValue::I64(value as i64)
    }
}

impl From<i64> for FilterValue {
    fn from(value: i64) -> Self {
        FilterValue::I64(value)
    }
}

impl From<u32> for FilterValue {
    fn from(value: u32) -> Self {
        FilterValue::U64(value as u64)
    }
}

impl From<u64> for FilterValue {
    fn from(value: u64) -> Self {
        FilterValue::U64(value)
    }
}

impl From<f64> for FilterValue {
    fn from(value: f64) -> Self {
        FilterValue::F64(value)
    }
}

impl From<bool> for FilterValue {
    fn from(value: bool) -> Self {
        FilterValue::Bool(value)
    }
}

impl From<tantivy::DateTime> for FilterValue {
    fn from(value: tantivy::DateTime) -> Self {
        FilterValue::Date(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Debug, Clone)]
pub enum Filter {
    Condition {
        field_name: String,
        op: FilterOp,
        value: FilterValue,
    },
    And(Vec<Filter>),
    Or(Vec<Filter>),
}

impl Filter {
    pub fn and(self, other: Filter) -> Filter {
        match self {
            Filter::And(mut v) => {
                v.push(other);
                Filter::And(v)
            }
            _ => Filter::And(vec![self, other]),
        }
    }

    pub fn or(self, other: Filter) -> Filter {
        match self {
            Filter::Or(mut v) => {
                v.push(other);
                Filter::Or(v)
            }
            _ => Filter::Or(vec![self, other]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    field: FieldRef,
    query: String,
    filter: Option<Filter>,
    page: i64,
    per_page: i64,
}

impl SearchQuery {
    pub fn new(field: FieldRef, query: impl Into<String>) -> Self {
        Self {
            field,
            query: query.into(),
            filter: None,
            page: 1,
            per_page: 10,
        }
    }

    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn page(mut self, page: i64) -> Self {
        self.page = page;
        self
    }

    pub fn per_page(mut self, per_page: i64) -> Self {
        self.per_page = per_page;
        self
    }

    pub fn field(&self) -> &FieldRef {
        &self.field
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn filter(&self) -> Option<&Filter> {
        self.filter.as_ref()
    }

    pub fn get_page(&self) -> i64 {
        self.page
    }

    pub fn get_per_page(&self) -> i64 {
        self.per_page
    }
}
