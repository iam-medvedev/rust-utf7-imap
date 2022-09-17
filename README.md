# utf7-imap

[![Test Status](https://github.com/iam-medvedev/rust-utf7-imap/workflows/test/badge.svg?event=push)](https://github.com/iam-medvedev/rust-utf7-imap/actions)
[![Crate](https://img.shields.io/crates/v/utf7-imap.svg)](https://crates.io/crates/utf7-imap)
[![API](https://docs.rs/utf7-imap/badge.svg)](https://docs.rs/utf7-imap)

A Rust library for encoding and decoding [UTF-7](https://datatracker.ietf.org/doc/html/rfc2152) string as defined by the [IMAP](https://datatracker.ietf.org/doc/html/rfc3501) standard in [RFC 3501 (#5.1.3)](https://datatracker.ietf.org/doc/html/rfc3501#section-5.1.3).

Idea is based on Python [mutf7](https://github.com/cheshire-mouse/mutf7) library.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
utf7-imap = "0.3.1"
```

### Encode

Encode UTF-7 IMAP mailbox name

<https://datatracker.ietf.org/doc/html/rfc3501#section-5.1.3>

```rust
use utf7_imap::encode_utf7_imap;

let test_string = String::from("Отправленные");
assert_eq!(utf7_imap::encode_utf7_imap(test_string), "&BB4EQgQ,BEAEMAQyBDsENQQ9BD0ESwQ1-");
```

### Decode

Decode UTF-7 IMAP mailbox name
<https://datatracker.ietf.org/doc/html/rfc3501#section-5.1.3>

```rust
use utf7_imap::decode_utf7_imap;
let test_string = String::from("&BB4EQgQ,BEAEMAQyBDsENQQ9BD0ESwQ1-");
assert_eq!(decode_utf7_imap(test_string), "Отправленные");
```

# License

utf7-imap is [MIT licensed](LICENSE).
