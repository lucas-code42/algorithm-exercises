package main

func main() {
	reverseVowels("Aa")
}

func reverseVowels(s string) string {
	regularList := regularList(s)
	reverseLsit := reverseList(s)

	tmp := []rune(s)
	for index := 0; index < len(s); index++ {
		if isVowel(rune(s[index])) {
			for i, v := range regularList {
				if index == v {
					tmp[index] = rune(s[reverseLsit[i]])
				}
			}
		}
	}

	return string(tmp)
}

func regularList(s string) []int {
	l := []int{}

	for i := 0; i < len(s); i++ {
		if isVowel(rune(s[i])) {
			l = append(l, i)
		}
	}

	return l
}

func reverseList(s string) []int {
	listaNormal := regularList(s)
	for i, j := 0, len(listaNormal)-1; i < j; i, j = i+1, j-1 {
		listaNormal[i], listaNormal[j] = listaNormal[j], listaNormal[i]
	}

	return listaNormal
}

func isVowel(r rune) bool {
	return r == 'a' || r == 'e' || r == 'i' || r == 'o' || r == 'u' || r == 'A' || r == 'E' || r == 'I' || r == 'O' || r == 'U'
}