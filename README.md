# encoded

Rust macros converting character encodings.

## install

```sh
cargo add encoded
```

## usage

```rs
const BYTES: &[u8] = encoded::shift_jis!("漢字");
assert_eq!(BYTES, b"\x8a\xbf\x8e\x9a");
```

For more information, see the [documentation](https://docs.rs/encoded/).
