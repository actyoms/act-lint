use proc_macro::TokenStream;

use attribute_derive::Attribute;
use proc_macro2::Ident;
use syn::{DeriveInput, Result};

use crate::{Attributes, Input};

pub(crate) fn parse_input(input: TokenStream) -> Result<Input> {
    let drive_input: DeriveInput = syn::parse(input).unwrap();
    Attributes::from_attributes(&drive_input.attrs)
        .and_then(|a| {
            if a.visit.is_none() && a.visitor.is_none() {
                Err(syn::Error::new_spanned(
                    drive_input.ident.clone(),
                    "either `visit` or `visitor` must be specified in #[act(...)] attribute",
                ))
            } else {
                validate_visits(&drive_input.ident, a)
            }
        })
        .map(|attrs| Input::from_attributes(attrs, drive_input))
}

fn predefined_visit(v: &str) -> bool {
    matches!(v, "str" | "map" | "bool" | "f64" | "i64" | "u64" | "number")
}

fn validate_visits(ident: &Ident, a: Attributes) -> Result<Attributes> {
    if let Some(visit) = &a.visit {
        let err = visit
            .iter()
            .filter_map(|v| {
                if !predefined_visit(v) {
                    Some(v.as_str().to_string())
                } else {
                    None
                }
            })
            .reduce(|a, b| format!("{}, {}", a, b))
            .map(|v| {
                syn::Error::new_spanned(
                    ident.clone(),
                    format!("invalid `visit` value(s): {} expected `str`, `bool`, `number`, `f64`, `i64`, `u64` or `map`", v),
                )
            });
        if let Some(err) = err {
            return Err(err);
        }
    }
    Ok(a)
}
