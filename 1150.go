package main

import "fmt"

func getUserInput() int {
	var n int
	fmt.Scan(&n)
	return n
}

func calc(init, end int) int {
	var count int
	var sum int	
	for {
		sum += init
		count++
		if sum > end {
			return count
		}
		init++
	}
}

func main() {
	x := getUserInput()
	var y int
	for {
		y = getUserInput()
		if y > x {
			break
		}
	}
	fmt.Println(calc(x, y))	
}