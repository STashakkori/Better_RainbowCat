// Next-gen Rainbow Cat
// By: Sina Tashakkori, QVLx Labs

use std::io::{self};

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read input");

  print_rainbow(input.trim().to_string());
}

fn print_rainbow(s: String) {
  let ansi_code = String::from("\x1b[38;5;");
  let mut colored_string = String::new();
  let raw = s.chars();
  let mut count = 0;
  let mut code = 0;
  for (_i,character) in raw.enumerate() {
    if count == 28 { count = 0; } 
    colored_string.push_str(&ansi_code);
    if count == 0 || count == 1 { code = 33 }; // darker blue
    if count == 2 || count == 3 { code = 37 }; // lighter blue
    if count == 4 || count == 5 { code = 51 }; // blue green
    if count == 6 || count == 7 { code = 83 }; // lighter bluegreen
    if count == 8 || count == 9 { code = 154 }; // neon green
    if count == 10 || count == 11 { code = 226 }; // bright yellow
    if count == 12 || count == 13 { code = 214 }; // light orange
    if count == 14 || count == 15 { code = 208 }; // darker orange
    if count == 16 || count == 17 { code = 203 }; // rose
    if count == 18 || count == 19 { code = 162 }; // pink
    if count == 20 || count == 21 { code = 165 }; // magenta
    if count == 22 || count == 23 { code = 129 }; // light purple
    if count == 24 || count == 25 { code = 55 }; // darker purple
    if count == 26 || count == 27 { code = 62 }; // darker purple
		colored_string.push_str(&code.to_string()[..]);
		colored_string.push_str("m");
		colored_string.push_str(&character.to_string());
		colored_string.push_str("\x1b[0m");
    if character != ' ' &&
       character != '\n' &&
       character != '\r' &&
       character != '\t' {
         count = count + 1;
    } 
  }
  println!("{}", colored_string);
}