extern crate act_macrons;
extern crate act_trait;
extern crate serde;

use proc_macro::TokenStream;

use attribute_derive::Attribute;
use proc_macro2::Ident;
use syn::{Data, DeriveInput, Error, Generics, Visibility};

use deserialize::impl_deserializer;
use parse::parse_input;

mod deserialize;
mod expecting;
mod parse;
mod visitors;

#[derive(Attribute, Debug)]
#[attribute(ident = act)]
struct Attributes {
    #[attribute(conflicts = [visit])]
    visitor: Option<String>,
    #[attribute(conflicts = [visitor])]
    visit: Option<Vec<String>>,
    expecting: Option<String>,
}

struct Input {
    name: Ident,
    attrs: Attributes,
    #[allow(dead_code)]
    vis: Visibility,
    #[allow(dead_code)]
    generics: Generics,
    #[allow(dead_code)]
    data: Data,
    impls: Implementations,
}

#[derive(Default)]
struct Implementations {
    _bool: bool,
    _f64: bool,
    _i64: bool,
    _map: bool,
    _str: bool,
    _u64: bool,
    _number: bool,
}

impl Input {
    fn should_add_i64(&self) -> bool {
        self.impls._i64 || self.impls._number
    }
    fn should_add_u64(&self) -> bool {
        self.impls._u64 || self.impls._i64 || self.impls._number
    }
    fn should_add_f64(&self) -> bool {
        self.impls._f64 || self.impls._number
    }
    fn should_add_str(&self) -> bool {
        self.impls._str
    }
    fn should_add_bool(&self) -> bool {
        self.impls._bool
    }
    fn should_add_map(&self) -> bool {
        self.impls._map
    }
}

impl Input {
    fn from_attributes(attrs: Attributes, drive_input: DeriveInput) -> Input {
        let impls = if let Some(visit) = &attrs.visit {
            Implementations {
                _bool: visit.contains(&"bool".to_string()),
                _f64: visit.contains(&"f64".to_string()),
                _i64: visit.contains(&"i64".to_string()),
                _map: visit.contains(&"map".to_string()),
                _str: visit.contains(&"str".to_string()),
                _u64: visit.contains(&"i64".to_string()),
                _number: visit.contains(&"number".to_string()),
            }
        } else {
            Implementations::default()
        };
        Input {
            name: drive_input.ident,
            attrs,
            vis: drive_input.vis,
            generics: drive_input.generics,
            data: drive_input.data,
            impls,
        }
    }
}

#[proc_macro_derive(Deserialize, attributes(act))]
pub fn deserialize_derive(tokens: TokenStream) -> TokenStream {
    parse_input(tokens)
        .map(impl_deserializer)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
