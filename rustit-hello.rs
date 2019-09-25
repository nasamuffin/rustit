use std::env;

fn main() {
  // Dump the commands we heard from the rustit dispatcher.

  let args: Vec<String> = env::args().collect();

  println!("Hello rustit world!");

  // Args...
  println!("{:?} arguments: {:?}", args.len()-1, &args[1..]);

  println!("Ok, goodnight.");
}
