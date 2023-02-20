package main

import fmt

func main() {
	// fmt.Println(Numero de rodadas)
	runTimes := inputInt()

	for i := 0; i < runTimes; i++ {
		// fmt.Println(Numero de entradas)
		howManyEntrys := inputInt()

		var array []int
		for j := 0; j < howManyEntrys; j++ {
			var alphabetPosition int
			entryPosition := j
			var elementPosition int

			data := inputString()

			for k := 0; k < len(data); k++ {
				alphabetPosition = int(data[k] - 65)
				elementPosition = k

				s := alphabetPosition + entryPosition + elementPosition
				array = append(array, s)
			}
		}
		fmt.Println(sumArray(array))
	}
}

func inputString() string {
	var input string
	fmt.Scan(&input)
	return input
}

func inputInt() int {
	var input int
	fmt.Scan(&input)
	return input
}

func sumArray(data []int) int {
	var result int
	for _, v := range data {
		result += v
	}
	return result
}
