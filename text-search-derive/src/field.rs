use std::fmt::format;

use super::symbol::*;
use quote::quote;
use syn::Field;
use template::{FieldInfo, FieldType, Stored};

pub fn get_field_info(field: &Field) -> proc_macro2::TokenStream {
    let mut field_type: Option<FieldType> = None;
    let mut stored: Option<Stored> = None;

    for attr in &field.attrs {
        if attr.path() != TEXT_SEARCH {
            continue;
        }

        if attr.meta.path() == INDEXED_STRING {
            field_type = Some(FieldType::String);
        } else if attr.meta.path() == INDEXED_TEXT {
            field_type = Some(FieldType::Text);
        } else if attr.meta.path() == NOT_INDEXED {
            field_type = Some(FieldType::NotIndexed);
        }

        if attr.meta.path() == STORED {
            stored = Some(Stored::Yes);
        } else if attr.meta.path() == NOT_STORED {
            stored = Some(Stored::No);
        }
    }

    let field_type_token = match field_type {
        Some(x) => match x {
            FieldType::String => quote! {text_search::FieldType::String},
            FieldType::Text => quote! {text_search::FieldType::Text},
            FieldType::NotIndexed => quote! {text_search::FieldType::NotIndexed},
        },
        None => quote! {text_search::FieldType::NotIndexed},
    };

    let stored_token = match stored {
        Some(x) => match x {
            Stored::Yes => quote! {text_search::Stored::Yes},
            Stored::No => quote! {text_search::Stored::No},
        },
        None => quote!{text_search::Stored::No},
    };

    let field_name = format!("{}", field.ident.as_ref().unwrap().to_string());
    quote!{text_search::FieldInfo::new(#field_name.into(), #field_type_token, #stored_token),}
}
