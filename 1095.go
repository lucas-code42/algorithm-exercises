package main

import "fmt"

func main() {
	i := 1
	j := 60

	for k := j; k >= 0; k -= 5 {
		fmt.Printf("I=%d J=%d\n", i, k)
		i += 3
	}
}