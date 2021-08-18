use std::collections::HashMap;

fn main() {
  let parentheses = "[(([{}]){[]})]{}".to_string();
  let is_valid = valid_parentheses(&parentheses);

  println!("{:?} is valid parentheses: {:?}", parentheses, is_valid);
}

fn valid_parentheses(str: &String) -> bool {
  let mut pairs = HashMap::new();
  let mut stack = Vec::new();

  pairs.insert("(".to_string(), ")".to_string());
  pairs.insert("{".to_string(), "}".to_string());
  pairs.insert("[".to_string(), "]".to_string());

  let str_char = str.chars();

  for (_, char) in str_char.enumerate() {
    match pairs.get(&char.to_string()) {
      Some(_) => {
        stack.push(char.to_string());
      },
      None => {
        match stack.pop() {
          Some(x) => {
            if pairs[&x] != char.to_string() {
              return false;
            }
          },
          _ => return false,
        }
      },
    }
  }

  if stack.len() != 0 {
    return false;
  }

  return true;
}