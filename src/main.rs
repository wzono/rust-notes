use std::env;
mod fibonacci;
mod temperature_convert;

fn main() {
  let args = env::args().collect::<Vec<String>>();

  if args.len() < 2 {
    return println!("[error] no key was given.")
  }
  
  let note_key = args[1].parse::<usize>().unwrap();

  match note_key {
    1 => fibonacci::start(),
    2 => temperature_convert::start(),
    _ => println!("[warn] give a right key for note start.")
  }
}
