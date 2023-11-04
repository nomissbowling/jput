#![doc(html_root_url = "https://docs.rs/jput/0.1.2")]
//! puts and putc on unicode-width align for Rust
//!

use std::error::Error;
use std::fmt::Write;
use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};
use colored::{Colorize, Color, ColoredString};

/// sput padding
pub fn sput_pad(p: bool, n: i32, w: i32) -> Result<String, Box<dyn Error>> {
  let mut o = String::new();
  let l = if n < 0 { if p {0} else {-n-w} } else { if p {n-w} else {0} };
  for _ in 0..l { o.write_char(' ')?; }
  Ok(o)
}

/// jput padding
pub fn jput_pad(p: bool, n: i32, w: i32) -> Result<(), Box<dyn Error>> {
  print!("{}", sput_pad(p, n, w)?);
  Ok(())
}

/// sputc char
pub fn sputc(c: char, n: i32) -> Result<String, Box<dyn Error>> {
  let mut o = String::new();
  let w = c.width().ok_or("width")? as i32; // width_cjk()
  o.write_fmt(format_args!("{}{}{}",
    sput_pad(true, n, w)?, c, sput_pad(false, n, w)?))?;
  Ok(o)
}

/// jputc char
pub fn jputc(c: char, n: i32) -> Result<(), Box<dyn Error>> {
  print!("{}", sputc(c, n)?);
  Ok(())
}

/// jputc char colored
pub fn jputc_colored(c: char, n: i32, k: Color) -> Result<(), Box<dyn Error>> {
  jputs_colored(&format!("{}", c).color(k), n)
}

/// sputs String
pub fn sputs(s: &String, n: i32) -> Result<String, Box<dyn Error>> {
  let mut o = String::new();
  let w = s.width() as i32; // width_cjk()
  o.write_fmt(format_args!("{}{}{}",
    sput_pad(true, n, w)?, s, sput_pad(false, n, w)?))?;
  Ok(o)
}

/// jputs String
pub fn jputs(s: &String, n: i32) -> Result<(), Box<dyn Error>> {
  print!("{}", sputs(s, n)?);
  Ok(())
}

/// sputs ColoredString
pub fn sputs_colored(s: &ColoredString, n: i32)
  -> Result<String, Box<dyn Error>> {
  let mut o = String::new();
  let w = s.width() as i32; // width_cjk()
  o.write_fmt(format_args!("{}{}{}",
    sput_pad(true, n, w)?, s.to_string(), sput_pad(false, n, w)?))?;
  Ok(o)
}

/// jputs ColoredString
pub fn jputs_colored(s: &ColoredString, n: i32) -> Result<(), Box<dyn Error>> {
  let w = s.width() as i32; // width_cjk()
  jput_pad(true, n, w)?;
  print!("{}", s);
  jput_pad(false, n, w)?;
  Ok(())
}

/// jprint
#[macro_export]
macro_rules! jprint {
  ($n: expr, $fmt: expr) => {
    jputs(&format!($fmt), $n)
  };
  ($n: expr, $fmt: expr, $($p: expr), +) => {
    jputs(&format!($fmt, $($p), +), $n)
  };
}
// use jprint;

/// jprint colored
#[macro_export]
macro_rules! jprint_colored {
  ($c: expr, $n: expr, $fmt: expr) => {
    jputs_colored(&format!($fmt).color($c), $n)
  };
  ($c: expr, $n: expr, $fmt: expr, $($p: expr), +) => {
    jputs_colored(&format!($fmt, $($p), +).color($c), $n)
  };
}
// use jprint_colored;

/// tests
#[cfg(test)]
mod tests {
  use super::*;

  /// test s
  macro_rules! test_s {
    ($t: expr, $s: expr) => { println!("L{}R", $s); assert_eq!($t, $s); };
  }

  /// test j
  macro_rules! test_j {
    ($j: expr) => { print!("L"); assert_eq!((), $j); println!("R"); };
  }

  /// test macro
  macro_rules! test_m {
    ($t: expr, $s: ident, $p: expr, $q: expr, $j: ident) => {
      test_s!($t, $s($p, $q).unwrap());
      test_j!($j($p, $q).unwrap());
    };
  }

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn jput_tests() {
    let s = vec![
      "01234567",
      "０１２３",
      "あ\\ゐゑ",
      "をん",
      "aヰ ヱz"
    ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    test_m!("01234567", sputs, &s[0], -8, jputs);
    test_m!("あ      ", sputc, 'あ', -8, jputc);
    test_m!("      ん", sputc, 'ん', 8, jputc);
    test_m!("あ\\ゐゑ ", sputs, &s[2], -8, jputs);
    test_m!("    をん", sputs, &s[3], 8, jputs);
    test_m!("aヰ ヱz ", sputs, &s[4], -8, jputs);
    test_m!(" aヰ ヱz", sputs, &s[4], 8, jputs);
    test_m!("０１２３", sputs, &s[1], 8, jputs);
    test_m!("\u{1b}[33mあ\\ゐゑ\u{1b}[0m ",
      sputs_colored, &s[2].color(Color::Yellow), -8, jputs_colored);
  }
}
