use crate::chapters::ownership;
pub fn pass_by_ref() {
  let mut num = 32;   // The variable must be mut
  set_to_zero(&mut num);  // Pass by reference by &mut
  println!("num = {}", &num);
}

fn set_to_zero(num: &mut i32) {
    *num = 0; // To make warning go away
    println!("set_to_zero: num = {}", num);
}

pub fn reference_and_borrowing() {
  // A reference always points to a valid value.
  let string = ownership::give_ownership();
  let size = string_len_ref(&string); // 'borrowing'
  println!("length of \"{string}\" is {size}");

  fn string_len_ref(str: &String) -> usize {
    // str does not have ownership of the string

    // will throw a compile time error
    // str.push_str("something");
    str.len()
  }

  {
    // mutable references
    // the varibale it references must be mutable
    let mut string = ownership::give_ownership();
    let string_w = &mut string;

    // because only one mut ref can exist at a time
    // to prevent data races
    /*
        let string_w2 = &mut string;
        // will throw a compile time error
    */
    // also there cannot be immutable references to the object
    // when a mutable reference is present
    /*
        let string_w2 = &string;
        // will throw a compile time error
    */
    
    fn append_string(str: &mut String) {
        str.push_str(" --append-something");
    }
    append_string(string_w);
    println!("string after append {string_w}");

    // a reference's scope ends when it is last used
    let mut string = ownership::give_ownership();
    let str_ref1 = &string; 
    let str_ref2 = &string; 
    println!("{str_ref1}, {str_ref2}"); // <-- ends here

    let string_w = &mut string; 
    println!("mutable reference: {string_w}");

    /*
        fn dangle() -> &String {
            let s = String::from("something");
            // &s  
            // this throws an error because s get deallocated at the end of the fn. 
            // &s will be a dangling reference

        }
    */
  }
}

pub fn slices() {
  // string slice: reference to a part of string
  let mut string = String::from("0123456789");
  let slice = &string[0..5];  // assume string contains only ascii characters
  
  // string.clear(); // can't create mut ref when the slice ref is alive.
  // clear_string(&mut string);

  println!("the slice is {slice}");
  string.clear();
}
