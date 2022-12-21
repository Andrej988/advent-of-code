package main

import (
	"fmt"
	"strings"
)

func solveFirstChallenge(lines []string) {
	results, monkiesQueue := parseLines(lines)
	numOfIterations := 0

	for {
		if monkiesQueue.IsEmpty() {
			break
		}

		numOfIterations++

		monkey := monkiesQueue.PopFront()
		knowNumbers := 0

		num1, ok1 := results[monkey.num1]
		if ok1 {
			knowNumbers++
		}

		num2, ok2 := results[monkey.num2]
		if ok2 {
			knowNumbers++
		}

		if knowNumbers == 2 {
			switch strings.Trim(monkey.operation, " ") {
			case "+":
				results[monkey.name] = num1 + num2
			case "-":
				results[monkey.name] = num1 - num2
			case "*":
				results[monkey.name] = num1 * num2
			case "/":
				results[monkey.name] = num1 / num2
			}
		} else {
			monkiesQueue.PushBack(monkey)
		}
	}

	result, ok := results["root"]
	if ok {
		fmt.Printf("Result: %v (took %v iterations)", result, numOfIterations)
	} else {
		fmt.Println("Root is not known!!!")
	}
}
