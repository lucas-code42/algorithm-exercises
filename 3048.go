package main

import fmt

func main() {

	n := input()
	var array []int

	for i := 0; i < n; i++ {
		array = append(array, input())
	}

	fmt.Println(getTarget(array))
}

func getTarget(data []int) int {
	var array []int
	
	array = append(array, data[0])
	tmp := data[0]
	
	for i := 0; i < len(data); i++ {
		if data[i] != tmp {
			array = append(array, data[i])
			tmp = data[i]
		}
	}

	return len(array)
}

func input() int {
	var n int
	fmt.Scan(&n)
	return n
}
