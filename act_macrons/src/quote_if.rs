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
macro_rules! quote_if {
    ($check:ident, {$($tt:tt)*}) => {
        if $check {
            quote::quote! {
                /// from act_macrons::quote_if
                $($tt)*
            }
        } else {
            quote::quote! {}
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn return_tokens() {
        let token_stream = quote_if!(true, {
            println!("Hello, World!");
        });
        assert_eq!(
            token_stream.to_string(),
            r#"# [doc = r" from act_macrons::quote_if"] println ! ("Hello, World!") ;"#
        );
    }

    #[test]
    fn return_nothing() {
        let token_stream = quote_if!(false, {
            println!("Hello, World!");
        });
        assert_eq!(token_stream.to_string(), "");
    }
}
