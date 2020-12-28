use std::collections::HashMap;

pub fn start() {
  create_hash_map();
  ref_hash_map();
  get_value();
  traversal_map();
  map_or_insert();
}

fn create_hash_map() {
  let keys = vec!["key1", "key2"];
  let values = vec![1, 2];

  // 注解是必须的
  // collect 可能创建多种不同的结构
  // 键和值的类型可以推导出，使用 _ 省略
  let map: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

  println!("{:#?}", map);
}

fn ref_hash_map() {
  let mut map = HashMap::new();

  let s1 = "Hello".to_string();
  let s2 = "Value".to_string();

  map.insert(&s1, &s2);

  println!("{}", s1);

  println!("{:#?}", map);
}

fn get_value() {
  let mut map = HashMap::new();

  map.insert("Hello", 1);

  let key = String::from("Hello");

  // use ref
  let v1 = map.get(&key[..]);

  let v2 = map.get("good");

  if let Some(t) = v1 {
    println!("{}", t);
  }

  // do not execute
  if let Some(t) = v2 {
    println!("{}", t);
  }
}

// 无序遍历
fn traversal_map() {
  let mut map = HashMap::new();

  map.insert("k1", "v1");
  map.insert("k2", "v2");
  map.insert("k3", "v3");
  map.insert("k4", "v4");


  for (key, value) in map {
    println!("key: {}, value: {}", key, value);
  }



  let mut map2 = HashMap::new();

  let s2 = String::from("k2");

  map2.insert("k1".to_string(), 1);
  map2.insert(s2, 2);

  // here map2 have moved itself keys and values
  // we can add & to map2 for borrowing rather than moving
  for (key, value) in map2 {
    println!("key: {}, value: {}", key, value);
  }

  // can not execute
  // let v2 = map2.get(&s2);
}

fn map_or_insert() {
  let str = "hello world my friend world";

  let map = stat_word_count(str);

  println!("{:#?}", map);
}

fn stat_word_count(sentence: &str) -> HashMap<&str, i32> {
  let mut map = HashMap::new();

  for word in sentence.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    // or_insert 返回值的可变引用
    *count += 1; // 解引用
  }

  map
}
