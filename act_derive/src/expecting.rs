extern crate act_trait;
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;

use crate::Input;

pub(crate) fn impl_expecting(input: &Input) -> TokenStream {
    let name = &input.name;
    if let Some(x) = &input.attrs.expecting {
        let expected = x.to_string();
        quote! {
            use act_trait::Expecting as _;

            impl act_trait::Expecting for #name {
                fn expecting(formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_fmt(format_args!(#expected))
                }
            }
        }
    } else {
        quote! {}
    }
}
