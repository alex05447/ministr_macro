use proc_macro::{Literal, TokenStream, TokenTree};

pub(crate) trait ToLiteral {
    fn to_literal(&self) -> Literal;
}

impl ToLiteral for u64 {
    fn to_literal(&self) -> Literal {
        Literal::u64_unsuffixed(*self)
    }
}

impl ToLiteral for u32 {
    fn to_literal(&self) -> Literal {
        Literal::u32_unsuffixed(*self)
    }
}

pub(crate) fn str_hash_impl<H: ToLiteral>(
    item: TokenStream,
    hash: fn(&str) -> H,
    macro_name: &str,
) -> TokenStream {
    let mut iter = item.into_iter();

    let string = iter.next().expect(&format!(
        "`{}` macro takes one non-empty quoted string literal - none were provided",
        macro_name
    ));

    let result = match string {
        TokenTree::Literal(string_lit) => {
            // At least [" "].
            let orig_string = string_lit.to_string();
            assert!(
                orig_string.len() >= 3,
                "`{}` macro takes one non-empty quoted string literal - `{}` was provided",
                orig_string,
                macro_name
            );

            // Trim quotes: ["asdf"] -> [asdf].
            if let Some(string) = orig_string.strip_prefix("\"") {
                if let Some(string) = string.strip_suffix("\"") {
                    let hash_lit = hash(&string).to_literal();

                    TokenStream::from(TokenTree::Literal(hash_lit))
                } else {
                    panic!("`{}` macro takes one non-empty quoted string literal - `{}` does not end with a quote", orig_string, macro_name);
                }
            } else {
                panic!("`{}` macro takes one non-empty quoted string literal - `{}` does not start with a quote", orig_string, macro_name);
            }
        }

        TokenTree::Group(group) => str_hash_impl(group.stream(), hash, macro_name),

        TokenTree::Ident(ident) => {
            panic!(
                "`{}` macro takes one non-empty quoted string literal - ident `{}` was provided",
                ident, macro_name
            );
        }

        TokenTree::Punct(punct) => {
            panic!(
                "`{}` macro takes one non-empty quoted string literal - punct `{}` was provided",
                punct, macro_name
            );
        }
    };

    assert!(
        iter.next().is_none(),
        "`{}` macro takes one non-empty quoted string literal - multiple were provided",
        macro_name
    );

    result
}
