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
