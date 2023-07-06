use std::thread;
use std::time::Duration;

pub fn spawn_threads() {
  let handle = thread::spawn(|| {
    println!("Spawned thread starts running.");
    for i in 1..10 {
      println!("Spawned thread does something {i}");
      thread::sleep(Duration::from_millis(500));
    }
    println!("Spawned thread stops running.");
  });

  for i in 1..5 {
    println!("Main thread does something {i}");
    thread::sleep(Duration::from_millis(500));
  }
  
  handle.join().unwrap();
}