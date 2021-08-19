package main

import (
	"fmt"
	"strings"
	ht"dsa.com/UniqueChar"
	ga"dsa.com/GroupAnograms"
	vp"dsa.com/ValidPalindrome"
	vp2"dsa.com/ValidParentheses"
	bs"dsa.com/BinarySearch"
	rs"dsa.com/RotateMatrix"
	ts"dsa.com/TwoSum"
)

func main() {
	str := "WWeerriridz"
	index := ht.FindUniqueChar(str)

	anogramArr := []string{"ab", "ba", "eat", "ate", "feel", "efel", "rise", "tea", "rode", "orde", "gere", "me", "yered", "redye"}
	anogram := ga.GroupAnagrams(anogramArr)

	palindrome := strings.ToLower("rotator")
	isPalindrome := vp.ValidPalindrome(palindrome)

	parentheses := "[(([{}]){[]})]"
	isValidParentheses := vp2.ValidParentheses(parentheses)

	arr := []int{1,3,4,5,6,7,8}
	target := 9
	indexOfTarget := bs.BinarySearch(arr, target)

	matrix := [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
	rs.RotateMatrix(matrix)

	twoSums := []int{1,4,5,6,7,8}
	t := 14
	indices := ts.TwoSum(twoSums, t)

	fmt.Println("Unique char index: ", index)
	fmt.Println("Anogram: ", anogram)
	fmt.Println(fmt.Sprintf("%s is valid palindrome: %v", palindrome, isPalindrome))
	fmt.Println(fmt.Sprintf("%s is valid parentheses: %v", parentheses, isValidParentheses))
	fmt.Println(fmt.Sprintf("Index of target %v in array %v is %v", target, arr, indexOfTarget))
	fmt.Println(fmt.Sprintf("Indices of two sum in array %v is %v", twoSums, indices))
}