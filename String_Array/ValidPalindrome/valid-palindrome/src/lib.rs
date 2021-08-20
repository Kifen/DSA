pub fn valid_palindrome(s: String) -> bool {
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

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn it_works() {
      let palindrome = "rotator";
      assert_eq!(valid_palindrome(palindrome.to_string()), true);
    }
}
