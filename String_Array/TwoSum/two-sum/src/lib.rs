use std::collections::HashMap;

pub fn two_sum(arr: &Vec<i32>, target: &i32) -> Vec<i32> {
  let mut hash_table = HashMap::new();

  for (i, v) in arr.iter().enumerate() {
    let want = target - v;
    match hash_table.get(&want) {
      Some(x) => {
        return vec![*x, i as i32];
      },
      None => hash_table.insert(v, i as i32),
    };
  }

  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

    const ARR: Vec<i32> = vec![1, 2, 3, 4, 9, 11];
    
    #[test]
    fn it_works() {
        let target = 15;
        assert_eq!(two_sum(&ARR, &target), vec![3, 5]);
    }

    #[test]
    fn it_fails() {
      let target = 9;
      assert_eq!(two_sum(&ARR, &target), vec![])
    }
}
