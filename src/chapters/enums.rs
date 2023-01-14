pub fn enums() {
  #[derive(Debug)]
  enum Operation {
    MoveTo(u32, u32), 
    Message(String),
    Attack(String, u32, u32),
    Stop,
  }

  impl Operation {
    fn print(&self) {
      println!("{:?}", self);
    }
  }

  let atk = Operation::Attack(String::from("Blue Lazer"), 0, 0);
  atk.print();
  
  fn get_op_description(op: &Operation) -> String {
    // Must handle all cases of enum
    match op {
      Operation::Attack(method, x, y) => {
          String::from(format!("Attacked ({x}, {y}) using {method}"))
      }, 
      Operation::Message(msg) => {
          String::from(format!("Message: {msg}"))
      },
      Operation::Stop => String::from("Stop all operations"),
      Operation::MoveTo(x, y) => String::from( format!("Moved to ({x}, {y})")),
    }
  }
  println!("{}", get_op_description(&atk));

  // if let
  match &atk {
    Operation::Attack(method, x, y) => println!("Attacked ({x}, {y}) using {method}"),
    _ => println!("Not an attack operation"),
  };
  // <=>
  if let Operation::Attack(method, x, y) = atk {
      println!("Attacked ({x}, {y}) using {method}");
  } else {
    println!("Not an attack operation");
  }
}
