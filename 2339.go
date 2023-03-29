package main

import "fmt"

func main() {

	aluno := userInput()
	folhas := userInput()
	papel := userInput()

	if folhas < (aluno * papel) {
		fmt.Println("N")
	} else {
		fmt.Println("S")
	}

}

func userInput() int {
	var input int
	fmt.Scan(&input)
	return input
}
