use std::fmt::Display;
use std::cmp::PartialOrd;

type Position = (f64, f64);
type Velocity = (f64, f64);


trait Entity {
  fn position(&self) -> Position;
}

pub trait Mob {
  fn move_to(&self, pos: Position) {
    println!("default implementation: move to {:?}", pos);
  }
  fn velocity(&self) -> Velocity;
  fn position(&self) -> Position;
}

pub struct Ship {
  
}

// wither the trait or the struct must be in the local crate.
// because of coherence
impl Entity for Ship {
  fn position(&self) -> Position {
    (0.0, 0.0)
  }
}

impl Mob for Ship {
  // use the default implementation for move_to
  // by implementing it again we can override the default implementation for Ship

  fn position(&self) -> Position {
    (1.1, 1.1)
  }

  fn velocity(&self) -> Velocity {
    (0.0, 0.0)
  }
}

// trait as parameter
fn _move_to_center(mob: &impl Mob) {
  mob.move_to((0.0, 0.0));
}

// trait bound & where syntax
struct _TrivialMob<T>
where 
  T: Mob + Copy,
{
  entity: T,
}

// conditionally implement methods
struct Pair<T> {
  x: T, 
  y: T,
}

impl<T> Pair<T>
where 
  T: Display + PartialOrd 
{
  fn print_largest(&self) {
    // match arms must return the same type
    let largest = match self.x.gt(&self.y) {
      true => &self.x,
      false => &self.y,
    };
    println!("{largest} is larger");
  }
}
// blanket implementation
// impl<T: Display> ToString for T {
//   // --snip--
// }