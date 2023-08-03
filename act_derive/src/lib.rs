extern crate act_trait;
extern crate serde;

use proc_macro::TokenStream;

use attribute_derive::Attribute;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DeriveInput, Generics, Result, Visibility};

mod expecting;

#[derive(Attribute, Debug)]
#[attribute(ident = act)]
struct ActAttribute {
    #[attribute(conflicts = [visit])]
    visitor: Option<String>,
    #[attribute(conflicts = [visitor])]
    visit: Option<Vec<String>>,
    expecting: Option<String>,
}

impl ActAttribute {
    fn is_valid_visit(v: &str) -> bool {
        matches!(v, "str" | "map")
    }
}

struct Input {
    name: Ident,
    attrs: ActAttribute,
    #[allow(dead_code)]
    vis: Visibility,
    #[allow(dead_code)]
    generics: Generics,
    #[allow(dead_code)]
    data: Data,
}

#[proc_macro_derive(Deserialize, attributes(act))]
pub fn deserialize_derive(tokens: TokenStream) -> TokenStream {
    all_attrs(tokens.clone())
        .map(|input| impl_deserialize(&input))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn all_attrs(input: TokenStream) -> Result<Input> {
    let DeriveInput {
        ident,
        attrs: atts,
        vis,
        generics,
        data,
    } = syn::parse(input)?;
    ActAttribute::from_attributes(&atts)
        .and_then(|a| {
            if a.visit.is_none() && a.visitor.is_none() {
                Err(syn::Error::new_spanned(
                    ident.clone(),
                    "either `visit` or `visitor` must be specified in #[act(...)] attribute",
                ))
            } else {
                if let Some(visit) = &a.visit {
                    let err = visit
                        .iter()
                        .filter_map(|v| {
                            if !ActAttribute::is_valid_visit(v) {
                                Some(v.as_str().to_string())
                            } else {
                                None
                            }
                        })
                        .reduce(|a, b| format!("{}, {}", a, b))
                        .map(|v| {
                            syn::Error::new_spanned(
                                ident.clone(),
                                format!("invalid `visit` value(s): {} expected `str` or `map`", v),
                            )
                        });
                    if let Some(err) = err {
                        return Err(err);
                    }
                }
                Ok(a)
            }
        })
        .map(|attrs| Input {
            name: ident.clone(),
            attrs,
            vis,
            generics,
            data,
        })
}

fn impl_deserialize(input: &Input) -> proc_macro2::TokenStream {
    let attr = &input.attrs;
    let name = &input.name;
    let mut visitor = None;
    let visitor_x_impl = if attr.visitor.is_some() {
        quote! {}
    } else {
        visitor = Some(quote! { ActVisitor });
        visitor_token_stream(name, attr)
    };
    let expecting = expecting::impl_expecting(input);
    let gen = quote! {
        #expecting

        #visitor_x_impl

        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(#visitor)
            }
        }
    };
    // if let Some(expecting) = attr.expecting {
    //     expecting.into()
    // }
    gen
}

fn visitor_token_stream(name: &Ident, attr: &ActAttribute) -> proc_macro2::TokenStream {
    let mut need_str = false;
    let mut need_map = false;
    if let Some(visit) = &attr.visit {
        need_str = visit.contains(&"str".to_string());
        need_map = visit.contains(&"map".to_string());
    }
    let visit_str = visit_str_wrapper(name, need_str);
    let visit_map = visit_map_wrapper(name, need_map);
    quote! {
        struct ActVisitor;

        impl<'de> serde::de::Visitor<'de> for ActVisitor {
            type Value = #name;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                fn use_trait<T>(_method: fn(&mut std::fmt::Formatter) -> std::fmt::Result) {}
                use_trait::<#name>(<#name as act_trait::Expecting>::expecting);
                #name::expecting(formatter)
            }

            #visit_str

            #visit_map
        }
    }
}

fn visit_str_wrapper(name: &Ident, need: bool) -> proc_macro2::TokenStream {
    if need {
        quote! {
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                <#name as std::str::FromStr>::from_str(v)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &self))
            }
        }
    } else {
        quote! {}
    }
}

fn visit_map_wrapper(name: &Ident, need: bool) -> proc_macro2::TokenStream {
    if need {
        quote! {
            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                fn use_trait(_method: fn() -> bool) {}
                use_trait(<#name as act_trait::VisitMap<#name>>::is_map_visitor);
                #name::visit_map(map)
            }
        }
    } else {
        quote! {}
    }
}
