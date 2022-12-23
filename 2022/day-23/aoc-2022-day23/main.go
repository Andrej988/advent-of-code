package main

import "fmt"

func main() {
	filename := "input.txt"
	lines := readInputFile(filename)

	fmt.Println("First challenge:")
	solveFirstChallenge(lines)

	fmt.Println("-------------------")

	fmt.Println("Second challenge:")
	solveSecondChallenge(lines)
}