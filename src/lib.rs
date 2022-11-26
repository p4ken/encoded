use std::vec::Vec;

use proc_macro::{Literal, Span, TokenStream, TokenTree};
use quote::quote_spanned;

#[proc_macro]
pub fn shift_jis(tokens: TokenStream) -> TokenStream {
    // dbg!(&tokens);

    let tokens = tokens.into_iter().collect::<Vec<_>>();
    let literal = match &tokens[..] {
        [] => return error(Span::call_site(), "An argument needed."),
        [TokenTree::Literal(x)] => x,
        [x] => return error(x.span(), "Expected literal."),
        [_, x, ..] => return error(x.span(), "Too many arguments."),
    };
    dbg!(&literal);

    // let utf8 = input.to_string();
    // let (sjis, _, fail) =encoding_rs::SHIFT_JIS.encode(&utf8);
    // assert!(!fail);
    // dbg!(sjis);

    [TokenTree::Literal(Literal::byte_string(&[
        147u8, 250, 150, 123, 140, 234,
    ]))]
    .into_iter()
    .collect::<TokenStream>()
}

fn error(span: Span, message: &str) -> TokenStream {
    quote_spanned!(span.into() => compile_error!(#message)).into()
}
