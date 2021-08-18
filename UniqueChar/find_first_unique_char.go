package HashTable

func FindUniqueChar(str string) int {
	m := make(map[string]int)

	for i := 0; i < len(str); i++ {
		char := string(str[i])

		if _, ok := m[char]; ok {
			m[char]++
		} else {
			m[char] = 1
		}
	}

	for i := 0; i < len(str); i++ {
		char := string(str[i])

		if m[char] == 1 {
			return i
		}
	}

	return -1
}

