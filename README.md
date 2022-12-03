# encoded

Rust crate to convert character encodings at compile time.

## install

```sh
cargo add --git https://github.com/p4ken/encoded
```

## usage

```rs
let bytes = encoded::shift_jis!("漢字");
assert_eq!(bytes, b"\x8a\xbf\x8e\x9a");
```
