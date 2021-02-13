# mime2ext

[![Crates.io](https://img.shields.io/crates/v/mime2ext.svg)](https://crates.io/crates/mime2ext)
[![API reference](https://docs.rs/mime2ext/badge.svg)](https://docs.rs/mime2ext/)

A simple crate to look up a file extension for a mime type.

It embeds part of the [`mime-db`](https://github.com/jshttp/mime-db) database, packed efficiently into around 20 KiB. There are no runtime dependencies, and it's `no_std`-compatible.

## Example

```rust
use mime2ext::mime2ext;

assert_eq!(mime2ext("image/png"), Some("png"));
assert_eq!(mime2ext("application/octet-stream"), Some("bin"));
assert_eq!(mime2ext("text/html; charset=UTF-8"), Some("html"));
assert_eq!(mime2ext("nonexistent/mimetype"), None);
assert_eq!(mime2ext("invalid-mimetype"), None);
```

## Interoperability with `mime`

[`mime`](https://docs.rs/mime/)'s [`Mime`](https://docs.rs/mime/0.3.16/mime/struct.Mime.html) type is supported through its implementation of `AsRef<str>`, without any dependency on the crate:

```rust
use mime::{Mime, TEXT_PLAIN};
use mime2ext::mime2ext;

assert_eq!(mime2ext(TEXT_PLAIN), Some("txt"));
let mime: Mime = "text/xml; charset=latin1".parse()?;
assert_eq!(mime2ext(&mime), Some("xml"));
```

## Versioning

`mime2ext` includes a fixed version of `mime-db`. A new version of `mime2ext` has to be released for each new version of `mime-db`.

The currently used `mime-db` version is **1.45.0**.

## License

Both `mime2ext` and `mime-db` are licensed under the MIT license. See `LICENSE` and `mime-db/LICENSE`.

## See also

- [`mime_guess`](https://crates.io/crates/mime_guess), which converts in the opposite direction. It inspired the design of this crate.
