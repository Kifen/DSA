pub fn binary_search(arr: &Vec<i32>, target: &i32) -> i32 {
  let mut low: i32 = 0;
  let mut high: i32 = (arr.len() - 1) as i32;

  while low <= high {
    let mid = ((low + high)/2) as f64;
    let mid = mid.floor() as i32;
    let mid_val = &arr[mid as usize];

    if target == mid_val {
      return mid;
    } else if target < mid_val {
      high = mid - 1;
    } else {
      low = mid + 1;
    }
  }

  return low;
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn it_works() {
      let arr = vec![1,3,4,5,6,7,8, 9, 12, 33, 35, 45];
      let target = 35;
      
        assert_eq!(binary_search(&arr, &target), 10);
    }
}
