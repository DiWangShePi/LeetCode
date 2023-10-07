package main

func lengthOfLongestSubstring(s string) int {
	hashMap := make(map[byte]int)
	var j int
	var lengthOfLongest int

	for index, character := range s {
		opsition, exist := hashMap[byte(character)]
		if exist {
			if lengthOfLongest < (index - j) {
				lengthOfLongest = index - j
			}
			if j <= opsition {
				j = opsition + 1
			}
		}
		hashMap[byte(character)] = index
	}

	if lengthOfLongest < (len(s) - j) {
		return len(s) - j
	}
	return lengthOfLongest
}