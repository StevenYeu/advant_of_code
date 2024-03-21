package main

import (
	"bufio"
	"fmt"
	"os"
	"unicode"
)

func getPriority(ch rune) int {
	ascii := int(ch)

	alphabetLen := 26
	modOp := alphabetLen + 1
	capEnd := int('Z') + 1
	lowEnd := int('z') + 1
	if unicode.IsUpper(ch) {
		return ((ascii-capEnd)+modOp)%modOp + alphabetLen
	}

	return ((ascii - lowEnd) + modOp) % modOp
}

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	sum := 0
	for scanner.Scan() {
		containerOneMap := make(map[rune]struct{})
		line := scanner.Text()
		count := len(line)
		containerOne := line[0 : count/2]
		containerTwo := line[count/2:]

		for _, ch := range containerOne {
			containerOneMap[ch] = struct{}{} 
		}
		for _, ch := range containerTwo {
			if _, ok := containerOneMap[ch]; ok {
				sum += getPriority(ch)
				break
			}
		}
	}
	fmt.Println(sum)
}
