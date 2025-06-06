mod context;
mod field_info;
mod struct_info;
mod indexable;
use context::Ctxt;
use field_info::get_field_info;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use template::StructInfo;

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

    let mut struct_info = StructInfo::new(name.to_string());

    for field in fields.named.iter() {
        struct_info.add_field(get_field_info(&ctxt, field));
    }

    indexable::impl_indexable_token(name, struct_info).into()
}

