extern crate base64;
extern crate encoding_rs;
extern crate regex;

use encoding_rs::UTF_16BE;
use regex::Regex;

/**
 * Decode UTF-7 IMAP mailbox name
 *
 * https://datatracker.ietf.org/doc/html/rfc3501#section-5.1.3
 * Based on https://github.com/cheshire-mouse/mutf7
 */
pub fn decode_utf7_imap(text: String) -> String {
  let re = Regex::new(r"&[^&-]*-").unwrap();
  let mut result = text.clone();

  for cap in re.captures_iter(&text) {
    let encoded_text = cap.get(0).map_or("", |m| m.as_str());
    println!("{}", encoded_text);
    let decoded_text = decode_utf7_part(String::from(encoded_text));

    result = text.replace(&encoded_text, &decoded_text);
  }

  return result;
}

fn decode_utf7_part(text: String) -> String {
  if text == "&-" {
    return String::from("&");
  }

  let text_mb64 = &text[1..text.len() - 1];
  let mut text_b64 = text_mb64.replace(",", "/");

  while (text_b64.len() % 4) != 0 {
    text_b64 += "=";
  }

  let text_u16 = base64::decode(text_b64).unwrap();
  let (cow, _encoding_used, _had_errors) = UTF_16BE.decode(&text_u16);
  let result = cow.as_ref();

  return String::from(result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn decode_test() {
    let test_string = "&BB4EQgQ,BEAEMAQyBDsENQQ9BD0ESwQ1-";
    assert_eq!(decode_utf7_imap(String::from(test_string)), "Отправленные");
  }
}
