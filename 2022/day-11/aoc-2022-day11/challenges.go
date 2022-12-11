package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func SolveFirstChallenge1(monkies *[]Monkey) {
	playRounds(monkies, 20, 3)
	result := calculateInspectionCounterResult(monkies);
	fmt.Println("Result:", result)
}

func SolveSecondChallenge(monkies *[]Monkey) {
	playRounds(monkies, 10000, 1)
	result := calculateInspectionCounterResult(monkies);
	fmt.Println("Result:", result)
}

func playRounds(monkies *[]Monkey, numberOfRounds int, worryLevel int) {
	mod := getMod(monkies)

	for i:=1; i<=numberOfRounds; i++ {
		playRound(monkies, mod, worryLevel)
	}
	
	//fmt.Println("--------------------------------")
	//printMonkies(monkies)
}

func getMod(monkies *[]Monkey) int {
	mods := buildMods(monkies)
	mod := 1
	for _, modulo := range mods {
		mod = mod * modulo
	}
	return mod
}

func buildMods(monkies *[]Monkey) []int {
	var mods []int
	for _, monkey := range *monkies {
		mods = append(mods, monkey.test.conditionDivisibleBy)
	}
	return mods
}

func playRound(monkies *[]Monkey, mod int, worryLevel int) {
	for i := 0; i<len(*monkies); i++ {
		for j := 0; j < len((*monkies)[i].items); j++ {
			//Inspect
			(*monkies)[i].increaseInspectionCounter()
			newWorryLevel := calculateNewWorryLevel((*monkies)[i].items[j], &(*monkies)[i].operation, mod, worryLevel)
			
			//Throw
			if newWorryLevel%int64((*monkies)[i].test.conditionDivisibleBy) == 0 {
				(*monkies)[(*monkies)[i].test.throwToMonkeyIfTrue].addItem(newWorryLevel)
			} else {
				(*monkies)[(*monkies)[i].test.throwToMonkeyIfFalse].addItem(newWorryLevel)
			}

			//Remove
			(*monkies)[i].items = (*monkies)[i].items[1:]
			j -= 1
		}
	}

}

func calculateNewWorryLevel(old int64, operation *Operation, mod int, worryLevel int) int64 {
	item1 := parseItem(old, operation.item1)
	item2 := parseItem(old, operation.item2)
	var result int64 = 0

	switch strings.Trim(operation.operator, " ") {
	case "+":
		result = item1 + item2
	case "*":
		result = item1 * item2
	default:
		panic("NOT A VALID OPERATOR!!!")
	}
	
	return result%int64(mod) / int64(worryLevel);	
}

func parseItem(old int64, item string) int64 {
	if item == "old" {
		return old
	} else {
		i, _ := strconv.ParseInt(item, 10, 64)
		return i
	}
}

func calculateInspectionCounterResult(monkies *[]Monkey) int {
	var counters []int;
	for _, monkey := range *monkies {
		counters = append(counters, monkey.inspectionCounter)
	}
	
	sort.Sort(sort.Reverse(sort.IntSlice(counters)))
	return counters[0] * counters[1]
}
