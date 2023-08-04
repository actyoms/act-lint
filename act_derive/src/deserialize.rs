use quote::{format_ident, quote};

use crate::{expecting, visitors, Input};

pub(crate) fn impl_deserializer(input: Input) -> proc_macro2::TokenStream {
    let attr = &input.attrs;
    let name = &input.name;
    let visitor;
    let impl_visitor;
    if let Some(visitor_name) = &attr.visitor {
        visitor = format_ident!("{}", visitor_name);
        impl_visitor = quote! {};
    } else {
        visitor = format_ident!("{}Visitor", name);
        impl_visitor = visitors::impl_visitor(name, &visitor, &input);
    };
    let expecting = expecting::impl_expecting(&input);
    quote! {
        use act_trait as _;

        #expecting

        #impl_visitor

        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(#visitor)
            }
        }
    }
}
