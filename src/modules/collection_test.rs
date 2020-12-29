use std::collections::HashMap;

pub fn start() {
  let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, 2.0, 2.0];
  let (avg, middle, mode) = get_vec_stat(&vec);
  println!("{}, {}, {}", avg, middle, mode);

}

fn get_vec_stat(vec: &Vec<f32>) -> (f32, f32, f32) {
  let mut cloned_vec = vec.clone();
  let mut map = HashMap::new();
  let avg: f32;
  let mut mode: f32 = vec[0];
  let mut sum: f32 = 0.0;
  let mut max_count = 0;

  let middle_indexs = if vec.len() % 2 == 0 {
    vec![vec.len() / 2, vec.len() / 2 - 1]
  } else {
    vec![vec.len() / 2]
  };

  &cloned_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

  // 求中位数
  let mut middle = 0.0;
  let indexs_len = middle_indexs.len() as f32;
  for i in middle_indexs {
    middle += cloned_vec[i];
  }

  middle = middle / indexs_len;

  // 求平均数
  for &num in cloned_vec.iter() {
    let count = map.entry(num.to_string()).or_insert(0);
    *count += 1;
    sum += num;
  }

  avg = sum / vec.len() as f32;

  // 求众数
  for (key, value) in &map {
    if value > &max_count {
      mode = key.parse::<f32>().unwrap();
      max_count = *value;
    }
  }

  (avg, middle, mode)
}
