use std::{str::FromStr, vec::Vec};

use litrs::{CharLit, StringLit};
use proc_macro::{Literal, Span, TokenStream, TokenTree};

/// Encode a string literal into Shift_JIS bytes array.
///
/// # Examples
///
/// ```
/// let kanji = encoded::shift_jis!("漢字");
/// assert_eq!(kanji, b"\x8a\xbf\x8e\x9a");
/// ```
///
/// Fail to compile if there is unmappable characters.
///
/// ```compile_fail
/// let ougai = encoded::shift_jis!('鷗');
/// //                              ^^^^
/// ```
#[proc_macro]
pub fn shift_jis(tokens: TokenStream) -> TokenStream {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    let literal = match &tokens[..] {
        [TokenTree::Literal(x)] => x,
        [x] => return error(x.span(), "argument must be a literal"),
        [..] => return error(Span::call_site(), "shift_jis! takes 1 argument"),
    };

    let utf8 = if let Ok(x) = StringLit::try_from(literal) {
        x.value().to_owned()
    } else if let Ok(x) = CharLit::try_from(literal) {
        x.value().to_string()
    } else {
        return error(literal.span(), "literal must be a string or a char");
    };

    let (sjis, _, fail) = encoding_rs::SHIFT_JIS.encode(&utf8);
    if fail {
        return error(
            literal.span(),
            "some characters couldn't convert to Shift_JIS",
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
