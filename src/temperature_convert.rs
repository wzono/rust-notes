use std::env;

/// 总结
/// 1. match 的使用，相当于 switch， 但比 switch 更灵活，可以使用 | 来同时满足多个值
/// 2. block 是表达式，会直接返回值
/// 3. 浮点数参与运算时，需要将参与运算的数字字面量也写成小数点的形式
/// 4. 读取命令行参数需要使用 env 包
/// 5. 双冒号有几种作用：
///   1. 命名空间作用域，引用子模块
///   2. 指定方法泛型
/// 
/// 6. 读取命令行参数时第一个参数是二进制文件所在的相对路径
/// 7. String 可以调用 parse 方法进行转换，并调用 unwrap 进行拆箱
/// 8. String 可以调用 chars 来获取 Char Array,并可以调用 collect 来转换成可迭代对象
/// 9. 
/// 

pub fn start() {
  let args = env::args().collect::<Vec<String>>();
  let degree = args[2].parse::<f32>().unwrap();
  let to = args[3].chars().collect::<Vec<char>>();

  println!("{}", temperature_convert(degree, to[0]));
}


fn temperature_convert(degree: f32, to: char) -> f32 {
  match to {
    'c' | 'C' => to_celsius(degree),
    'f' | 'F' => to_fahrenheit(degree),
    _ => 0.0
  }
}

fn to_celsius(degree: f32) -> f32 {
  (degree - 32.0) / 9.0 * 5.0
}

fn to_fahrenheit(degree: f32) -> f32 {
  degree * 9.0 / 5.0 + 32.0
}
