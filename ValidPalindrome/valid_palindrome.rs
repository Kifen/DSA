fn main() {
  let palindrome = "rotator";
  let is_palindrome = valid_palindrome(palindrome.to_string());
  println!("{} is palindrome: {}", palindrome, is_palindrome);
}

fn valid_palindrome(s: String) -> bool {
  let s_bytes = s.as_bytes();
  let mut left = 0;
  let mut right = s.len() - 1;

  while left < right {
    if s_bytes[left] != s_bytes[right] {
      return false;
    }

    left += 1;
    right -= 1;
  }

  return true;
}