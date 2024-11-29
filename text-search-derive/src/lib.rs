mod utils;
mod macros;
mod models;
use models::{IndexType, StorageType, FieldMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Field, Meta};

/// Marks a struct as searchable
#[proc_macro_attribute]
pub fn searchable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let struct_name = &input.ident;
    let fields = if let syn::Data::Struct(DataStruct { fields, .. }) = &input.data {
        fields
    } else {
        panic!("#[searchable] can only be applied to structs");
    };

    let field_configs = fields.iter().map(|field| {
        let name = field.ident.as_ref().unwrap().to_string();
        quote! {
            field_configs.push((#name.to_string(), field_meta.get(#name).cloned()));
        }
    });

    let gen = quote! {
        impl #struct_name {
            pub fn lucene_schema() -> Vec<(&'static str, Option<FieldMeta>)> {
                let mut field_configs = Vec::new();
                #(#field_configs)*
                field_configs
            }
        }
    };

    let output = quote! {
        #input
        #gen
    };

    output.into()
}

#[proc_macro_attribute]
pub fn field(attr: TokenStream, item: TokenStream) -> TokenStream {
    let meta = Attribute::parse_outer.parse(attr).unwrap();

    // Default enum values
    let mut index_type = IndexType::None;
    let mut storage_type = StorageType::NotStored;

    for meta_item in meta {
        let a = meta_item.path();
    }
    

    let input = Field::parse_named(item).unwrap();
    let field_name = input.ident.as_ref().unwrap();


    let output = quote! {
        #[allow(non_upper_case_globals)]
        const #field_name: FieldMeta = FieldMeta {
            index_type: #index_type,
            storage_type: #storage_type,
        };

        #input
    };

    output.into()
}