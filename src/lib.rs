//! Convert character encodings at compile time.
//!
//! Fucntion-style macros convert from a string literal (UTF-8)
//! into byte string literal (any character encoding).
//!
//! It helps reducing runtime processing costs and binary size.
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

use proc_macro::TokenStream;

mod inner;

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
    inner::convert(tokens, encoding_rs::SHIFT_JIS)
}


/// # Examples
///
/// ```
/// let bytes = encoded::big5!("漢字");
/// assert_eq!(bytes, b"\xba\x7e\xa6\x72");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn big5(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::BIG5)
}

/// # Examples
///
/// ```
/// let bytes = encoded::gbk!("漢字");
/// assert_eq!(bytes, b"\x9d\x68\xd7\xd6");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn gbk(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::GBK)
}

/// # Examples
///
/// ```
/// let bytes = encoded::euc_kr!("한글");
/// assert_eq!(bytes, b"\xc7\xd1\xb1\xdb");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn euc_kr(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::EUC_KR)
}

/// # Examples
///
/// ```
/// let bytes = encoded::koi8_r!("Кириллица");
/// assert_eq!(bytes, b"\xeb\xc9\xd2\xc9\xcc\xcc\xc9\xc3\xc1");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn koi8_r(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::KOI8_R)
}

/// # Examples
///
/// ```
/// let bytes = encoded::windows_1251!("Кириллица");
/// assert_eq!(bytes, b"\xca\xe8\xf0\xe8\xeb\xeb\xe8\xf6\xe0");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn windows_1251(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1251)
}
