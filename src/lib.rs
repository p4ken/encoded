//! Convert character encodings at compile time
//! to reduce runtime processing costs and built binary size.
//!
//! Fucntion-style macros convert from a string literal (UTF-8)
//! into byte string literal (non UTF-8).
//!
//! # Examples
//!
//! ```
//! let bytes = encoded::shift_jis!("漢字");
//! assert_eq!(bytes, b"\x8a\xbf\x8e\x9a");
//! ```
//!
//! Can be used with [std::io::Write]:
//!
//! ```
//! use std::io::{Cursor, Write};
//!
//! let mut buff = Cursor::new(Vec::new());
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
//! Any unmappable characters result a compile error:
//!
//! ```compile_fail
//! let bytes = encoded::shift_jis!("鷗外");
//! //                              ^^^^^^
//! ```

use proc_macro::TokenStream;

mod inner;


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
/// let bytes = encoded::iso_8859_6!("الْأَبْجَدِيَّة");
/// assert_eq!(bytes, b"\xc7\xe4\xf2\xc3\xee\xc8\xf2\xcc\xee\xcf\xf0\xea\xee\xf1\xc9");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn iso_8859_6(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_6)
}

/// # Examples
///
/// ```
/// let bytes = encoded::iso_8859_7!("αλφάβητο");
/// assert_eq!(bytes, b"\xe1\xeb\xf6\xdc\xe2\xe7\xf4\xef");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn iso_8859_7(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_7)
}

/// # Examples
///
/// ```
/// let bytes = encoded::iso_8859_8!("אלפבית");
/// assert_eq!(bytes, b"\xe0\xec\xf4\xe1\xe9\xfa");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn iso_8859_8(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_8)
}

/// # Examples
///
/// ```
/// let bytes = encoded::iso_8859_15!("œufs");
/// assert_eq!(bytes, b"\xbdufs");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn iso_8859_15(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_15)
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
/// let bytes = encoded::koi8_u!("кирилиця");
/// assert_eq!(bytes, b"\xcb\xc9\xd2\xc9\xcc\xc9\xc3\xd1");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn koi8_u(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::KOI8_U)
}

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
/// let bytes = encoded::windows_1251!("Кириллица");
/// assert_eq!(bytes, b"\xca\xe8\xf0\xe8\xeb\xeb\xe8\xf6\xe0");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn windows_1251(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1251)
}

/// # Examples
///
/// ```
/// let bytes = encoded::windows_1252!("œufs");
/// assert_eq!(bytes, b"\x9cufs");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn windows_1252(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1252)
}

/// # Examples
///
/// ```
/// let bytes = encoded::windows_1256!("الْأَبْجَدِيَّة");
/// assert_eq!(bytes, b"\xc7\xe1\xfa\xc3\xf3\xc8\xfa\xcc\xf3\xcf\xf6\xed\xf3\xf8\xc9");
/// ```
///
/// See also [crate].
#[proc_macro]
pub fn windows_1256(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1256)
}
