mod models;
use models::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Meta};


#[proc_macro_derive(MySerialize, attributes(string, text, stored))]
pub fn my_serialize(input: TokenStream) -> TokenStream {
    //input
    todo!()
}