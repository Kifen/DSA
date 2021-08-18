const binary_search = (arr, target) => {
  let low = 0;
  let high = arr.length - 1;

  while (low <= high) {
    const mid = Math.floor((high + low) / 2);
    const midVal = arr[mid];
    if (target === midVal) {
      return mid;
    } else if (target > midVal) {
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }

  return low;
};

const run = (() => {
  const arr = [1, 3, 5, 6, 7, 8];
  const target = 2;

  console.log(binary_search(arr, target));
})();
