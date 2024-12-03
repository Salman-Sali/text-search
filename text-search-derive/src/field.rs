use std::fmt::format;

use crate::context::Ctxt;

use quote::quote;
use syn::Field;
use template::{symbol::*, FieldInfo, FieldType, Stored};

pub fn get_field_info(ctxt: &Ctxt, field: &Field) -> FieldInfo {
    let mut is_id: bool = false;
    let mut field_type: Option<FieldType> = None;
    let mut stored: Option<Stored> = None;

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

            let _field_type = if meta.path == INDEXED_STRING {
                Some(FieldType::indexed_string)
            } else if meta.path == INDEXED_TEXT {
                Some(FieldType::indexed_text)
            } else if meta.path == NOT_INDEXED {
                 Some(FieldType::not_indexed)
            } else {
                None
            };
    
            let _stored = if meta.path == STORED {
                Some(Stored::stored)
            } else if meta.path == NOT_STORED {
                Some(Stored::not_stored)
            } else {
                None
            };

            if field_type.is_some() && _field_type.is_some() {
                panic!("Cannot have {:?} and {:?} together", field_type.clone().unwrap(), _field_type.unwrap());
            } else if field_type.is_none() {
                field_type = _field_type;
            }

            if stored.is_some() && _stored.is_some() {
                panic!("Cannot have {:?} and {:?} together", stored.clone().unwrap(), _stored.unwrap());
            } else if stored.is_none() {
                stored = _stored;
            }

            if is_id && (field_type.is_some() || stored.is_some()) {
                panic!("Cannot have other attributes when field has id attribute.")
            }

            Ok(())
        }){
            ctxt.syn_error(err);
        }
    }

    let field_name = field.ident.as_ref().unwrap().to_string();
    if is_id {
        FieldInfo::new_id_field(field_name)
    } else {
        FieldInfo::new(field_name, field_type, stored)
    }
    
}


pub fn generate_field_info_code(field_info: &FieldInfo) -> proc_macro2::TokenStream {
    let field_type_token = match field_info.field_type {
            FieldType::indexed_string => quote! {text_search::FieldType::indexed_string},
            FieldType::indexed_text => quote! {text_search::FieldType::indexed_text},
            FieldType::not_indexed => quote! {text_search::FieldType::not_indexed}
    };

    let stored_token = match field_info.stored {
            Stored::stored => quote! {text_search::Stored::stored},
            Stored::not_stored => quote! {text_search::Stored::not_stored}
    };

    let field_name = format!("{}", field_info.field_name);
    quote!{text_search::FieldInfo::new(#field_name.into(), #field_type_token, #stored_token),}
}