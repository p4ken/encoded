//! Macro in rust that encodes characters as compile time.
//! 
//! 

use std::{str::FromStr, vec::Vec};

use litrs::StringLit;
use proc_macro::{Literal, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn shift_jis(tokens: TokenStream) -> TokenStream {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    let literal = match &tokens[..] {
        [TokenTree::Literal(x)] => x,
        [x] => return error(x.span(), "argument must be a string literal"),
        [..] => return error(Span::call_site(), "shift_jis! takes 1 argument"),
    };

    let utf8 = match StringLit::try_from(literal) {
        Ok(x) => x,
        Err(_) => return error(literal.span(), "argument must be a string literal"),
    };

    let (sjis, _, fail) = encoding_rs::SHIFT_JIS.encode(utf8.value());
    if fail {
        return error(
            literal.span(),
            "some characters couldn't convert to SHIFT_JIS",
        );
    }

    dbg!([TokenTree::Literal(Literal::byte_string(&sjis[..]))]);
    [TokenTree::Literal(Literal::byte_string(&sjis[..]))]
        .into_iter()
        .collect::<TokenStream>()
}

fn error(span: Span, message: &str) -> TokenStream {
    TokenStream::from_str(&format!("compile_error!(\"{}\")", message))
        .unwrap()
        .into_iter()
        .map(|mut t| {
            t.set_span(span);
            t
        })
        .collect()
}
