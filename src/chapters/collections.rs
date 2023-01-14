use std::{vec, collections::HashMap};

pub fn vector() {
  let _v1: Vec<i32> = Vec::new();
  let mut v1 = vec![0, 1, 2];

  v1.push(4);
  v1.push(5);

  let third_elm = &v1[2];
  println!("third element of v1 = {third_elm}");
  // let third_elm = &v1[99]
  // panics

  let hundredth_elm = v1.get(99);
  match hundredth_elm {
    Some(hundredth_elm) => println!("The 100th element in v1 is {hundredth_elm}"),
    None => println!("No 100th element in v1."),
  }

  {
    let _v2 = vec![0, 1, 3];
    // third_elm = &v2[0]; 
    // causes a compile time error because v2 dies before third_elm
  } // v2 and all its elements freed here

  println!("third_elm after v2 being cleared = {third_elm}");

  for i in &v1 {
    print!("{i}, ")
  }
  print!("\n");

  println!("Modify vector elements in a for loop");
  for i in &mut v1 {
    *i += 50;
    print!("{i}, ");
    // v1.push(1);
    // causes a compile time error because mut ref of v1 is already borrowed 
    // at the beginning of the for loop
  }
  print!("\n");

  enum VecElm {
    Number(f64),
    Text(String),
  }
  let v2 = vec![VecElm::Number(5.0), VecElm::Text(String::from("this is a string"))];
  for (indx, elm) in v2.iter().enumerate() {
    match elm {
      VecElm::Number(num) => println!("v2[{indx}] is a number, {num}"),
      VecElm::Text(string) => println!("v2[{indx}] is a string, \"{string}\""),
    }
  }
}

pub fn string() {
  let str1 = String::from("String object from string literal");
  let str2 = String::new();
  
  let str3 = str1 + &str2;  // Coerce &String to &str
  println!("str1 has been moved, str2 = {str2}, str3 = {str3}");

  // using ranges to create string slices can cause panic in runtime.

  let cn_string = String::from("你好！");
  // traverse u8 scalar values
  for c in cn_string.chars() {
    println!("{c}");
  }

  // traverse bytes
  for b in cn_string.bytes() {
    println!("{b}");
  }
}

pub fn hashmap() {
  let mut coord_to_chunk = HashMap::new();
  coord_to_chunk.insert((0, 0), 15);
  let _chunk = coord_to_chunk
    .get(&(0, 1))
    .copied()
    .unwrap_or(0);

  for (key, value) in &coord_to_chunk {
    println!("coord_to_chunk[{:?}] = {value}", key);
  }
  
  let desc = String::from("The 0,0 chunk");
  let mut coord_to_desc: HashMap<(i32, i32), String> = HashMap::new();
  coord_to_desc.insert((0, 0), desc);  // ownership of desc is moved to coord_to_chunk
  // println!("desc = {desc}");
  // compiler error

  let mut str_to_str = HashMap::new();
  let str1 = String::from("string 1");
  let str2 = String::from("string 2");

  str_to_str.insert(str1, &str2);

  let k = String::from("string 1");
  let default_str = &String::from("default string");  // This ref must live as long as the HashMap
  let v = str_to_str
    .get(&k)
    .unwrap_or(&default_str);
  println!("str_to_str[\"string1\"] = {v}");
  
  // "upsert"
  coord_to_chunk
    .entry((1, 1))
    .or_insert(42);
  
  // example: count chars in a string
  let string = String::from("This is a string consisting of characters");
  let mut char_to_occurance = HashMap::new();
  for chr in string.chars() {
    let count = char_to_occurance.entry(chr).or_insert(0);
    *count += 1;
  }
  println!("{:?}", char_to_occurance);
}