use std::{str::FromStr, vec::Vec};

use encoding_rs::Encoding;
use litrs::{CharLit, StringLit};
use proc_macro::{Literal, Span, TokenStream, TokenTree};

pub fn convert(tokens: TokenStream, encoding: &'static Encoding) -> TokenStream {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    let literal = match &tokens[..] {
        [TokenTree::Literal(x)] => x,
        [x] => return error(x.span(), "argument must be a literal"),
        [..] => return error(Span::call_site(), "macro takes 1 argument"),
    };

    let utf8 = if let Ok(x) = StringLit::try_from(literal) {
        x.value().to_owned()
    } else if let Ok(x) = CharLit::try_from(literal) {
        x.value().to_string()
    } else {
        return error(literal.span(), "literal must be a string");
    };

    let (sjis, _, fail) = encoding.encode(&utf8);
    if fail {
        let message = format!("{} cannot be converted to {}", utf8, encoding.name());
        return error(literal.span(), &message);
    }

    // dbg!([TokenTree::Literal(Literal::byte_string(&sjis[..]))]);
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
