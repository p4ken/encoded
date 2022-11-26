# encoded

Function-style macro that encodes characters at compile time in Rust.

## install

```sh
cargo add --git https://github.com/p4ken/encoded
```

## usage

```rs
use encoded::shift_jis;

let kanji_sjis = shift_jis!("漢字");
```
