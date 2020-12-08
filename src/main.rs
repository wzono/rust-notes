mod modules;

use crate::modules::*;

use std::env;
fn main() {
  let args = env::args().collect::<Vec<String>>();

  if args.len() < 2 {
    return println!("[error] no key was given.");
  }

  let note_key = args[1].parse::<usize>().unwrap();

  match note_key {
    1 => fibonacci::start(),
    2 => temperature_convert::start(),
    3 => own::start(),
    4 => struct_test::start(),
    5 => enum_test::start(),
    6 => match_test::start(),
    7 => module_test::start(),
    _ => println!("[warn] give a right key for note start."),
  }
}
