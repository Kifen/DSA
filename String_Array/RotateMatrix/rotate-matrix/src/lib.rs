pub fn rotate_matrix(matrix: &mut Vec<Vec<i32>>) {
  matrix.reverse();

  for i in 0..matrix.len() {
    for j in 0..matrix.len() {
      if i < j {
        let temp = matrix[j][i];
        matrix[j][i] = matrix[i][j];
        matrix[i][j] = temp;
      }
    }
  }  
}

#[cfg(test)]
mod tests {
  use super::*;
    #[test]
    fn it_works() {
      let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![21, 48, 59]];
      rotate_matrix(&mut matrix);

        assert_eq!(matrix, vec![vec![21, 4, 1], vec![48, 5, 2], vec![59, 6, 3]]);
    }
}
