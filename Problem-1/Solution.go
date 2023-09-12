package main

import "fmt"

func twoSum(nums []int, target int) []int {
	hashMap := make(map[int]int)

	for index, value := range nums {
		opIndex, exist := hashMap[value]
		if exist {
			return []int{opIndex, index}
		}
		hashMap[target - value] = index
	}
	return []int{}
}


func main() {
	fmt.Println("Hello")
}
