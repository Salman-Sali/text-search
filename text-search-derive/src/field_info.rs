use std::fmt::format;

use crate::context::Ctxt;

use quote::quote;
use syn::{Field, Type};
use template::{symbol::*, FieldInfo, FieldType, IndexType};

pub fn get_field_info(ctxt: &Ctxt, field: &Field) -> FieldInfo {
    let mut is_id: bool = false;
    let mut index_type: Option<IndexType> = None;
    let mut stored: Option<bool> = None;
    let field_type: FieldType = if let Type::Path(type_path) = &field.ty {
        if let Some(segment) = type_path.path.segments.last() {
            FieldType::get_field_type(segment.ident.to_string().as_str())
        } else {
            FieldType::Unhandled
        }
    } else {
        FieldType::Unhandled
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
                panic!("Cannot have {:?} and {:?} together", index_type.clone().unwrap(), _index_type.unwrap());
            } else if index_type.is_none() {
                index_type = _index_type;
            }

            if stored.is_some() && _stored.is_some() {
                panic!("Cannot have {:?} and {:?} together", stored.clone().unwrap(), _stored.unwrap());
            } else if stored.is_none() {
                stored = _stored;
            }

            if is_id && (index_type.is_some() || stored.is_some()) {
                panic!("Cannot have other attributes when field has id attribute.")
            }

            Ok(())
        }){
            ctxt.syn_error(err);
        }
    }

    let field_name = field.ident.as_ref().unwrap().to_string();
    if is_id {
        FieldInfo::new_id_field(field_name, field_type)
    } else {
        FieldInfo::new(field_name, field_type, index_type, stored.unwrap_or(false))
    }
    
}


pub fn gen_field_info_token(field_info: &FieldInfo) -> proc_macro2::TokenStream {
    let index_type_token = match field_info.index_type {
            IndexType::indexed_string => quote! {text_search::IndexType::indexed_string},
            IndexType::indexed_text => quote! {text_search::IndexType::indexed_text},
            IndexType::indexed => quote! {text_search::IndexType::indexed},
            IndexType::not_indexed => quote! {text_search::IndexType::not_indexed}
    };

    let stored_token = match field_info.stored {
            true => quote! {true},
            false => quote! {false}
    };

    let field_type_token = match field_info.field_type {
        FieldType::String => quote!{text_search::FieldType::String},
        FieldType::I32 => quote!{text_search::FieldType::I32},
        FieldType::Unhandled => quote!{text_search::FieldType::Unhandled},
    };

    let field_name = format!("{}", field_info.field_name);

    
    quote!{text_search::FieldInfo::new(#field_name.into(), #field_type_token, Some(#index_type_token), #stored_token),}
}

// pub fn gen_field_info_to_document(field_info: &FieldInfo) -> proc_macro2::TokenStream {

// }