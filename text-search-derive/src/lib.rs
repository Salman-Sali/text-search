mod field_info;
mod indexable;
use field_info::parse_field_info;
use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

// Note: We use manual parsing instead of darling for the top-level derive
// to maintain compatibility with the existing code structure

#[proc_macro_derive(Indexed, attributes(text_search))]
pub fn text_search_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    // Get the fields from the input
    let fields = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
        } else {
            return syn::Error::new_spanned(input, "Only named fields are supported.")
                .to_compile_error()
                .into();
        }
    } else {
        return syn::Error::new_spanned(input, "Only structs are supported.")
            .to_compile_error()
            .into();
    };

    let mut derive_fields: Vec<field_info::DeriveFieldInfo> = Vec::new();
    let mut errors: Vec<syn::Error> = Vec::new();

    for field in fields.named.iter() {
        match parse_field_info(field) {
            Ok(info) => derive_fields.push(info),
            Err(e) => {
                // Collect the error
                errors.push(e);
            }
        }
    }

    // If there were parsing errors, report them
    if !errors.is_empty() {
        let mut combined = errors.remove(0);
        for err in errors {
            combined.extend(err);
        }
        return combined.to_compile_error().into();
    }

    let impl_indexable = indexable::impl_indexable_token(
        name.clone(),
        text_search_core::StructInfo::new(name.to_string()),
        &derive_fields,
    );

    impl_indexable.into()
}
