package main

import "fmt"

func main() {
	filename := "input.txt"
	var lines []string = readInputFile(filename)

	fmt.Println("First challenge:")
	solveFirstChallenge(lines)

	fmt.Println("-----------------")

	fmt.Println("Second challenge:")
	solveSecondChallenge(lines)
}