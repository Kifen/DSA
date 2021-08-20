const group_anagrams = (arr) => {
  let hashTable = {};

  for (let str of arr) {
    const sortedStr = str.split("").sort().join("");
    if (hashTable[sortedStr]) hashTable[sortedStr].push(str);
    else hashTable[sortedStr] = [str];
  }

  return Object.values(hashTable);
};

const run = (() => {
  const wordList = ["ab", "ba", "tea", "eat", "ball", "labl", "fee", "rise"];
  const ans = group_anagrams(wordList);
  console.log("Ans: ", ans);
})();
