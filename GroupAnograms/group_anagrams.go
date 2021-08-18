package GroupAnagrams

import (
	"sort"
	"strings"
)

func GroupAnagrams(strs []string) (anagrams [][]string) { 
	hashTable := make(map[string][]string)

	for _, str := range strs {
		v := strings.Split(str, "")
		sort.Strings(v)
		sortedStr := strings.Join(v, "")

		if _, ok := hashTable[sortedStr]; ok {
			hashTable[sortedStr] = append(hashTable[sortedStr], str)
		} else {
			hashTable[sortedStr] = []string{str}
		}
	}

	for _, v := range hashTable {
		anagrams = append(anagrams, v)
	}

	return
}