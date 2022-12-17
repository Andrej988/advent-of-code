package main

import (
	"fmt"
)

func main() {
	filename := "input.txt"
	lines := readInputFile(filename)
	gas := parseGas(lines)
	
	fmt.Println("First challenge:")
	solveFirstChallenge(gas)

	fmt.Println("----------------")

	fmt.Println("Second challenge:")
	solveSecondChallenge(gas)
}