use tantivy::schema::{
    DateOptions, NumericOptions, OwnedValue, STRING, SchemaBuilder, TEXT, TextOptions,
};
use tantivy::{DateTime, TantivyDocument, Term};

use crate::index_type::IndexType;

/// Trait for types that can be indexed and stored in tantivy.
pub trait IndexField: Sized {
    /// Adds a field with this type to the schema
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType);

    /// Adds this value to a document
    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field);

    /// Converts an OwnedValue back to this type
    fn from_owned_value(value: &OwnedValue) -> Option<Self>;

    /// Creates a term for this value (used for delete operations)
    fn to_term(&self, field: tantivy::schema::Field) -> Term;
}

// String implementation
impl IndexField for String {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        let mut opts: TextOptions = match index_type {
            IndexType::indexed_string | IndexType::indexed => STRING,
            IndexType::indexed_text => TEXT,
            IndexType::not_indexed => Default::default(),
        };
        if stored {
            opts = opts.set_stored();
        }
        builder.add_text_field(name, opts);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        doc.add_text(field, self);
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        match value {
            OwnedValue::Str(s) => Some(s.clone()),
            _ => None,
        }
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        Term::from_field_text(field, self)
    }
}

// i32 implementation (stored as i64)
impl IndexField for i32 {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        let mut opts: NumericOptions = match index_type {
            IndexType::indexed => tantivy::schema::INDEXED.into(),
            IndexType::not_indexed => Default::default(),
            _ => panic!("Text indexing not supported for numeric fields"),
        };
        if stored {
            opts = opts.set_stored();
        }
        builder.add_i64_field(name, opts);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        doc.add_i64(field, *self as i64);
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        match value {
            OwnedValue::I64(i) => Some(*i as i32),
            _ => None,
        }
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        Term::from_field_i64(field, *self as i64)
    }
}

// i64 implementation
impl IndexField for i64 {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        let mut opts: NumericOptions = match index_type {
            IndexType::indexed => tantivy::schema::INDEXED.into(),
            IndexType::not_indexed => Default::default(),
            _ => panic!("Text indexing not supported for numeric fields"),
        };
        if stored {
            opts = opts.set_stored();
        }
        builder.add_i64_field(name, opts);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        doc.add_i64(field, *self);
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        match value {
            OwnedValue::I64(i) => Some(*i),
            _ => None,
        }
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        Term::from_field_i64(field, *self)
    }
}

// u32 implementation (stored as u64)
impl IndexField for u32 {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        let mut opts: NumericOptions = match index_type {
            IndexType::indexed => tantivy::schema::INDEXED.into(),
            IndexType::not_indexed => Default::default(),
            _ => panic!("Text indexing not supported for numeric fields"),
        };
        if stored {
            opts = opts.set_stored();
        }
        builder.add_u64_field(name, opts);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        doc.add_u64(field, *self as u64);
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        match value {
            OwnedValue::U64(u) => Some(*u as u32),
            _ => None,
        }
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        Term::from_field_u64(field, *self as u64)
    }
}

// u64 implementation
impl IndexField for u64 {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        let mut opts: NumericOptions = match index_type {
            IndexType::indexed => tantivy::schema::INDEXED.into(),
            IndexType::not_indexed => Default::default(),
            _ => panic!("Text indexing not supported for numeric fields"),
        };
        if stored {
            opts = opts.set_stored();
        }
        builder.add_u64_field(name, opts);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        doc.add_u64(field, *self);
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        match value {
            OwnedValue::U64(u) => Some(*u),
            _ => None,
        }
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        Term::from_field_u64(field, *self)
    }
}

// f64 implementation
impl IndexField for f64 {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        let mut opts: NumericOptions = match index_type {
            IndexType::indexed => tantivy::schema::INDEXED.into(),
            IndexType::not_indexed => Default::default(),
            _ => panic!("Text indexing not supported for numeric fields"),
        };
        if stored {
            opts = opts.set_stored();
        }
        builder.add_f64_field(name, opts);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        doc.add_f64(field, *self);
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        match value {
            OwnedValue::F64(f) => Some(*f),
            _ => None,
        }
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        Term::from_field_f64(field, *self)
    }
}

