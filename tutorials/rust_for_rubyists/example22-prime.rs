fn is_three(num: int) -> bool {
  num % 3 == 0
}

fn is_five(num: int) -> bool {
  num % 5 == 0
}

fn is_fifteen(num: int) -> bool {
  num % 15 == 0
}

#[test]
fn test_is_three_with_not_three() {
  assert!(!is_three(1), "One is not three");
}

#[test]
fn test_is_three_with_three() {
  assert!(is_three(3), "Three should be three");
}

#[test]
fn test_is_five_with_not_five() {
  assert!(!is_five(1), "One is not five");
}

#[test]
fn test_is_five_with_five() {
  assert!(is_five(5), "Five should be five");
}

#[test]
fn test_is_fifteen_with_not_fifteen() {
  assert!(!is_fifteen(1), "One is not fifteen");
}

#[test]
fn test_is_fifteen_with_fifteen() {
  assert!(is_fifteen(15), "Fifteen should be fifteen");
}

fn main() {
  for num in range(1i, 101) {
    let answer =
      if      is_fifteen(num) { "FizzBuzz".to_str() }
      else if is_three(num)   { "Fizz".to_str()     }
      else if is_five(num)    { "Buzz".to_str()     }
      else                    { num.to_str()        };

    println!("{}", answer);
  }
}
