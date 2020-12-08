// use std::collections::HashMap;

// pub fn start() {
//   let map : HashMap<&str, String> = HashMap::new();

//   take(&map);

//   println!("{}", map.len());
// }

// fn take(map: &HashMap<&str, String>) {
//   // do somthing
// }

pub fn start() {

  let x = 0;
  let y = x;

  println!("{}", y);

  // 可行
  let mut a = String::from("123");

  println!("{}", a);

  let c = &mut a;

  println!("{}", c);

  let b = &a;

  println!("{}", b);


  // 可行

  let d = &mut a;

  println!("{}", d);

  let e = &mut a;

  println!("{}", e);

  println!("{}", e);

  // 不可行

  // let f = &mut a;
  // let h = &mut a;

  // println!("{}, {}", f, h);

  let s = "Love and Peace";

  let word = first_word(s);

  // let z = [1,2,3];

  // let slice = &z[1..];

  println!("{}", s);

  println!("{}", word);
}

fn first_word(s: &str) -> &str {
  
  
  for (i,char) in s.chars().enumerate() {
    if char == ' ' {
      return &s[..i];
    }
  };

  return &s;
}
