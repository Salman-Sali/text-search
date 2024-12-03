mod models;
mod field;
mod context;
mod struct;
use context::Ctxt;
use field::get_field_info;
use models::*;

use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Meta};


#[proc_macro_derive(Indexed, attributes(text_search))]
pub fn text_search_macro(input: TokenStream) -> TokenStream {
    let ctxt = Ctxt::new();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields
        } else {
            panic!("Only named fields are supported.");
        }
    } else {
        panic!("Only structs are supported.");
    };

    let mut field_tokens: proc_macro2::TokenStream = quote! {};
    for field in fields.named.iter() {
        get_field_info(&ctxt, field).to_tokens(&mut field_tokens);
    }

    //ctxt.check();

    impl_indexable_token(name, field_tokens).into()
}

fn impl_indexable_token(struct_name: Ident, field_tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        impl text_search::Indexable for #struct_name {
            fn get_struct_info(self) -> text_search::StructInfo {
                text_search::StructInfo {
                    fields: vec![
                        #field_tokens
                    ]
                }
            }
        }
    }
}