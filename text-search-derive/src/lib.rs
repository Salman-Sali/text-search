mod context;
mod field_info;
mod indexable;
mod struct_info;
use context::Ctxt;
use field_info::get_field_info;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

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

    let mut derive_fields: Vec<field_info::DeriveFieldInfo> = Vec::new();

    for field in fields.named.iter() {
        derive_fields.push(get_field_info(&ctxt, field));
    }

    let impl_indexable = indexable::impl_indexable_token(
        name.clone(),
        text_search_core::StructInfo::new(name.to_string()),
        &derive_fields,
    );

    impl_indexable.into()
}
