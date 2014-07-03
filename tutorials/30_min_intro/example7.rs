use std::sync::{Arc, Mutex};

fn main() {
  let numbers = vec![1i, 2i, 3i];
  let numbers = Arc::new(Mutex::new(numbers));

  for num in range(0u, 3) {
    let (tx, rx) = channel();
    tx.send(numbers.clone());

    spawn(proc() {
      let numbers_lock = rx.recv();

      // Take the lock, along with exclusive access to
      // the underlying array
      let mut numbers = numbers_lock.lock();

      // This is ugly for now, but will be replaced by
      // `numbers[num as uint] += 1` in the near future.
      // See: See: https://github.com/rust-lang/rust/issues/6515
      *numbers.get_mut(num as uint) = *numbers.get_mut(num as uint) + 1;

      println!("{}", *numbers.get(num as uint));

      // When `numbers` goes out of scope, the lock is dropped
    });
  }
}
