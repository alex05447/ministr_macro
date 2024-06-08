use {
    proc_macro::TokenStream,
    proc_macro2::{Literal as Literal2, TokenStream as TokenStream2, TokenTree as TokenTree2},
    quote::quote,
};

pub(crate) fn nestr_impl(item: TokenStream2, owned: bool) -> TokenStream {
    let macro_name = if owned { "nestring" } else { "nestr" };

    let mut iter = item.into_iter();

    let string_tt = iter.next().expect(&format!(
        "`{}` macro takes one non-empty quoted string literal - none were provided",
        macro_name
    ));

    let result = match string_tt {
        TokenTree2::Literal(string_lit) => {
            // At least [" "].
            let orig_string = string_lit.to_string();
            assert!(
                orig_string.len() >= 3,
                "`{}` macro takes one non-empty quoted string literal - `{}` was provided",
                macro_name,
                orig_string
            );

            // Trim quotes: ["asdf"] -> [asdf].
            if let Some(no_prefix_string) = orig_string.strip_prefix("\"") {
                if let Some(_no_suffix_string) = no_prefix_string.strip_suffix("\"") {
                    let string_lit: Literal2 = string_lit.into();

                    if owned {
                        TokenStream::from(quote!(
                            unsafe { ministr::NonEmptyString::from_unchecked(#string_lit) }
                        ))
                    } else {
                        TokenStream::from(quote!(
                            unsafe { ministr::NonEmptyStr::new_unchecked(#string_lit) }
                        ))
                    }
                } else {
                    panic!("`{}` macro takes one non-empty quoted string literal - `{}` does not end with a quote", macro_name, orig_string);
                }
            } else {
                panic!("`{}` macro takes one non-empty quoted string literal - `{}` does not start with a quote", macro_name, orig_string);
            }
        }

        TokenTree2::Group(group) => nestr_impl(group.stream(), owned),

        TokenTree2::Ident(ident) => {
            panic!(
                "`{}` macro takes one non-empty quoted string literal - ident `{}` was provided",
                macro_name, ident
            );
        }

        TokenTree2::Punct(punct) => {
            panic!(
                "`{}` macro takes one non-empty quoted string literal - punct `{}` was provided",
                macro_name, punct
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
