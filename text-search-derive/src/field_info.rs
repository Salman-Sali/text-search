use crate::context::Ctxt;

use syn::{Field, Type};
use text_search_core::{FieldInfo, IndexType, symbol::*};

/// Extended field info that includes the parsed type for code generation
pub struct DeriveFieldInfo {
    pub info: FieldInfo,
    pub type_path: syn::TypePath,
}

/// Extract the inner type T from Vec<T>
fn extract_vec_inner_type(type_path: &syn::TypePath) -> Option<syn::TypePath> {
    let segment = type_path.path.segments.last()?;
    if segment.ident != "Vec" {
        return None;
    }

    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(Type::Path(inner))) = args.args.first() {
            return Some(inner.clone());
        }
    }
    None
}

pub fn get_field_info(ctxt: &Ctxt, field: &Field) -> DeriveFieldInfo {
    let mut is_id: bool = false;
    let mut index_type: Option<IndexType> = None;
    let mut stored: Option<bool> = None;

    // Extract the type path
    let type_path = if let Type::Path(tp) = &field.ty {
        tp.clone()
    } else {
        // Create a placeholder for unsupported types
        syn::TypePath {
            qself: None,
            path: syn::parse_str("std::marker::PhantomData<u8>").unwrap(),
        }
    };

    for attr in &field.attrs {
        if attr.path() != TEXT_SEARCH {
            continue;
        }

        if let syn::Meta::List(meta) = &attr.meta {
            if meta.tokens.is_empty() {
                continue;
            }
        }

        if let Err(err) = attr.parse_nested_meta(|meta| {
            if meta.path == ID {
                is_id = true;
            }

            let _index_type = if meta.path == INDEXED_STRING {
                Some(IndexType::indexed_string)
            } else if meta.path == INDEXED_TEXT {
                Some(IndexType::indexed_text)
            } else if meta.path == NOT_INDEXED {
                Some(IndexType::not_indexed)
            } else {
                None
            };

            let _stored = if meta.path == STORED {
                Some(true)
            } else if meta.path == NOT_STORED {
                Some(false)
            } else {
                None
            };

            if index_type.is_some() && _index_type.is_some() {
                panic!(
                    "Cannot have {:?} and {:?} together",
                    index_type.clone().unwrap(),
                    _index_type.unwrap()
                );
            } else if index_type.is_none() {
                index_type = _index_type;
            }

            if stored.is_some() && _stored.is_some() {
                panic!(
                    "Cannot have {:?} and {:?} together",
                    stored.clone().unwrap(),
                    _stored.unwrap()
                );
            } else if stored.is_none() {
                stored = _stored;
            }

            if is_id && (index_type.is_some() || stored.is_some()) {
                panic!("Cannot have other attributes when field has id attribute.")
            }

            Ok(())
        }) {
            ctxt.syn_error(err);
        }
    }

    let field_name = field.ident.as_ref().unwrap().to_string();
    let info = if is_id {
        FieldInfo::new_id_field(field_name)
    } else {
        FieldInfo::new(field_name, index_type, stored.unwrap_or(true))
    };

    DeriveFieldInfo { info, type_path }
}

/// Get the type path for a field - for Vec<T>, returns the inner T type
pub fn get_field_type_path(field: &DeriveFieldInfo) -> syn::TypePath {
    // If it's a Vec<T>, extract T
    if let Some(inner) = extract_vec_inner_type(&field.type_path) {
        inner
    } else {
        field.type_path.clone()
    }
}

/// Check if field is a Vec type
pub fn is_vec_field(field: &DeriveFieldInfo) -> bool {
    extract_vec_inner_type(&field.type_path).is_some()
}
