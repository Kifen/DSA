package TwoSum

func TwoSum(arr []int, target int) (indices []int) {
	hashTable := make(map[int]int)

	for i, num := range arr {
		want := target - num

		if _, ok := hashTable[want]; ok {
			indices = append(indices, hashTable[want], i)
			break
		} else {
			hashTable[num] = i 
		}
	}

	return
}