package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)

	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	sum := 0
	maxCalories := 0
	elf := 1
	maxElf := 1
	for scanner.Scan() {

		parsed := scanner.Text()
		if parsed == "" {

			if sum > maxCalories {
				maxCalories = max(sum, maxCalories)
				maxElf = elf
			}

			sum = 0
			elf++
			continue
		}
		cal, err := strconv.Atoi(parsed)
		if err != nil {
			panic(err)
		}
		sum += cal
	}

	file.Close()

	if sum > maxCalories {
		maxCalories = max(sum, maxCalories)
		maxElf = elf
	}

	fmt.Printf("Elf %d has the most calories with %d\n", maxElf, maxCalories)

}
