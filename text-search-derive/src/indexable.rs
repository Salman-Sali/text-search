use quote::{ToTokens, quote};
use syn::{Expr, Ident, parse_str};
use text_search_core::StructInfo;

use crate::field_info::{DeriveFieldInfo, get_field_type_path, is_vec_field};

pub fn impl_indexable_token(
    struct_name: Ident,
    _struct_info: StructInfo,
    derive_fields: &[DeriveFieldInfo],
) -> proc_macro2::TokenStream {
    let get_as_document = generate_as_document(derive_fields);
    let get_from_doc = generate_from_document(derive_fields);
    let get_id_term = generate_get_id_term(derive_fields);
    let get_term_from_id = generate_get_term_from_id(derive_fields);
    let generate_schema = generate_schema_fn(derive_fields);
    let get_struct_info = generate_get_struct_info_token(&struct_name, derive_fields);
    let field_ref_impl = generate_field_ref_impls(&struct_name, derive_fields);

    quote! {
        impl text_search::Indexable for #struct_name {
            #get_as_document
            #get_from_doc
            #get_id_term
            #generate_schema
            #get_struct_info
        }

        impl #struct_name {
            #get_term_from_id
        }

        #field_ref_impl
    }
}

fn generate_field_ref_impls(
    struct_name: &Ident,
    derive_fields: &[DeriveFieldInfo],
) -> proc_macro2::TokenStream {
    let field_refs: Vec<proc_macro2::TokenStream> = derive_fields
        .iter()
        .filter_map(|field| {
            let field_name = field.info.field_name.as_str();
            let field_ident = parse_str::<syn::Ident>(field_name).ok()?;
            Some(quote! {
                pub const #field_ident: text_search::FieldRef = text_search::FieldRef::new(#field_name);
            })
        })
        .collect();

    quote! {
        #[allow(non_upper_case_globals)]
        impl #struct_name {
            #(#field_refs)*
        }
    }
}

