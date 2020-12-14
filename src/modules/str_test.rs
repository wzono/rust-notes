pub fn start() {
  test_initial();
  test_update();
}



fn test_initial() {
  let data = "initial";
  let _s = data.to_string(); // 产生了借用，使用了 data 的不可变引用

  // to_string 方法创建 String 对象
  let _s = "initial".to_string();

  // 类比于
  // utf-8 编码，可以包含任何可以正确编码的数据
  let s = String::from("😁"); 

  println!("{}", s);
}

fn test_update() {
  let mut s = "foo".to_string();
  let s2 = "bar";
  s.push_str(s2); // 参数 string 使用 &str，不会获取所有权
  println!("s2 is {}", s2); // work!

  // push 方法是 push 单个 char 字符
  s.push('l');

  // 使用 + 

  let s1 = "Hello,".to_string();
  let s2 = "world!".to_string();

  // s1 被移动了
  let s3 = s1 + &s2;

  println!("{}, {}", s2, s3);

}