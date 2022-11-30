# encoded

Macro in rust that encodes characters as compile time.

## install

```sh
cargo add --git https://github.com/p4ken/encoded
```

## usage

```rs
use encoded::shift_jis;

let kanji_sjis = shift_jis!("漢字");
```
