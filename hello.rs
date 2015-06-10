use std::io;

fn main() {
  println!("To Who you want to say hello?, Please give a name:");
  let mut name = String::new();
  io::stdin().read_line(&mut name)
    .ok()
    .expect("Failed to read line");

  println!("Hello {}!", name.trim());
}
