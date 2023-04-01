package main

import "fmt"

func userInputInt() int {
	var n int
	_, err := fmt.Scan(&n)
	if err != nil {
		panic(err)
	}
	return n
}

func userInputFloat() float64 {
	var n float64
	_, err := fmt.Scan(&n)
	if err != nil {
		panic(err)
	}
	return n
}

func main() {
	rounds := userInputInt()

	for i := 0; i < rounds; i++ {
		k := userInputFloat()
		var c int

		for {
			k = k / 2
			c++

			if k < 1 {
				break
			}
		}

		fmt.Printf("%d dias\n", c)
	}
}
