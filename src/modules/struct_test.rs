#[derive(Debug, Clone)]
struct User {
  name: String,
  age: u32,
  active: bool,
}

impl User {
  pub fn print_name(&self) {
    println!("{}", self.name);
  }
}

#[derive(Debug)]
struct Rectangle {
  width: f32,
  height: f32,
}

impl Rectangle {
  pub fn _area(&self) -> f32 {
    return self.width * self.height;
  }

  pub fn can_hold(&self, other: &Rectangle) -> bool {
    return self.width >= other.width && self.height >= other.height;
  }
}

impl Rectangle {
  pub fn square(size: f32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

pub fn start() {
  let user1 = User {
    name: String::from("hahahah"),
    age: 12,
    active: true,
  };

  let rec1 = Rectangle {
    width: 200.0,
    height: 200.0,
  };

  user1.print_name();

  noop(user1.name);

  println!("{:#?}", user1.age);

  println!(
    "can the other rec be hold? {}",
    rec1.can_hold(&Rectangle::square(200.0))
  );

  println!("area of rec1: {}", rec1._area());
}

fn noop(_s: String) {}
