mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
}

pub use front_of_house::hosting;

pub fn start() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}
