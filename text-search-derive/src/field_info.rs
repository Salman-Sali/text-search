use darling::FromMeta;
use text_search_core::IndexType;

/// Extended field info that includes the parsed type for code generation
#[derive(Debug)]
pub struct DeriveFieldInfo {
    pub info: text_search_core::FieldInfo,
    pub type_path: syn::TypePath,
}

/// Darling receiver for the `#[text_search(...)]` attribute flags.
///
/// Each flag is a bare word (e.g. `#[text_search(id)]` or
/// `#[text_search(indexed_text, stored)]`). Darling treats a bare word
/// `foo` as `foo = true` when the receiver field is `bool`.
#[derive(Debug, Default, FromMeta)]
pub struct TextSearchAttr {
    #[darling(default)]
    pub id: bool,
    #[darling(default)]
    pub indexed_string: bool,
    #[darling(default)]
    pub indexed_text: bool,
    #[darling(default)]
    pub indexed: bool,
    #[darling(default)]
    pub not_indexed: bool,
    #[darling(default)]
    pub stored: bool,
    #[darling(default)]
    pub not_stored: bool,
}

impl TextSearchAttr {
    /// Validate flags and convert into a `text_search_core::FieldInfo`.
    fn into_field_info(
        self,
        field_name: String,
    ) -> Result<text_search_core::FieldInfo, darling::Error> {
        // Determine the index type (at most one allowed)
        let index_flags: Vec<(&str, IndexType)> = [
            ("indexed_string", IndexType::indexed_string),
            ("indexed_text", IndexType::indexed_text),
            ("indexed", IndexType::indexed),
            ("not_indexed", IndexType::not_indexed),
        ]
        .into_iter()
        .filter(|(name, _)| match *name {
            "indexed_string" => self.indexed_string,
            "indexed_text" => self.indexed_text,
            "indexed" => self.indexed,
            "not_indexed" => self.not_indexed,
            _ => false,
        })
        .collect();

        if index_flags.len() > 1 {
            return Err(darling::Error::custom(
                "Cannot specify multiple index types for the same field",
            ));
        }

        let index_type = index_flags.into_iter().next().map(|(_, it)| it);

        // Validate stored / not_stored
        if self.stored && self.not_stored {
            return Err(darling::Error::custom(
                "Cannot specify both `stored` and `not_stored` for the same field",
            ));
        }

        let stored = if self.not_stored {
            Some(false)
        } else if self.stored {
            Some(true)
        } else {
            None
        };

        // Validate: id cannot coexist with index type or stored flags
        if self.id && (index_type.is_some() || stored.is_some()) {
            return Err(darling::Error::custom(
                "Cannot have other attributes when field has `id` attribute",
            ));
        }

        if self.id {
            Ok(text_search_core::FieldInfo::new_id_field(field_name))
        } else {
            Ok(text_search_core::FieldInfo::new(
                field_name,
                index_type,
                stored.unwrap_or(true),
            ))
        }
    }
}

/// Parse a `syn::Field` into a `DeriveFieldInfo` using darling for attribute parsing.
///
/// Returns `syn::Error` so that `lib.rs` can collect errors without changes.
pub fn parse_field_info(field: &syn::Field) -> Result<DeriveFieldInfo, syn::Error> {
    // Extract the type path
    let type_path = if let syn::Type::Path(tp) = &field.ty {
        tp.clone()
    } else {
        syn::TypePath {
            qself: None,
            path: syn::parse_str("std::marker::PhantomData<u8>").unwrap(),
        }
    };

    // Find the #[text_search(...)] attribute and parse it with darling
    let attr = field
        .attrs
        .iter()
        .find(|a| a.path().is_ident("text_search"));

    let ts_attr = match attr {
        Some(a) => {
            let meta = &a.meta;
            TextSearchAttr::from_meta(meta).map_err(|e| e.with_span(&a.meta))?
        }
        None => TextSearchAttr::default(),
    };

    // Validate: field must have a name
    let field_name = field
        .ident
        .as_ref()
        .ok_or_else(|| syn::Error::new_spanned(field, "Field must have a name"))?
        .to_string();

    let info = ts_attr
        .into_field_info(field_name)
        .map_err(|e| e.with_span(field))?;

    Ok(DeriveFieldInfo { info, type_path })
}

/// Extract the inner type T from Vec<T>
fn extract_vec_inner_type(type_path: &syn::TypePath) -> Option<syn::TypePath> {
    let segment = type_path.path.segments.last()?;
    if segment.ident != "Vec" {
        return None;
    }

    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(syn::Type::Path(inner))) = args.args.first() {
            return Some(inner.clone());
        }
    }
    None
}

/// Get the type path for a field - for Vec<T>, returns the inner T type
pub fn get_field_type_path(field: &DeriveFieldInfo) -> syn::TypePath {
    if let Some(inner) = extract_vec_inner_type(&field.type_path) {
        inner
    } else {
        field.type_path.clone()
    }
}

/// Check if field is a Vec type
pub fn is_vec_field(field: &DeriveFieldInfo) -> bool {
    extract_vec_inner_type(&field.type_path).is_some()
}
