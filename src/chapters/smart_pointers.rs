use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::fmt::Display;
use std::rc::{Rc, Weak};

enum LinkedList<T>
where T: Copy {
  Cons(T, Box<LinkedList<T>>),
  Null
}

impl<T> LinkedList<T>
where T: Copy {
  pub fn push(&mut self, new_item: T) {
    match self {
      Self::Cons(_, next) => {
        next.push(new_item);
      },
      Self::Null => {
        *self = LinkedList::Cons(new_item, Box::new(LinkedList::Null));
      },
    }
  }
}

impl<T: Copy + fmt::Display> fmt::Display for LinkedList<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Cons(content, next) => {
        let res = write!(f, "{} ", content);
        next.fmt(f)?;
        res
      },
      Self::Null => {
        write!(f, "END \n")
      },
    }
  }
}

/// Used to store data on a heap
///   1. When you want a type which size cannot be known at compile time
///   2. Transferring large amount of data and have to make sure they are not being copied
///   3. Owning a value which at compile time we can only know that it implements some traits 
pub fn boxes() {
  let _box = Box::new(5); // stored on the heap

  let mut con_list = LinkedList::Null;
  con_list.push(0);
  con_list.push(1);
  con_list.push(2);
  con_list.push(42);
  println!("{}", con_list);
}

struct MyBox<T: Display> (T);
impl<T: Display> MyBox<T> {
  fn new(content: T) -> MyBox<T> {
    MyBox(content)
  }
}

impl<T: Display> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    // returns a reference to the inner data
    // *a = *(a.deref())
    &self.0
  }
}

pub fn my_box() {
  let mybox = MyBox::new(String::from("string in a box"));
  let content = &*mybox;
  println!("*mybox = {content}");

  // deref(MyBox) -> String
  // deref(String) -> &str slice
  // cannot coerse immutable ref to mutable ref
  hello(&mybox);

  fn hello(string: &str) {
    println!("{}", string);
  }
}

impl<T: Display> Drop for MyBox<T> {
  fn drop(&mut self) {
    println!(".drop called. data '{}' dropped", self.0);
  }
}

pub fn my_drop() {
  let mybox = MyBox::new(514);
  // manually drop mybox
  std::mem::drop(mybox);
  println!("mybox dropped");
}

enum RcList {
  Con(i32, Rc<RcList>),
  Null,
}

/// Allows a single value to have multiple owners
/// 
/// Ensures the value remains valid until all references goes out of scope using reference counting
pub fn rc() {
  let a = Rc::new(RcList::Con(0, Rc::new(RcList::Null)));

  println!("After creating a, a's ref count = {}", Rc::strong_count(&a));

  let _b = RcList::Con(1, Rc::clone(&a));

  println!("After creating b, a's ref count = {}", Rc::strong_count(&a));

  {
    let _c = RcList::Con(2, Rc::clone(&a));
    println!("After creating c, a's ref count = {}", Rc::strong_count(&a));
  }
  println!("After c goes out of scope, a's ref count = {}", Rc::strong_count(&a));

  // Rc::clone merely increments the reference count
  
}

struct MockMessenger {
  messages: RefCell<Vec<String>>,
}

impl MockMessenger {
  
  fn new() -> MockMessenger {
    MockMessenger { messages: RefCell::new(Vec::new()) }
  }

  fn send(&self, msg: &str) {
    self.messages.borrow_mut().push(msg.to_string());
  }
}

/// Enforce borrowing rules at runtime (instead of compile time)
/// 
/// Interior mutability: mutate the value inside of a immutable value
/// 
/// Use Rc<RefCell<T>> to give a mutable data to multiple owners
/// 
/// NOT thread safe! Use Mutex instead. 
pub fn refcell() {
  let messenger = MockMessenger::new();
  messenger.send("message #0");
}

#[derive(Debug)]
struct TreeNode {
  parent: RefCell<Weak<TreeNode>>,
  children: RefCell<Vec<Rc<TreeNode>>>,
  content: i32,
}

impl TreeNode {
  fn new(content: i32) -> TreeNode {
    TreeNode { 
      parent: RefCell::new(Weak::new()), 
      children: RefCell::new(Vec::new()), 
      content
    }
  }
}

impl Drop for TreeNode {
  fn drop(&mut self) {
    println!("tree node with content {} dropped.", self.content);
  }
}

/// Reference cycles cauases memory leak.
/// https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#creating-a-reference-cycle
/// Reference count of a and b never reach 0. a <-> b
pub fn ref_cycles() {
  // solution: use weak reference
  let i = 42;
  let i_rc = Rc::new(&i);
  println!("i_rc reference count = {}", Rc::strong_count(&i_rc));
  let i_weak = Rc::downgrade(&i_rc);
  println!("i_rc reference count after creating i_weak = {}", Rc::strong_count(&i_rc));
  // weak count doesn't need to be 0 for a RC to be collected 
  println!("i_rc weak reference count after creating i_weak = {}", Rc::weak_count(&i_rc));

  // when doing anything with the weak reference, we must check the value it points to still exists. 
  let i_val = match i_weak.upgrade() {
    Some(rc) => rc, 
    None => Rc::new(&0), 
  };
  println!("i_val = {i_val}");

  let root = Rc::new( TreeNode::new(0) );
  {
    let branch = Rc::new( TreeNode::new(1) );
    root.children.borrow_mut().push(Rc::clone(&branch));
    *(branch.parent.borrow_mut()) = Rc::downgrade(&root);

    let leaf = Rc::new( TreeNode::new(2) );
    branch.children.borrow_mut().push(Rc::clone(&leaf));
    *(leaf.parent.borrow_mut()) = Rc::downgrade(&branch);

    println!("branch ref count = {}, leaf ref count = {}", Rc::strong_count(&branch), Rc::strong_count(&leaf));
  }

  println!("{:?}", *(root).children.borrow());

}