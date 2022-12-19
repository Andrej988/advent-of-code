package main

import "fmt"

func main() {
	filename := "input.txt"
	lines := readInputFile(filename)
	blueprints := parseLinesToBlueprints(lines)

	fmt.Println("First challenge:")
	solveFirstChallenge(blueprints)

	fmt.Println("----------------")

	//fmt.Println("Second challenge:")
	//solveSecondChallenge(gas)
}