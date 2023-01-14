pub fn types() {
  let str = "string";
  let num = -2; // Defaults to i32
  let u32num: u32 = 4_000;
  let arch: isize = -128; // Arch dependent
  let booleanvar: bool = true;
  let overflow: i8 = 127;
  let float: f32 = 1_145.141_919;
  let chr = 'c';
  let emoji = '😁';

  println!("str = {}", str);
  println!("num = {}", num);
  println!("u32num = {}", u32num);
  println!("booleanvar = {}", booleanvar);
  println!("arch = {}", arch);
  println!("overflow = {}", overflow - 1);
  println!("float = {}", float);
  println!("{}har {}", chr, emoji);

  println!("{}", "Display trait");
  println!("{:?}", "Debug trait");

  // Mutable variables;
  let mut variable = 0;
  println!("variable before change = {}", variable);
  variable = 32767;
  println!("variable after change = {}", variable);

  // Consts
  const BOOK_FLY_CONST: &str = "shufeikang"; // Must provide explicit type;
  println!("constant {}", BOOK_FLY_CONST);
}

pub fn tuples() {
  // initilization 
  let t1 = ("tuple", 1.53, 2);
  // indexing
  println!("first element of t1 = {}", t1.0);
  println!("second element of t1 = {}", t1.1);
  println!("third element of t1 = {}", t1.2);
  // print a whole tuple
  println!("t1 = {:?}", t1); 
  
  // destruct a tuple
  let (name, height, index) = t1; // for mut, use let (name, mut height, mut index);
  println!("name = {}", name);
  println!("height = {}", height);
  println!("index = {}", index);
}

pub fn arrays() {
  // Initilization
  let arr1 = [1, 2, 3, 4, 5];
  println!("arr1 = {:?}", arr1);
  let arr2: [i32; 10] = [0; 10];
  println!("arr2 = {:?}", arr2);

  for i in 0..arr1.len() {
      println!("arr[{}] = {}", i, arr1[i]);
  }
  
  // use iterator
  for i in arr2.iter() {
      print!(" {} ", i);
  }
  print!("\n");
  // println!("{}", arr1[10]);   runtime error

  // mutable array
  let mut arr3: [i32; 20] = [0; 20];
  arr3[1] = 514;
  println!("{:?}", arr3);
}