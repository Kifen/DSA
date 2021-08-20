use std::collections::HashMap;

pub fn find_first_unique_char(s: String) -> i32 {
  let mut hash_table = HashMap::new();
  let bytes = s.as_bytes();

  for (_, &char) in bytes.iter().enumerate() {
    let v = hash_table.entry(char).or_insert(0);
    *v += 1;
  }

  for (i, &char) in bytes.iter().enumerate() {
    let index: i32 = match hash_table.get(&char) {
      Some(v) => *v as i32,
      _ => 0,
    };

    if index == 1 {
      return i as i32;
    }
  }

  return -1;
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn it_works() {
      let s = "MERMAIDDERAIpxCvfg";

        assert_eq!(find_first_unique_char(s.to_string()), 12);
    }
}
