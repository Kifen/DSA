package BinarySearch

import "math"

func BinarySearch(arr []int, target int) int {
	low := 0
	high := len(arr) - 1

	for low <= high {
		midFloor := math.Floor(float64((high + low)/2))
		mid := int(midFloor)
		midVal := arr[mid]

		if target == midVal {
			return mid
		} else if target < midVal {
			high = mid - 1
		} else {
			low = mid + 1
		}
	}

	return low
}