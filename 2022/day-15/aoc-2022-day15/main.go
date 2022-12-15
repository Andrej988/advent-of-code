package main

import "fmt"

func main() {
	filename := "input.txt"
	yIdx := 2000000
	rowLimit := 4000000

	/*filename := "input-simple.txt"
	yIdx := 10
	rowLimit := 20*/

	lines := readInputFile(filename)

	fmt.Println("First challenge:")
	sensors := parseSensors(lines)
	//fmt.Println(sensors)
	solveFirstChallenge(&sensors, yIdx)

	fmt.Println("----------------")

	fmt.Println("Second challenge:")
	solveSecondChallenge(&sensors, rowLimit)
}

