extern crate sync;
use std::io::println;

fn plus_one(channel: &sync::comm::DuplexStream<int, int>) {
  let mut value: int;

  loop{
    value = channel.recv();
    channel.send(value + 1);
    if value == 0 { break; }
  }
}

fn main() {
  let (from_child, to_child) = sync::comm::duplex();

  spawn(proc() {
    plus_one(&to_child);
  });

  from_child.send(24);
  from_child.send(25);

  from_child.send(0);

  for num in range(0i, 4) {
    let answer = from_child.recv();
    println(answer.to_str().as_slice());
  }
}
