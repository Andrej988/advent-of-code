package main

import (
	"fmt"
)

func main() {
	filename := "input.txt"
	lines := readInputFile(filename)

	fmt.Println("First challenge:")
	monkies := parseMonkies(lines)
	SolveFirstChallenge1(&monkies)

	fmt.Println("------------------")

	fmt.Println("Second challenge:")
	newMonkies := parseMonkies(lines)
	SolveSecondChallenge(&newMonkies)
}
