package main

import "fmt"

func main() {
	fmt.Println("First challenge:")
	filename1 := "input.txt"
	blueprints1 := parseLinesToBlueprints(readInputFile(filename1))
	solveFirstChallenge(blueprints1)

	fmt.Println("----------------")

	fmt.Println("Second challenge:")
	filename2 := "input2.txt"
	blueprints2 := parseLinesToBlueprints(readInputFile(filename2))
	solveSecondChallenge(blueprints2)
}