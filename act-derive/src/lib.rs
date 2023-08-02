extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro_derive(UntaggedDeserialize, attributes(visitor))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut visitor = ast.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("visitor") {
            attr.parse_args::<syn::Ident>().ok().map(|ident| quote! { #ident })
        } else {
            None
        }
    });
    let visitor_x_impl = if visitor.is_some() {
        quote! {}
    } else {
        visitor = Some(quote! { VistorX });
        quote! {
            struct VistorX;

            impl<'de> serde::de::Visitor<'de> for VistorX {
                type Value = Env;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_fmt(format_args!(
                        "a string matching {IN_STRING_PATTERN} or map...",
                    ))
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    from_str::<InString>(v)
                        .map(Env::InStringExpression)
                        .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &self))
                }

                fn visit_map<A>(self, _: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
                {
                    todo!()
                }
            }
        }
    };
    let gen = quote! {
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
    gen.into()
}
