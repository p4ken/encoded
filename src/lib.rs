//! Convert character encodings at compile time.
//!
//! Fucntion-style macros convert from a string literal (UTF-8)
//! into byte string literal (any character encoding).
//!
//! It reduces processing costs and binary size
//! compared to runtime conversion.
//!
//! # Examples
//!
//! ```
//! let bytes = encoded::shift_jis!("漢字");
//! assert_eq!(bytes, b"\x8a\xbf\x8e\x9a");
//! ```
//!
//! The converted byte string can be used with [std::io::Write]:
//!
//! ```
//! use std::io::{Cursor, Write, Result};
//!
//! let mut buff = std::io::Cursor::new(Vec::new());
//! buff.write_all(encoded::shift_jis!("漢字"));
//! assert_eq!(buff.get_ref(), b"\x8a\xbf\x8e\x9a");
//! ```
//!
//! # Errors
//!
//! Argument must be a literal:
//!
//! ```compile_fail
//! let kanji = "漢字";
//! let bytes = encoded::shift_jis!(kanji);
//! //                              ^^^^^
//! ```
//!
//! If there were unmappable characters, it also be a compile error:
//!
//! ```compile_fail
//! let bytes = encoded::shift_jis!("鷗");
//! //                              ^^^^
//! ```

use std::{str::FromStr, vec::Vec};

use encoding_rs::Encoding;
use litrs::{CharLit, StringLit};
use proc_macro::{Literal, Span, TokenStream, TokenTree};

/// # Examples
///
/// ```
/// let bytes = encoded::shift_jis!("漢字");
/// assert_eq!(bytes, b"\x8a\xbf\x8e\x9a");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn shift_jis(tokens: TokenStream) -> TokenStream {
    convert(tokens, encoding_rs::SHIFT_JIS)
}

fn convert(tokens: TokenStream, encoding: &'static Encoding) -> TokenStream {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    let literal = match &tokens[..] {
        [TokenTree::Literal(x)] => x,
        [x] => return error(x.span(), "argument must be a literal"),
        [..] => {
            let message = &format!("{}! takes 1 argument", encoding.name().to_lowercase());
            return error(Span::call_site(), message);
        }
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
        let message = format!("{} cannot convert to {}", utf8, encoding.name());
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
