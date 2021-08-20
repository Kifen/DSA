pub fn three_sum(arr: &mut Vec<i32>) -> Vec<Vec<&i32>> {
  arr.sort();
  let mut output: Vec<Vec<&i32>> = vec![];

  for i in 0..arr.len() - 2 {
    let mut left = i + 1;
    let mut right = arr.len() - 1;

    if i > 0 && &arr[i] == &arr[i - 1] {
      continue;
    }

    while left < right {
      let total = &arr[i] + &arr[left] + &arr[right];
      if total == 0 {
        output.push(vec![&arr[i], &arr[left], &arr[right]]);

        while left < right && &arr[right] == &arr[right - 1]{
          right -= 1;
        }

        while left < right && &arr[left] == &arr[left + 1] {
          left += 1;
        }

        left += 1;
        right -= 1;
      } else if total > 0 {
        right -= 1;
      } else {
        left += 1;
      }
    }
  }

  output
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//     #[test]
//     fn it_works() {
//       let mut arr = vec![-1, 0, 1, -2, 3, -1, 9, -6, -3];
//       let ans = vec![vec![&arr[7], &arr[8], &arr[6]], vec![&arr[8], &arr[1], &arr[4]], vec![&arr[3], &arr[0], &arr[4]], vec![&arr[0], &arr[1], &arr[2]]];

//       assert_eq!(ans, three_sum(&mut arr));
//       //println!("{:?}", three_sum(&mut arr));
//     }
// }
