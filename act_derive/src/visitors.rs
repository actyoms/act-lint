use proc_macro2::Ident;
use quote::{format_ident, quote};

use act_macrons::{impl_visit, quote_if};

use crate::Input;

pub(crate) fn impl_visitor(
    name: &Ident,
    visitor: &Ident,
    input: &Input,
) -> proc_macro2::TokenStream {
    let visit_bool = impl_visit_bool(name, input.should_add_bool());
    let visit_f64 = impl_visit_f64(name, input.should_add_f64());
    let visit_i64 = impl_visit_i64(name, input.should_add_i64());
    let visit_map = impl_visit_map(name, input.should_add_map());
    let visit_str = impl_visit_str(name, input.should_add_str());
    let visit_u64 = impl_visit_u64(name, input.should_add_u64());
    quote! {
        struct #visitor;

        impl<'de> serde::de::Visitor<'de> for #visitor {
            type Value = #name;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #name::expecting(formatter)
            }

            #visit_bool
            #visit_f64
            #visit_i64
            #visit_map
            #visit_str
            #visit_u64
        }
    }
}

pub(crate) fn impl_visit_map(name: &Ident, need: bool) -> proc_macro2::TokenStream {
    quote_if!(need,
        {
            fn visit_map<E>(self, map: E) -> Result<Self::Value, E::Error>
            where
                E: serde::de::MapAccess<'de>,
            {
                #name::from_map(map)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))
            }
        }
    )
}

pub(crate) fn impl_visit_str(name: &Ident, should_add: bool) -> proc_macro2::TokenStream {
    let impl_method = format_ident!("visit_{}", "str");
    let from_method = format_ident!("from_{}", "str");
    let arg_type: proc_macro2::TokenStream = "&str".parse().unwrap();
    // serde::de::Unexpected::Str()
    let unexp = format_ident!("Str");
    if should_add {
        impl_visit!(name, impl_method, from_method, arg_type, unexp)
    } else {
        quote! {}
    }
}

pub(crate) fn impl_visit_bool(name: &Ident, should_add: bool) -> proc_macro2::TokenStream {
    let impl_method = format_ident!("visit_{}", "bool");
    let from_method = format_ident!("from_{}", "bool");
    let arg_type = format_ident!("bool");
    // serde::de::Unexpected::Bool()
    let unexp = format_ident!("Bool");
    if should_add {
        impl_visit!(name, impl_method, from_method, arg_type, unexp)
    } else {
        quote! {}
    }
}

pub(crate) fn impl_visit_f64(name: &Ident, should_add: bool) -> proc_macro2::TokenStream {
    let impl_method = format_ident!("visit_{}", "f64");
    let from_method = format_ident!("from_{}", "f64");
    let arg_type: proc_macro2::TokenStream = "f64".parse().unwrap();
    // serde::de::Unexpected::Float()
    let unexp = format_ident!("Float");
    if should_add {
        impl_visit!(name, impl_method, from_method, arg_type, unexp)
    } else {
        quote! {}
    }
}

pub(crate) fn impl_visit_i64(name: &Ident, need: bool) -> proc_macro2::TokenStream {
    let impl_method = format_ident!("visit_{}", "i64");
    let from_method = format_ident!("from_{}", "i64");
    let arg_type: proc_macro2::TokenStream = "i64".parse().unwrap();
    // serde::de::Unexpected::Signed()
    let unexp = format_ident!("Signed");
    if need {
        impl_visit!(name, impl_method, from_method, arg_type, unexp)
    } else {
        quote! {}
    }
}

pub(crate) fn impl_visit_u64(name: &Ident, need: bool) -> proc_macro2::TokenStream {
    let impl_method = format_ident!("visit_{}", "u64");
    let from_method = format_ident!("from_{}", "u64");
    let arg_type: proc_macro2::TokenStream = "u64".parse().unwrap();
    // serde::de::Unexpected::Unsigned()
    let unexp = format_ident!("Unsigned");
    if need {
        impl_visit!(name, impl_method, from_method, arg_type, unexp)
    } else {
        quote! {}
    }
}

// struct Vis;
//
// impl<'de> Visitor<'de>  for Vis {
//     type Value = ();
//
//     fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
//         todo!()
//     }
//
//     fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_char<E>(self, v: char) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_none<E>(self) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
//         todo!()
//     }
//
//     fn visit_unit<E>(self) -> Result<Self::Value, E> where E: Error {
//         todo!()
//     }
//
//     fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
//         todo!()
//     }
//
//     fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de> {
//         todo!()
//     }
//
//     fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
//         todo!()
//     }
//
//     fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error> where A: EnumAccess<'de> {
//         todo!()
//     }
// }
