use std::collections::BTreeMap;
use std::env;


/// 总结
/// 1. 计算斐波那契数列方式可以直接使用递归
/// 2. 但要生成序列的话，在每次层递归完毕后需要插入新的值
/// 3. 由于数组和可变Vec都不能直接在某个位置上插入值（按道理可变 Vec 也可以实现）
/// 4. 所以采用 Map 来存储每次计算的值，再下次使用到同一值可以避免重复计算
/// 
/// 知识点
/// 1. String::parse 以及拆箱 unwrap
/// 2. 如果定义的 mut，在传递引用时也需要传递 mut，传递引用时使用 & 传递
/// 3. 在函数中接受引用时，使用 &，并且若是接受 mut，也需要带上 mut
/// 4. 与 c++ 一致，解引用使用 *
/// 5. 取 map 的迭代器的值的方式为 i.0 和 i.1 ，分别对应 key 和 value
/// 6. HashMap 是没有顺序的，可以使用 BTreeMap 代替
///
/// 几个疑问
/// 1. 为什么 map 在判断key存在/获取值 时需要使用 & 来取引用，而在插入时又不需要
/// 
/// update: 因为 Map 的 Key 类型不一定是基本类型，如果不使用 & 来取引用，很可能会产生 Move 行为导致所有权转移，而之后又可能会用到这个 key。（从注释中可以得知 The key may be any borrowed form of the map's key type, but the ordering on the borrowed form must match the ordering on the key type.）
/// 而为什么 insert 不使用引用呢？我猜测是他在设计之初猜测了用户行为，认为 key 的最终使用就是用来 insert ，因此直接产生 Move 行为
/// 

pub fn start() {
  let args = env::args().collect::<Vec<String>>();

  let n = args[2].parse::<usize>().unwrap();

  g(n);
}


fn f(n: usize, map: &mut BTreeMap<usize, usize>) -> usize {
  
  if map.contains_key(&n) {
    return *(map.get(&n).unwrap());
  }

  let c = match n {
    1 | 2 => 1,
    0 => 0,
    _ => f(n - 1, map) + f(n - 2, map),
  };

  map.insert(n, c);

  c
}

fn g(n: usize) {
  let mut map: BTreeMap<usize, usize> = BTreeMap::new();
  map.insert(1, 1);
  map.insert(2, 1);

  f(n, &mut map);

  for i in map.iter() {
    println!("{}", i.1);
  }

}



