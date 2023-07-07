use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};  // Multi Producer Single Consumer

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

pub fn thread_move() {
  let v = vec![0, 1, 2]; 
  let handle = thread::spawn(move || {
    for item in v.iter() {
      println!("spawned thread: {item}");
      thread::sleep(Duration::from_millis(500));
    }
  });

  handle.join().unwrap();
}

pub fn channel() {
  let (tx, rx) = mpsc::channel::<String>();
  // vs. mpsc::sync_channel, sync_channel is bounded, sender will block if
  // buffer is full. 

  thread::spawn(move || {
    let val = String::from("hello from spawned thread. ");
    tx.send(val).unwrap(); // Ownership of val is taken
    thread::sleep(Duration::from_millis(500)); 
    let val2 = String::from("Still alive");
    tx.send(val2).unwrap();
  });

  // recv blocks when there is nothing in the channel. 
  let mut received = rx.recv().unwrap();
  println!("Received: {received}");

  // try_recv doesn't block, it instead returns Err if no value is to be read
  received = match rx.try_recv() {
    Ok(s) => s, 
    Err(_) => {
      println!("Read from channel failed. ");
      String::from("nothing")
    },
  };
  println!("Received: {received}");
} 

pub fn multi_producer() {

  let (tx, rx) = mpsc::channel::<i32>();
  let tx1 = tx.clone();

  thread::spawn(move || {
    let vals = vec![0, 1, 2, 3];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_millis(500));
    }
  });
  
  thread::spawn(move || {
    let vals = vec![4, 5, 6, 7];
    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_millis(500));
    }
  });

  for val in rx {
    println!("Received: {val}");
  }
}

pub fn counter_using_mutex() {
  let mut handlers = vec![];
  let counter = Arc::new(Mutex::new(0));
  
  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handler = thread::spawn(move || {
      for _ in 0..100 {
        let mut num = counter.lock().unwrap();
        *num += 1; // Mutex provides internal mutability
      }
    });
    handlers.push(handler);
  }

  for handler in handlers {
    handler.join().unwrap();
  }

  println!("The counter value is {}", *counter.lock().unwrap());
}

pub fn deadlock() {
  let counter1 = Arc::new(Mutex::new(0));
  let counter2 = Arc::new(Mutex::new(0));

  let counter1_t1 = Arc::clone(&counter1);
  let counter2_t1 = Arc::clone(&counter2);
  let handler1 = thread::spawn(move || {
    let mut c1 = counter1_t1.lock().unwrap();
    let mut c2 =counter2_t1.lock().unwrap();
    *c1+=1;
    *c2+=1;
  });

  let counter1_t2 = Arc::clone(&counter2);
  let counter2_t2 = Arc::clone(&counter2);
  let handler2 = thread::spawn(move || {
    let mut c2 = counter2_t2.lock().unwrap();
    let mut c1 = counter1_t2.lock().unwrap();
    *c1+=1;
    *c2+=1;
  });

  handler1.join().unwrap();
  handler2.join().unwrap();
}
