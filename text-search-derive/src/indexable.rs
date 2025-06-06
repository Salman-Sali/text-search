use quote::{quote, ToTokens};
use syn::{parse_str, Expr, Ident};
use text_search_core::StructInfo; 

use crate::field_info::{generate_field_info_temp_var_assignments, generate_field_info_to_document, generate_field_info_token, generate_term_initialisation};

pub fn impl_indexable_token(
    struct_name: Ident,
    struct_info: StructInfo,
) -> proc_macro2::TokenStream {
    let get_struct_info = generate_get_struct_info_token(&struct_info);
    let get_as_document = generate_as_document(&struct_info);
    let get_from_doc = generate_from_document(&struct_info);
    let get_id_term = generate_get_id_term(&struct_info);
    let get_term_from_id = generate_get_term_from_id(&struct_info);
    quote! {
        impl text_search::Indexable for #struct_name {
            #get_struct_info
            #get_as_document
            #get_from_doc
            #get_id_term
        }

        impl #struct_name {
            #get_term_from_id
        }
    }
}

fn generate_get_struct_info_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let mut field_tokens: proc_macro2::TokenStream = quote! {};
    for field in &struct_info.fields {
        generate_field_info_token(&field).to_tokens(&mut field_tokens);
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

fn generate_as_document(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let mut field_tokens: proc_macro2::TokenStream = quote! {};
    for field in &struct_info.fields {
        generate_field_info_to_document(field).to_tokens(&mut field_tokens);
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


fn generate_from_document(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name = parse_str::<Expr>(&struct_info.struct_name).unwrap();
    let mut field_temp_var_assignments: proc_macro2::TokenStream = quote! {};
    let mut field_self_assignement: proc_macro2::TokenStream = quote! {};
    
    for field in &struct_info.fields {
        generate_field_info_temp_var_assignments(field).to_tokens(&mut field_temp_var_assignments);
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

fn generate_get_id_term(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name = parse_str::<Expr>(&struct_info.struct_name).unwrap();
    let id_field_info = struct_info.get_id_field();
    let term_initialisation: proc_macro2::TokenStream = generate_term_initialisation(&id_field_info, true);
    let field_name = id_field_info.field_name.clone();
    quote! {
        fn get_id_term(&self) -> text_search::tantivy::Term {
            let field = #struct_name::get_struct_info().generate_schema().get_field(#field_name).unwrap();
            #term_initialisation
        }
    }
}

fn generate_get_term_from_id(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name = parse_str::<Expr>(&struct_info.struct_name).unwrap();
    let id_field_info = struct_info.get_id_field();
    let id_field_type = match id_field_info.field_type {
        text_search_core::FieldType::String => quote! { String },
        text_search_core::FieldType::I32 => quote! { i32 },
        text_search_core::FieldType::Unhandled => panic!("unhandled field type."),
    };
    let id_field_name_expr = parse_str::<Expr>(&id_field_info.field_name).unwrap();
    let field_name = id_field_info.field_name.clone();
    let term_initialisation: proc_macro2::TokenStream = generate_term_initialisation(&id_field_info, false);
    quote! {
        pub fn get_term_from_id(#id_field_name_expr: #id_field_type) -> text_search::tantivy::Term {
            use text_search::Indexable;
            let field = #struct_name::get_struct_info().generate_schema().get_field(#field_name).unwrap();
            #term_initialisation
        }
    }
}