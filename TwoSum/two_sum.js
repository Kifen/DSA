const twoSum = (arr, target) => {
  let hashTable = {};
  for (let i = 0; i < arr.length; i++) {
    const num = arr[i];
    const want = target - num;

    if (want in hashTable) {
      const leftIndex = hashTable[want];
      return [leftIndex, i];
    } else {
      hashTable[num] = i;
    }
  }
};

const run = (() => {
  const arr = [1, 3, 5, 6, 7];
  const target = 6;

  console.log(twoSum(arr, target));
})();
