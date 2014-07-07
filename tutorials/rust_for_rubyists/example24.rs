use std::io::println;

fn main() {
  let (chan, port) = channel();

  spawn(proc() {
    chan.send(10i);
  });

  println(port.recv().to_str().as_slice());
}
