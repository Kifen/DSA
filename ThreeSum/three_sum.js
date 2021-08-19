const threeSum = (arr) => {
  let output = [];
  arr.sort((a, b) => a - b);

  for (let i = 0; i < arr.length - 2; i++) {
    let left = i + 1;
    let right = arr.length - 1;

    if (i > 0 && arr[i] == arr[i - 1]) continue;

    while (left < right) {
      let total = arr[left] + arr[right] + arr[i];
      if (total == 0) {
        output.push([arr[i], arr[left], arr[right]]);

        while (left < right && arr[right - 1] == arr[right]) {
          right--;
        }

        while (left < right && arr[left + 1] == arr[left]) {
          left++;
        }

        right--;
        left++;
      } else if (total > 0) {
        right--;
      } else {
        left++;
      }
    }
  }

  return output;
};


const run = (() => {
  const arr = [-1, 0, 1, 2, -1, -4]
  console.log(threeSum(arr))
})()