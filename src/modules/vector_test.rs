
pub fn start() {
  let s = String::from("1");
  let mut v = vec![s, String::from("2")];

  // 取数方式1

  v.push(String::from("4"));

  let first_number_2 = v.get(0);

  if let Some(d) = first_number_2 {
    println!("{}", d);
  }


  
}