use std::collections::HashMap;

pub fn valid_parentheses(str: &String) -> bool {
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

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn it_works() {
      let parentheses = "[(([{}]){[]})]{}".to_string();

        assert_eq!(valid_parentheses(&parentheses), true);
    }
}
