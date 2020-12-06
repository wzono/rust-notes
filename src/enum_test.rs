#[derive(Debug)]
enum Operation {
  Add(f32),         // 枚举元组结构体
  _Add2d(f32, f32), // 枚举元组结构体
  _Move {
    // 枚举匿名结构体
    x: f32,
    y: f32,
  },
  Sub(f32),
}

impl Operation {
  pub fn get_first(&self) {
    println!("{:#?}", self);
  }
}

pub fn start() {
  let op = Operation::Add(40.0);

  let op2 = Operation::Sub(30.0);

  do_op(&op);

  do_op(&op2);

  let x: Option<Operation> = Some(Operation::Sub(30.0));

  // x.get_first();

  match x {
    Some(v) => v.get_first(),
    None => println!("Empty Operation")
  }
}

fn do_op(op: &Operation) {
  op.get_first();
}
