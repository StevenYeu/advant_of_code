package main

import (
	"bufio"
	"os"
)

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)

	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {

	}

}
