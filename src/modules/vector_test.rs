pub fn start() {
  let s = String::from("1");
  let mut v = vec![s, String::from("2")];

  // 取数方式1

  v.push(String::from("4"));

  let first_number_2 = v.get(0);

  if let Some(d) = first_number_2 {
    println!("{}", d);
  }

  let string1 = "string";
  let string2 = "string2";

  {
    let mut str_vec = vec![string1, string2];

    let a = str_vec[0];
    str_vec.push("string");
    println!("{}", a);
  }

  println!("{}", string2);

  let string3 = String::from("hahah");
  let string4 = String::from("ggag");
  let string5 = String::from("sgasgd");

  {
    let mut str_vec = vec![string3, string4];

    let a = &str_vec[0];
    println!("{}", a);
    str_vec.push(string5);
  }

  // now string3, string4 and string5 had dropped...
  // println!("{}", string3); painc
}
