use std::collections::HashMap;

fn main() {
  let s = "MERMAIDDERAIo";
  let index = find_first_unique_char(s.to_string());

  println!("Index of unique char is {} ", index);
}

fn find_first_unique_char(s: String) -> i32 {
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