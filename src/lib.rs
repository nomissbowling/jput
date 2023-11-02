#![doc(html_root_url = "https://docs.rs/jput/0.1.0")]
//! puts and putc on unicode-width align for Rust
//!

use std::error::Error;
use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};

pub fn jput_pad(p: bool, n: i32, w: i32) {
  let l = if n < 0 { if p {0} else {-n-w} } else { if p {n-w} else {0} };
  for _ in 0..l { print!(" "); }
}

pub fn jputc(c: char, n: i32) -> Result<(), Box<dyn Error>> {
  let w = c.width().ok_or("width")? as i32; // width_cjk()
  jput_pad(true, n, w);
  print!("{}", c);
  jput_pad(false, n, w);
  Ok(())
}

pub fn jputs(s: &String, n: i32) -> Result<(), Box<dyn Error>> {
  let w = s.width() as i32; // width_cjk()
  jput_pad(true, n, w);
  print!("{}", s);
  jput_pad(false, n, w);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  // [-- --nocapture] [-- --show-output]
  #[test]
  fn jput_tests() {
    let s = vec![
      "01234567",
      "０１２３",
      "あゐゑ",
      "をん",
      "aヰヱz"
    ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
    print!("L"); assert_eq!(jputs(&s[0], -8).unwrap(), ()); println!("R");
    print!("L"); assert_eq!(jputc('あ', -8).unwrap(), ()); println!("R");
    print!("L"); assert_eq!(jputc('ん', 8).unwrap(), ()); println!("R");
    print!("L"); assert_eq!(jputs(&s[2], -8).unwrap(), ()); println!("R");
    print!("L"); assert_eq!(jputs(&s[3], 8).unwrap(), ()); println!("R");
    print!("L"); assert_eq!(jputs(&s[4], -8).unwrap(), ()); println!("R");
    print!("L"); assert_eq!(jputs(&s[4], 8).unwrap(), ()); println!("R");
    print!("L"); assert_eq!(jputs(&s[1], 8).unwrap(), ()); println!("R");
  }
}
