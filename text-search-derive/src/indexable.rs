use quote::{quote, ToTokens};
use syn::{parse_str, Expr, Ident};
use template::StructInfo;

use crate::field_info::{gen_field_info_temp_var_assignments, gen_field_info_to_document, gen_field_info_token};

pub fn impl_indexable_token(
    struct_name: Ident,
    struct_info: StructInfo,
) -> proc_macro2::TokenStream {
    let get_struct_info = gen_get_struct_info_token(&struct_info);
    let get_as_document = gen_as_document(&struct_info);
    let get_from_doc = gen_from_document(&struct_info);
    quote! {
        impl text_search::Indexable for #struct_name {
            #get_struct_info
            #get_as_document
            #get_from_doc
        }
    }
}

fn gen_get_struct_info_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let mut field_tokens: proc_macro2::TokenStream = quote! {};
    for field in &struct_info.fields {
        gen_field_info_token(&field).to_tokens(&mut field_tokens);
    }

    let struct_name = format!("{}", struct_info.struct_name);
    quote! {
        fn get_struct_info() -> text_search::StructInfo {
            text_search::StructInfo {
                struct_name: #struct_name.into(),
                fields: vec![
                    #field_tokens
                ]
            }
        }
    }
}

fn gen_as_document(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let mut field_tokens: proc_macro2::TokenStream = quote! {};
    for field in &struct_info.fields {
        gen_field_info_to_document(field).to_tokens(&mut field_tokens);
    }
    let struct_name = parse_str::<Expr>(&struct_info.struct_name).unwrap();
    quote! {
        fn as_document(&self) -> text_search::tantivy::TantivyDocument {
            let struct_info  = #struct_name::get_struct_info();
            let schema = struct_info.generate_schema();
            let mut doc = text_search::tantivy::TantivyDocument::default();
            #field_tokens
            doc            
        }
    }
}


fn gen_from_document(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name = parse_str::<Expr>(&struct_info.struct_name).unwrap();
    let mut field_temp_var_assignments: proc_macro2::TokenStream = quote! {};
    let mut field_self_assignement: proc_macro2::TokenStream = quote! {};
    
    for field in &struct_info.fields {
        gen_field_info_temp_var_assignments(field).to_tokens(&mut field_temp_var_assignments);
        let field_name = field.field_name.clone();
        let field_value_var = parse_str::<Expr>((field_name.to_owned() + "_value").as_str()).unwrap();
        let field_name_var = parse_str::<Expr>(field_name.as_str()).unwrap();
        quote!{#field_name_var: #field_value_var,}.to_tokens(&mut field_self_assignement);
    }
    quote! {
        fn from_doc(doc : text_search::tantivy::TantivyDocument) -> Self {
            let schema = #struct_name::get_struct_info().generate_schema();

            #field_temp_var_assignments
            Self {
                #field_self_assignement
            }
        }
    }
}