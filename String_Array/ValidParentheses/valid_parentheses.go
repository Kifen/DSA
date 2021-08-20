package ValidParentheses

func ValidParentheses(str string) bool {
	stack := make([]string, 0)
	pairs := map[string]string {
		"(": ")", 
		"{": "}", 
		"[": "]",
	}

	for _, v := range str {
		if _, ok := pairs[string(v)]; ok {
			stack = append(stack, string(v))
		} else {
			n := len(stack) - 1
			last := stack[n]
			if pairs[last] != string(v) {
				return false
			}
			stack = stack[:n]
		}
	}

	if len(stack) != 0 {
		return false
	}

	return true
}