fn generate_get_struct_info_token(
    struct_name: &Ident,
    derive_fields: &[DeriveFieldInfo],
) -> proc_macro2::TokenStream {
    let name = format!("{}", struct_name);

    // Generate field info tokens for runtime introspection
    let mut field_tokens: proc_macro2::TokenStream = quote! {};
    for field in derive_fields {
        let field_name = &field.info.field_name;
        let is_id = field.info.is_id;
        let stored = field.info.stored;
        let index_type = match field.info.index_type {
            text_search_core::IndexType::indexed_string => {
                quote! { text_search::IndexType::indexed_string }
            }
            text_search_core::IndexType::indexed_text => {
                quote! { text_search::IndexType::indexed_text }
            }
            text_search_core::IndexType::indexed => quote! { text_search::IndexType::indexed },
            text_search_core::IndexType::not_indexed => {
                quote! { text_search::IndexType::not_indexed }
            }
        };

        let token = quote! {
            text_search::FieldInfo {
                is_id: #is_id,
                field_name: #field_name.into(),
                index_type: #index_type,
                stored: #stored,
            },
        };
        token.to_tokens(&mut field_tokens);
    }

    quote! {
        fn get_struct_info() -> text_search::StructInfo {
            text_search::StructInfo {
                struct_name: #name.into(),
                fields: vec![#field_tokens],
            }
        }
    }
}

fn generate_as_document(derive_fields: &[DeriveFieldInfo]) -> proc_macro2::TokenStream {
    let mut field_tokens: proc_macro2::TokenStream = quote! {};

    for field in derive_fields {
        let field_name_str = &field.info.field_name;
        let field_name = parse_str::<Expr>(field_name_str).unwrap();
        let is_vec = is_vec_field(field);

        // For Vec<T>, use the full type path including Vec
        let type_for_trait = if is_vec {
            let inner = get_field_type_path(field);
            quote! { Vec<#inner> }
        } else {
            let type_path = get_field_type_path(field);
            quote! { #type_path }
        };

        let token = quote! {
            let #field_name = schema.get_field(#field_name_str).unwrap();
            <#type_for_trait as text_search::IndexField>::add_to_document(&self.#field_name, &mut doc, #field_name);
        };
        token.to_tokens(&mut field_tokens);
    }

    quote! {
        fn as_document(&self) -> text_search::tantivy::TantivyDocument {
            let schema = Self::generate_schema();
            let mut doc = text_search::tantivy::TantivyDocument::default();
            #field_tokens
            doc
        }
    }
}

fn generate_from_document(derive_fields: &[DeriveFieldInfo]) -> proc_macro2::TokenStream {
    let mut field_assignments: proc_macro2::TokenStream = quote! {};

    for field in derive_fields {
        let field_name_str = &field.info.field_name;
        let field_name = parse_str::<Expr>(field_name_str).unwrap();
        let type_path = get_field_type_path(field);
        let is_vec = is_vec_field(field);

        let assignment = if is_vec {
            // For Vec<T>, use IndexFieldVec trait
            quote! {
                #field_name: {
                    let field = schema.get_field(#field_name_str).unwrap();
                    <Vec<#type_path> as text_search::IndexFieldVec<#type_path>>::from_doc_all(&doc, field)
                },
            }
        } else {
            // For single values
            quote! {
                #field_name: {
                    let field = schema.get_field(#field_name_str).unwrap();
                    <#type_path as text_search::IndexFieldSingle>::from_doc_single(&doc, field)
                        .unwrap_or_default()
                },
            }
        };
        assignment.to_tokens(&mut field_assignments);
    }

    quote! {
        fn from_doc(doc : text_search::tantivy::TantivyDocument) -> Self {
            let schema = Self::generate_schema();
            Self {
                #field_assignments
            }
        }
    }
}

fn generate_get_id_term(derive_fields: &[DeriveFieldInfo]) -> proc_macro2::TokenStream {
    // Find the id field
    let id_field = derive_fields
        .iter()
        .find(|f| f.info.is_id)
        .expect("Missing id field");

    let field_name_str = &id_field.info.field_name;
    let field_name = parse_str::<Expr>(field_name_str).unwrap();
    let type_path = get_field_type_path(id_field);

    quote! {
        fn get_id_term(&self) -> text_search::tantivy::Term {
            let field = Self::generate_schema().get_field(#field_name_str).unwrap();
            <#type_path as text_search::IndexField>::to_term(&self.#field_name, field)
        }
    }
}

fn generate_get_term_from_id(derive_fields: &[DeriveFieldInfo]) -> proc_macro2::TokenStream {
    // Find the id field
    let id_field = derive_fields
        .iter()
        .find(|f| f.info.is_id)
        .expect("Missing id field");

    let field_name_str = &id_field.info.field_name;
    let field_name_ident = parse_str::<syn::Ident>(field_name_str).unwrap();
    let type_path = get_field_type_path(id_field);

    quote! {
        pub fn get_term_from_id(#field_name_ident: #type_path) -> text_search::tantivy::Term {
            let field = Self::generate_schema().get_field(#field_name_str).unwrap();
            <#type_path as text_search::IndexField>::to_term(&#field_name_ident, field)
        }
    }
}

/// Generate the generate_schema function that creates tantivy schema at compile time
fn generate_schema_fn(derive_fields: &[DeriveFieldInfo]) -> proc_macro2::TokenStream {
    let mut field_tokens: proc_macro2::TokenStream = quote! {};

    for field in derive_fields {
        let field_name_str = &field.info.field_name;
        let type_path = get_field_type_path(field);
        let stored = field.info.stored;
        let index_type = match field.info.index_type {
            text_search_core::IndexType::indexed_string => {
                quote! { text_search::IndexType::indexed_string }
            }
            text_search_core::IndexType::indexed_text => {
                quote! { text_search::IndexType::indexed_text }
            }
            text_search_core::IndexType::indexed => quote! { text_search::IndexType::indexed },
            text_search_core::IndexType::not_indexed => {
                quote! { text_search::IndexType::not_indexed }
            }
        };

        let token = quote! {
            <#type_path as text_search::IndexField>::add_to_schema(&mut schema_builder, #field_name_str, #stored, #index_type);
        };
        token.to_tokens(&mut field_tokens);
    }

    quote! {
        fn generate_schema() -> text_search::tantivy::schema::Schema {
            let mut schema_builder = text_search::tantivy::schema::Schema::builder();
            #field_tokens
            schema_builder.build()
        }
    }
}
