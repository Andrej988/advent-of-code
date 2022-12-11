package main

import (
	"strconv"
	"strings"
)

func parseMonkies(lines []string) []Monkey {
	var monkies []Monkey

	for i := 0; i < len(lines); i++ {
		if(strings.HasPrefix(lines[i], "Monkey"))  {
			monkey := Monkey{
				id:                parseMonkeyId(lines[i]),
				inspectionCounter: 0,
				items:             parseItems(lines[i+1]),
				operation:         parseOperation(lines[i+2]),
				test:              parseTest(lines[i+3], lines[i+4], lines[i+5]),
			}
			monkies = append(monkies, monkey)
		}
		i += 5
	}
	return monkies
}

func parseMonkeyId(line string) int {
	l, _ := strconv.Atoi(line[len(line)-2 : len(line)-1])
	return l
}

func parseItems(line string) []int64 {
	var items []int64
	lineToTokenize := strings.Trim(strings.Replace(line[strings.Index(line, ":")+2:], ",", "", -1), " ")
	tokens := strings.Split(lineToTokenize, " ")
	for _, token := range tokens {
		item, _ := strconv.ParseInt(token, 10, 64)
		items = append(items, item)
	}
	return items
}

func parseOperation(line string) Operation {
	lineToTokenize := strings.Trim(strings.Replace(line[strings.Index(line, "=")+2:], ",", "", -1), " ")
	tokens := strings.Split(lineToTokenize, " ")
	//fmt.Println("Tokens: {}", tokens)
	if len(tokens) > 3 {
		panic("LONGER EQUATION!!!")
	}

	return Operation{
		item1:    tokens[0],
		operator: tokens[1],
		item2:    tokens[2],
	}
}

func parseTest(testLine string, ifTrue string, ifFalse string) Test {
	divisibleBy, _ := strconv.Atoi(testLine[strings.Index(testLine, "divisible by")+13:])
	monkeyIfTrue, _ := strconv.Atoi(ifTrue[strings.Index(ifTrue, "monkey")+7:])
	monkeyIfFalse, _ := strconv.Atoi(ifFalse[strings.Index(ifFalse, "monkey")+7:])

	return Test{
		conditionDivisibleBy: divisibleBy,
		throwToMonkeyIfTrue:  monkeyIfTrue,
		throwToMonkeyIfFalse: monkeyIfFalse,
	}
}