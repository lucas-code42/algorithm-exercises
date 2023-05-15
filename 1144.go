package main

import "fmt"

func main() {
	n := func() int {
		var x int
		fmt.Scan(&x)
		return x
	}()

	for i := 1; i < n+1; i++ {
		fmt.Printf("%d %d %d\n", i, i*i, (i*i*i))
		fmt.Printf("%d %d %d\n", i, (i*i)+1, (i*i*i)+1)
	}

	return
}