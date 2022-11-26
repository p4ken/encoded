use std::{str::FromStr, vec::Vec};

use proc_macro::{Literal, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn shift_jis(tokens: TokenStream) -> TokenStream {
    // dbg!(&tokens);

    let tokens = tokens.into_iter().collect::<Vec<_>>();
    let literal = match &tokens[..] {
        [TokenTree::Literal(x)] => x,
        [x] => return error(x.span(), "argument must be a string literal"),
        _ => return error(Span::call_site(), "shift_jis! takes 1 argument2"),
    };
    dbg!(&literal);

    let literal = literal.to_string();
    dbg!(&literal);
    // 最初と最後が " かどうか？
    // let utf8 = match literal.chars() {
    //     [""] => panic!("yeaaah!"),
    //     _ => literal,
    // };

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
    let tokens = TokenStream::from_str(&format!("compile_error!(\"{}\")", message)).unwrap();
    tokens
        .into_iter()
        .map(|mut t| {
            t.set_span(span);
            t
        })
        .collect()
}
