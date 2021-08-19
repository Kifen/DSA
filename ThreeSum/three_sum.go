package ThreeSum

import "sort"

func ThreeSum(arr []int) (output [][]int) {
	sort.Ints(arr)

	for i := 0; i < len(arr) - 2; i++ {
		left := i + 1
		right := len(arr) - 1

		if i > 0 && arr[i] == arr[i-1] {
			continue
		}

		for left < right {
			total := arr[i] + arr[left] + arr[right]
			if total == 0 {
				output = append(output, []int{arr[i], arr[left], arr[right]})

				for left < right  && arr[right] == arr[right - 1] {
					right--
				}

				for left < right && arr[left] == arr[left + 1] {
					left++
				}

				left++
				right--
			} else if total > 0 {
				right--
			} else {
				left++
			}
		}
	}

	return
}