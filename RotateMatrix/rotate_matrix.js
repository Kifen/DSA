const rotate = (matrix) => {
  console.log("Input Matrix: ", matrix);
  matrix.reverse();

  for (let i = 0; i < matrix.length; i++) {
    for (let j = 0; j < i; j++) {
      console.log(i, j);
      let temp = matrix[i][j];
      matrix[i][j] = matrix[j][i];
      matrix[j][i] = temp;
    }
  }

  console.log("Rotated Matrix: ", matrix);
};

const run = (() => {
  const matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [21, 48, 59],
  ];
  rotate(matrix);
})();
