package main

import "fmt"

func staircase(n int64) {
	// Write your code here

	if n <= 0 || n > 100 {
		return
	}

	for i := int(n - 1); i >= 1; i-- {
		// fmt.Println(i)

		for j := 0; j < i; j++ {
			fmt.Printf(" ")
		}

		for k := i; k < int(n); k++ {
			fmt.Printf("#")
		}
		fmt.Printf("\n")

	}
	
	for i := 0; i < int(n); i++ {
		fmt.Printf("#")
	}

}

func main() {
	staircase(100)
}
