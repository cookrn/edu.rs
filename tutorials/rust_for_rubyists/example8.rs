fn is_three(num: int) -> bool {
  return false;
}

#[test]
fn test_is_three_with_not_three() {
  if is_three(1) {
    fail!("One is not three");
  }
}

#[test]
fn test_is_three_with_three() {
  if !is_three(3) {
    fail!("Three should be three");
  }
}
