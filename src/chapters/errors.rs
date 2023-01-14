/*
 * Recoverable errors: e.g. file not found, -> Wrapped in enums like Result<T, E>
 * Unrecoverable errors: e.g. reading outside an array -> panic!
 */

use std::fs::File;
use std::io::{ErrorKind, Read, self};

fn _create_file_if_doesnt_exist() {
  let greeting_file_result = File::open("hello.txt");

  let _greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error);
      }
    },
  };
}

fn _create_file_if_doesnt_exist_closure() {
  let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
}

fn _unwrap_and_expect() {
  File::open("hello.txt").unwrap(); // if ok, then return file obj, else panic!
  File::open("hello.txt")
    .expect("hello.txt should be included in this project"); // same with above, but with a message on panic!
}

/**
 * The question mark operator
 */
fn _read_file() -> Result<String, io::Error> {
  let mut content = String::new();
  File::open("hello.txt")?.read_to_string(&mut content)?;
  Ok(content)
}