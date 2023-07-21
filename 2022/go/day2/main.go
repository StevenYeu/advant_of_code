package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type GameResult = int

const (
	Lose GameResult = 3 * iota
	Draw
	Win
)

type Move = int

const (
	Rock Move = iota + 1
	Paper
	Scissor
)

func getMove(move string) Move {
	switch move {
	case "A", "X":
		return Rock
	case "B", "Y":
		return Paper
	case "C", "Z":
		return Scissor
	default:
		return Rock
	}
}

func getGameResult(oppo, your Move) GameResult {
	if oppo == your {
		return Draw
	}
	if (your == Scissor && oppo == Rock) || your > oppo {
		return Win
	}

	return Lose
}

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	if err != nil {
		panic(err)
	}
	points := 0
	for scanner.Scan() {
		parsedLine := scanner.Text()
		moves := strings.Split(parsedLine, " ")
		oppoMove := getMove(moves[0])
		yourMove := getMove(moves[1])
		points += yourMove
		points += getGameResult(oppoMove, yourMove)

	}
	fmt.Println(points)
}
