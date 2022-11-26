use std::vec::Vec;

use proc_macro::{TokenStream, TokenTree, Literal};

#[proc_macro]
pub fn shift_jis(tokens: TokenStream) -> TokenStream {
    // tmp clone
    let tokens_vec = tokens.clone().into_iter().collect::<Vec<_>>();
    let literal = match tokens_vec.as_slice() {
        [TokenTree::Literal(x)] => x,
        [_] => panic!("Expected literal."), //return syn::Error::new(x.span().into(), "expected Literal").to_compile_error().into(),
        _ => panic!("Expected single token."),
    };
    // literal.
    dbg!(&literal);

    // let utf8 = input.to_string();
    // let (sjis, _, fail) =encoding_rs::SHIFT_JIS.encode(&utf8);
    // assert!(!fail);
    // dbg!(sjis);

    [TokenTree::Literal(Literal::byte_string(&[147u8, 250, 150, 123, 140, 234]))].into_iter().collect::<TokenStream>()
}
