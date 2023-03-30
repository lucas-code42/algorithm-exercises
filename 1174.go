package main

import "fmt"

func main() {
	var arr []float64

	for i := 0; i < 100; i++ {
		user := getUserInput()
		arr = append(arr, user)
	}

	for j := 0; j < len(arr); j++ {
		if arr[j] <= 10.0 {
			fmt.Printf("A[%d] = ", j)
			value := fmt.Sprintf("%.1f", arr[j])
			fmt.Printf("%s\n", value)
		}

	}

}

func getUserInput() float64 {
	var input float64
	fmt.Scan(&input)
	return input
}