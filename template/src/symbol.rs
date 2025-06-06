use std::fmt::{self, Display};
use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const ID: Symbol = Symbol("id");
pub const TEXT_SEARCH: Symbol = Symbol("text_search");
pub const INDEXED: Symbol = Symbol("indexed");
pub const INDEXED_TEXT: Symbol = Symbol("indexed_text");
pub const INDEXED_STRING: Symbol = Symbol("indexed_string");
pub const NOT_INDEXED: Symbol = Symbol("not_indexed");
pub const STORED: Symbol = Symbol("stored");
pub const NOT_STORED: Symbol = Symbol("not_stored");


impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl PartialEq<Symbol> for &Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl PartialEq<Symbol> for &Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl Into<String> for Symbol {
    fn into(self) -> String {
        self.0.into()
    }
}