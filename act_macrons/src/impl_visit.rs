/// [quote!] a token stream if condition is true, otherwise [quote!] nothing.
/// ```
/// /// Usage.
/// use act_macrons::quote_if;
///
/// let token_stream = quote_if!(true, {
///    println!("Hello, World!");
/// });
/// # assert_eq!(token_stream.to_string(), r#"# [doc = r" from act_macrons::quote_if"] println ! ("Hello, World!") ;"#);
/// ```
#[macro_export]
macro_rules! impl_visit {
    ($name:ident, $impl_method:ident, $from_method:ident, $arg:tt, $unexp:tt) => {
        quote::quote! {
            fn #$impl_method<E>(self, v: #$arg) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                #$name::#$from_method(v)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::#$unexp(v), &self))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use proc_macro2::Ident;
    use quote::format_ident;

    #[test]
    #[allow(unused_variables)]
    fn return_tokens() {
        let ident = Ident::new("StrNumBool", proc_macro2::Span::call_site());
        let impl_method = format_ident!("impl_visit_{}", "str");
        let from_method = format_ident!("from_{}", "str");
        let arg = format!("&str");
        let unexp = format_ident!("Str");
        let actual = impl_visit!(ident, impl_method, from_method, arg, unexp).to_string();
        let expected = r#"fn impl_visit_str < E > (self , v : "&str") -> Result < Self :: Value , E > where E : serde :: de :: Error , { StrNumBool :: from_str (v) . map_err (| _ | E :: invalid_value (serde :: de :: Unexpected :: Str (v) , & self)) }"#.to_string();
        assert_eq!(actual, expected)
    }
}
