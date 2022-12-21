package main

import (
	"fmt"
	"strconv"
)

type Monkey struct {
	name      string
	num1      string
	num2      string
	operation string
}

func newMonkey(name string, num1 string, num2 string, operation string) Monkey {
	return Monkey{
		name:      name,
		num1:      num1,
		num2:      num2,
		operation: operation,
	}
}

func parseLines(inputLines []string, challenge2 bool) (map[string]int, map[string]Monkey) {
	results := make(map[string]int)
	monkies := make(map[string]Monkey)
	format := "%s %s %s"

	for _, line := range inputLines {
		name := line[:4]
		lineText := line[6:]

		// Is digit?
		if num, err := strconv.Atoi(lineText); err == nil {
			results[name] = int(num)
		} else {
			var num1, num2, operation string
			fmt.Sscanf(lineText, format, &num1, &operation, &num2)
			monkies[name] = newMonkey(name, num1, num2, operation)
		}
	}
	return results, monkies
}