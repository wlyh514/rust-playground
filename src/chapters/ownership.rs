
pub fn ownership() {
  // immutable, known at compile time
  let literal = "string literal";
  let string = give_ownership();

  let literal2 = literal; // copy (implements the Copy trait)
  println!("{literal2}");

  let string2 = string; // move (shallow copy)
  // compile time error
  // println!("{string}");
  take_ownership(string2);

  let string = give_ownership();
  let (string, size) = string_len(string);
  println!("length of \"{string}\" is {size}");
}

pub(crate) fn give_ownership() -> String {
    String::from("a string object") // moves out ownership
}

fn take_ownership(str: String) -> String {
    let str2 = str; 
    str2
}

/**
 * Takes away and returns ownership.
 */
fn string_len(str: String) -> (String, usize) {
    let size = str.len();
    (str, size)
}
