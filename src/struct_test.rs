
#[derive(Debug)]
struct User {
  name: String,
  age: u32,
  active: bool,
}


pub fn start() {
  let user1 = User {
    name: String::from("hahahah"),
    age: 12,
    active: true
  };

  println!("{:#?}", user1);
}