// bool implementation
impl IndexField for bool {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        let mut opts: NumericOptions = match index_type {
            IndexType::indexed => tantivy::schema::INDEXED.into(),
            IndexType::not_indexed => Default::default(),
            _ => panic!("Text indexing not supported for boolean fields"),
        };
        if stored {
            opts = opts.set_stored();
        }
        builder.add_bool_field(name, opts);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        doc.add_bool(field, *self);
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        match value {
            OwnedValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        Term::from_field_bool(field, *self)
    }
}

// DateTime implementation
impl IndexField for DateTime {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        let mut opts = DateOptions::default();
        match index_type {
            IndexType::indexed => opts = opts.set_indexed(),
            IndexType::not_indexed => {}
            _ => panic!("Text indexing not supported for date fields"),
        }
        if stored {
            opts = opts.set_stored();
        }
        builder.add_date_field(name, opts);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        doc.add_date(field, *self);
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        match value {
            OwnedValue::Date(d) => Some(*d),
            _ => None,
        }
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        Term::from_field_date(field, *self)
    }
}

// Vec<T> implementation for any T: IndexField
// Note: Vec<T> only supports types that serialize to the same tantivy field type
impl<T: IndexField> IndexField for Vec<T> {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        // Vec<T> uses the same schema as T (tantivy supports multi-value fields)
        T::add_to_schema(builder, name, stored, index_type);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        for item in self {
            item.add_to_document(doc, field);
        }
    }

    fn from_owned_value(_value: &OwnedValue) -> Option<Self> {
        // Vec needs special handling - we collect all values for the field
        // This is handled separately in the macro by using get_all()
        panic!("Vec::from_owned_value should not be called directly");
    }

    fn to_term(&self, _field: tantivy::schema::Field) -> Term {
        panic!("Cannot create term from Vec - use individual items");
    }
}

// Helper trait for getting the first value from a document (for non-Vec types)
pub trait IndexFieldSingle: IndexField {
    fn from_doc_single(
        doc: &tantivy::TantivyDocument,
        field: tantivy::schema::Field,
    ) -> Option<Self>;
}

impl<T: IndexField> IndexFieldSingle for T {
    fn from_doc_single(
        doc: &tantivy::TantivyDocument,
        field: tantivy::schema::Field,
    ) -> Option<Self> {
        doc.get_first(field).and_then(|v| T::from_owned_value(&v))
    }
}

// Helper trait for Vec<T> to collect all values
pub trait IndexFieldVec<T: IndexField>: IndexField {
    fn from_doc_all(doc: &tantivy::TantivyDocument, field: tantivy::schema::Field) -> Vec<T>;
}

impl<T: IndexField> IndexFieldVec<T> for Vec<T> {
    fn from_doc_all(doc: &tantivy::TantivyDocument, field: tantivy::schema::Field) -> Vec<T> {
        doc.get_all(field)
            .filter_map(|v| T::from_owned_value(&v))
            .collect()
    }
}

// Option<T> implementation for any T: IndexField
impl<T: IndexField> IndexField for Option<T> {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        T::add_to_schema(builder, name, stored, index_type);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        if let Some(value) = self {
            value.add_to_document(doc, field);
        }
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        Some(T::from_owned_value(value))
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        match self {
            Some(value) => value.to_term(field),
            None => panic!("Cannot create term from None"),
        }
    }
}

// Uuid implementation (stored as string)
#[cfg(feature = "uuid")]
impl IndexField for uuid::Uuid {
    fn add_to_schema(builder: &mut SchemaBuilder, name: &str, stored: bool, index_type: IndexType) {
        let mut opts: TextOptions = match index_type {
            IndexType::indexed_string | IndexType::indexed => STRING,
            IndexType::indexed_text => TEXT,
            IndexType::not_indexed => Default::default(),
        };
        if stored {
            opts = opts.set_stored();
        }
        builder.add_text_field(name, opts);
    }

    fn add_to_document(&self, doc: &mut TantivyDocument, field: tantivy::schema::Field) {
        doc.add_text(field, &self.to_string());
    }

    fn from_owned_value(value: &OwnedValue) -> Option<Self> {
        match value {
            OwnedValue::Str(s) => uuid::Uuid::parse_str(s).ok(),
            _ => None,
        }
    }

    fn to_term(&self, field: tantivy::schema::Field) -> Term {
        Term::from_field_text(field, &self.to_string())
    }
}
