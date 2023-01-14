fn longest<'a>(x: &'a str, y: &'a str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

pub fn lifetime() {
  let str1 = String::from("string 1");
  let str2 = String::from("str2");


  let longest_str = longest(&str1, &str2);
  println!("longest_str = {longest_str}");

  println!("str1 = {str1}, str2 = {str2}");
}