
use crate::context::Ctxt;

use quote::quote;
use syn::{parse_str, Expr, Field, Type};
use text_search_core::{symbol::*, FieldInfo, FieldType, IndexType};

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
    if is_id {
        FieldInfo::new_id_field(field_name, field_type)
    } else {
        FieldInfo::new(field_name, field_type, index_type, stored.unwrap_or(true))
    }
}

pub fn generate_field_info_token(field_info: &FieldInfo) -> proc_macro2::TokenStream {
    let index_type_token = match field_info.index_type {
        IndexType::indexed_string => quote! {text_search::IndexType::indexed_string},
        IndexType::indexed_text => quote! {text_search::IndexType::indexed_text},
        IndexType::indexed => quote! {text_search::IndexType::indexed},
        IndexType::not_indexed => quote! {text_search::IndexType::not_indexed},
    };

    let stored_token = match field_info.stored {
        true => quote! {true},
        false => quote! {false},
    };

    let field_type_token = match field_info.field_type {
        FieldType::String => quote! {text_search::FieldType::String},
        FieldType::I32 => quote! {text_search::FieldType::I32},
        FieldType::Unhandled => quote! {text_search::FieldType::Unhandled},
    };

    let field_name = format!("{}", field_info.field_name);

    quote! {text_search::FieldInfo::new(#field_name.into(), #field_type_token, Some(#index_type_token), #stored_token),}
}

pub fn generate_field_info_to_document(field_info: &FieldInfo) -> proc_macro2::TokenStream {
    let field_name = parse_str::<Expr>(&field_info.field_name).unwrap();
    let field_name_string = format!("{}", field_info.field_name);
    match field_info.field_type {
        FieldType::String => quote! {
            let #field_name = schema.get_field(#field_name_string).unwrap();
            doc.add_text(#field_name, &self.#field_name);
        },
        FieldType::I32 => quote! {
            let #field_name = schema.get_field(#field_name_string).unwrap();
            doc.add_i64(#field_name, self.#field_name as i64);
        },
        FieldType::Unhandled => panic!("Unhandled field type."),
    }
}

pub fn generate_field_info_temp_var_assignments(field_info: &FieldInfo) -> proc_macro2::TokenStream {
    let field_name = &field_info.field_name;
    let field_name_value = format!("{}", field_name);
    let field_id_var = parse_str::<Expr>((field_name.to_owned() + "_field_id").as_str()).unwrap();
    let field_owned_value_var = parse_str::<Expr>((field_name.to_owned() + "_owned_value").as_str()).unwrap();
    let field_value_var = parse_str::<Expr>((field_name.to_owned() + "_value").as_str()).unwrap();
    let field_value_assignment = match field_info.field_type {
        FieldType::String => quote! {if let text_search::tantivy::schema::OwnedValue::Str(s) = #field_owned_value_var { s } else { Default::default() };},
        FieldType::I32 =>  quote! {if let text_search::tantivy::schema::OwnedValue::I64(i) = #field_owned_value_var { i as i32 } else { Default::default() };},
        FieldType::Unhandled => panic!("Unhandled field type."),
    };
    quote! {
        let #field_id_var =  schema.get_field(#field_name_value).unwrap().field_id();
        let #field_owned_value_var = doc.field_values().into_iter().filter(|x| x.field.field_id() == #field_id_var).next().unwrap().value.clone();
        let #field_value_var = #field_value_assignment
    }
}

pub fn generate_term_initialisation(field_info: &FieldInfo, include_self: bool) -> proc_macro2::TokenStream {
    let field_name = parse_str::<Expr>(&{
        if include_self {
            let value = format!("self.{}", field_info.field_name.clone());
            value
        } else {
            field_info.field_name.clone()
        }        
    }).unwrap();
    //let field_name = parse_str::<Expr>(&field_info.field_name).unwrap();
    return match field_info.field_type {
        FieldType::String => quote! {
            return text_search::tantivy::Term::from_field_text(field, &#field_name);
        },
        FieldType::I32 => quote! {
            return text_search::tantivy::Term::from_field_i64(field, #field_name as i64);
        },
        FieldType::Unhandled => panic!("Unhandled field type."),
    };
}