use crate::third::say_hello as third_say_hello;

pub fn say_hello() {
  println!("Hello from first module");

  third_say_hello();
}