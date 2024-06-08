//! Exports some utility proc macros for strings.

mod ne_str;
mod str_hash;

use proc_macro::TokenStream;

/// Hashes the string literal to a `u64` using the Rust's [`default hasher`](std::collections::hash_map::DefaultHasher) (i.e. one used in the [`HashMap`](std::collections::HashMap)).
#[proc_macro]
pub fn str_hash_default(item: TokenStream) -> TokenStream {
    str_hash::str_hash_impl(item, ministr::str_hash_default, "str_hash_default")
}

/// Hashes the string literal to a `u32` using the FNV1a (32b) hash.
#[proc_macro]
pub fn str_hash_fnv1a(item: TokenStream) -> TokenStream {
    str_hash::str_hash_impl(item, ministr::str_hash_fnv1a, "str_hash_fnv1a")
}

/// Hashes the string literal to a `u64` using the FNV1a (64b) hash.
#[proc_macro]
pub fn str_hash_fnv1a_64(item: TokenStream) -> TokenStream {
    str_hash::str_hash_impl(item, ministr::str_hash_fnv1a_64, "str_hash_fnv1a_64")
}

/// Creates a [`NonEmptyStr`](ministr::NonEmptyStr) from a compile-time checked non-empty string literal.
#[proc_macro]
pub fn nestr(item: TokenStream) -> TokenStream {
    ne_str::nestr_impl(item.into(), false)
}

/// Creates a [`NonEmptyString`](ministr::NonEmptyString) from a compile-time checked non-empty string literal.
#[proc_macro]
pub fn nestring(item: TokenStream) -> TokenStream {
    ne_str::nestr_impl(item.into(), true)
}
