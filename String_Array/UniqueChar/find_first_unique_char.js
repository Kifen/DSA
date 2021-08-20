// Time complexity = O(n)
// Space complexity = O(64) = constant time
const findUniqueChar1 = (str) => {
  const hashTable = {};

  for (let char of str) {
    if (hashTable[char]) hashTable[char]++;
    else hashTable[char] = 1;
  }

  for (let i = 0; i < str.length; i++) {
    const char = str[i];
    if (hashTable[char] == 1) return i;
  }

  return -1;
};

// Time complexity = O(n)^2
const findUniqueChar2 = (str) => {
  for (let i = 0; i < str.length; i++) {
    const char = str[i];
    if (str.indexOf(char) === str.lastIndexOf(char)) {
      return i;
    }
  }

  return -1;
};
