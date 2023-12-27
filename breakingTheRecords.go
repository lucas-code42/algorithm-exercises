package main

import "fmt"

func breakingRecords(scores []int32) []int32 {
	var lowestScoreList []int32
	var highestScoreList []int32

	var higherScore int32
	var lowestScore int32

	higherScore, lowestScore = scores[0], scores[0]
	for _, v := range scores {
		if v > higherScore {
			higherScore = v
			highestScoreList = append(highestScoreList, higherScore)
		}
		if v < lowestScore {
			lowestScore = v
			lowestScoreList = append(lowestScoreList, lowestScore)
		}
	}

	return []int32{int32(len(highestScoreList)), int32(len(lowestScoreList))}
}

func main() {
	r := breakingRecords([]int32{10, 5, 20, 20, 4, 5, 2, 25, 1})
	fmt.Printf("RESULT = %d %d", r[0], r[1])
}
