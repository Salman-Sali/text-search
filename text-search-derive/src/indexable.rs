use quote::{quote, ToTokens};
use syn::{parse_str, Expr, Ident};
use template::StructInfo;

use crate::field_info::gen_field_info_token;

pub fn impl_indexable_token(
    struct_name: Ident,
    struct_info: StructInfo,
) -> proc_macro2::TokenStream {
    let get_struct_info = gen_get_struct_info_token(&struct_info);
    let get_as_document = gen_as_document(&struct_info);
    quote! {
        impl text_search::Indexable for #struct_name {
            #get_struct_info
            #get_as_document
        }
    }
}

fn gen_get_struct_info_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let mut field_tokens: proc_macro2::TokenStream = quote! {};
    for field in &struct_info.fields {
        gen_field_info_token(&field).to_tokens(&mut field_tokens);
    }

    quote! {
        fn get_struct_info(&self) -> text_search::StructInfo {
            text_search::StructInfo {
                fields: vec![
                    #field_tokens
                ]
            }
        }
    }
}

fn gen_as_document(struct_info: &StructInfo) -> proc_macro2::TokenStream {

    // for field in &struct_info.fields {

    // }
    let a = parse_str::<Expr>("value").unwrap();
    quote! {
        fn as_document(&self) -> text_search::tantivy::TantivyDocument {
            let mut doc = text_search::tantivy::TantivyDocument::default();
            #a;
            doc            
        }
    }
}
