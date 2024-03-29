//! A Rust library for encoding and decoding [UTF-7](https://datatracker.ietf.org/doc/html/rfc2152) string as defined by the [IMAP](https://datatracker.ietf.org/doc/html/rfc3501) standard in [RFC 3501 (#5.1.3)](https://datatracker.ietf.org/doc/html/rfc3501#section-5.1.3).
//!
//! Idea is based on Python [mutf7](https://github.com/cheshire-mouse/mutf7) library.

extern crate base64;
extern crate encoding_rs;
extern crate regex;

use encoding_rs::UTF_16BE;
use regex::{Captures, Regex};

/// Encode UTF-7 IMAP mailbox name
///
/// <https://datatracker.ietf.org/doc/html/rfc3501#section-5.1.3>
///
/// # Usage:
///
/// ```
/// use utf7_imap::encode_utf7_imap;
///
/// let test_string = String::from("Отправленные");
/// assert_eq!(utf7_imap::encode_utf7_imap(test_string), "&BB4EQgQ,BEAEMAQyBDsENQQ9BD0ESwQ1-");
/// ```
pub fn encode_utf7_imap(text: String) -> String {
    let mut result = "".to_string();
    let text = text.replace('&', "&-");
    let mut text = text.as_str();
    while !text.is_empty() {
        result = format!("{}{}", result, get_ascii(text));
        text = remove_ascii(text);
        if !text.is_empty() {
            let tmp = get_nonascii(text);
            result = format!("{}{}", result, encode_modified_utf7(tmp.to_string()));
            text = remove_nonascii(text);
        }
    }
    result
}
fn is_ascii_custom(c: u8) -> bool {
    (0x20..=0x7f).contains(&c)
}

fn get_ascii(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if !is_ascii_custom(item) {
            return &s[0..i];
        }
    }
    s
}

fn get_nonascii(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if is_ascii_custom(item) {
            return &s[0..i];
        }
    }
    s
}

fn remove_ascii(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if !is_ascii_custom(item) {
            return &s[i..];
        }
    }
    ""
}

fn remove_nonascii(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if is_ascii_custom(item) {
            return &s[i..];
        }
    }
    ""
}

fn encode_modified_utf7(text: String) -> String {
    let capacity = 2 * text.len();
    let mut input = Vec::with_capacity(capacity);
    let text_u16 = text.encode_utf16();
    for value in text_u16 {
        input.extend_from_slice(&value.to_be_bytes());
    }
    let text_u16 = base64::encode(input);
    let text_u16 = text_u16.trim_end_matches('=');
    let result = text_u16.replace('/', ",");
    format!("&{}-", result)
}

/// Decode UTF-7 IMAP mailbox name
///
/// <https://datatracker.ietf.org/doc/html/rfc3501#section-5.1.3>
///
/// # Usage:
///
/// ```
/// use utf7_imap::decode_utf7_imap;
///
/// let test_string = String::from("&BB4EQgQ,BEAEMAQyBDsENQQ9BD0ESwQ1-");
/// assert_eq!(decode_utf7_imap(test_string), "Отправленные");
/// ```
pub fn decode_utf7_imap(text: String) -> String {
    let pattern = Regex::new(r"&([^-]*)-").unwrap();
    pattern.replace_all(&text, expand).to_string()
}

fn expand(cap: &Captures) -> String {
    if cap.get(1).unwrap().as_str() == "" {
        "&".to_string()
    } else {
        decode_utf7_part(cap.get(0).unwrap().as_str().to_string())
    }
}

fn decode_utf7_part(text: String) -> String {
    if text == "&-" {
        return String::from("&");
    }

    let text_mb64 = &text[1..text.len() - 1];
    let mut text_b64 = text_mb64.replace(',', "/");

    while (text_b64.len() % 4) != 0 {
        text_b64 += "=";
    }

    let text_u16 = base64::decode(text_b64).unwrap();
    let (cow, _encoding_used, _had_errors) = UTF_16BE.decode(&text_u16);
    let result = cow.as_ref();

    String::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encode_test() {
        let test_string = String::from("Отправленные");
        assert_eq!(
            encode_utf7_imap(test_string),
            "&BB4EQgQ,BEAEMAQyBDsENQQ9BD0ESwQ1-"
        );
    }
    #[test]
    fn encode_test_split() {
        let test_string = String::from("Šiukšliadėžė");
        assert_eq!(encode_utf7_imap(test_string), "&AWA-iuk&AWE-liad&ARcBfgEX-")
    }

    #[test]
    fn encode_consecutive_accents() {
        let test_string = String::from("théâtre");
        assert_eq!(encode_utf7_imap(test_string), "th&AOkA4g-tre")
    }

    #[test]
    fn decode_test() {
        let test_string = String::from("&BB4EQgQ,BEAEMAQyBDsENQQ9BD0ESwQ1-");
        assert_eq!(decode_utf7_imap(test_string), "Отправленные");
    }
    #[test]
    fn decode_test_split() {
        // input string with utf7 encoded bits being separated by ascii
        let test_string = String::from("&AWA-iuk&AWE-liad&ARcBfgEX-");
        assert_eq!(decode_utf7_imap(test_string), "Šiukšliadėžė")
    }

    #[test]
    fn decode_consecutive_accents() {
        let test_string = String::from("th&AOkA4g-tre");
        assert_eq!(decode_utf7_imap(test_string), "théâtre")
    }

    use proptest::prelude::*;
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn fuzzy_dec_enc_check(s in "\\PC*") {
            assert_eq!(decode_utf7_imap(encode_utf7_imap(s.clone())),s)
        }
    }
}
