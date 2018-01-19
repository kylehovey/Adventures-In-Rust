#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
  // Test of debug print
  println!("This is a debug print of {:?}", 42);

  // Test of structure print
  println!("Structure: {0:?}, Deep: {1:?}", Structure(42), Deep(Structure(42)));
}
