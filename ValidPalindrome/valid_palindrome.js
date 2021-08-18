const isPalindrome = (s) => {
  const san = s.replace(/[^\w]/gi, "").toLowerCase();
  return san.split("").reverse().join("") === san;
};

const pointReversal = (s) => {
  s = s.replace(/[^\w]/gi, "").toLowerCase();
  let left = 0;
  let right = s.length - 1;

  while (left < right) {
    if (s[left] !== s[right]) return false;
    left++;
    right--;
  }

  return true;
};

const run = (() => {
  console.log(isPalindrome("rotators"));
  console.log(pointReversal("rotator"));
})();
