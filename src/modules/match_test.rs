pub fn start() {
  let some_u8_value = Some(0u8);
  match some_u8_value {
    Some(v) => println!("{}", v),
    _ => (),
  }

  if let Some(v) = some_u8_value {
    println!("{}", v);
  } else {
    // do nothing
  }
}
