use std::io::println;

fn main() {
  for num in range(0i, 500) {
    spawn(proc() {
      println("Hello");
    })
  }
}
