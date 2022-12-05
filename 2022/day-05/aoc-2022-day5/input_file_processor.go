package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func processInputFile(filename string) ([]crateStack, moves) {
	crateStacks := []crateStack{}
	reverseStacks := []crateStack{}
	craneMoves := newMoves()

	readFile, err := os.Open(filename)
	if err != nil {
		fmt.Println(err)
	}

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		lineText := fileScanner.Text()
		if strings.HasPrefix(lineText, "move") {
			craneMoves = append(craneMoves, buildMove(lineText))
		} else {
			reverseStacks = processInputFileStartingCrateStacks(reverseStacks, lineText)
		}
	}

	readFile.Close()

	for _, reverseStack := range reverseStacks {
		crateStacks = append(crateStacks, reverseStack.reverse())
	}

	return crateStacks, craneMoves
}

func processInputFileStartingCrateStacks(reverseStacks []crateStack, lineText string) []crateStack {
	if len(lineText) > 0 {
		crateIdx := 0
		startIdx := 1

		for startIdx < len(lineText) {
			value := lineText[startIdx:startIdx+1]
			if !isNumber(value) {
				if len(reverseStacks) < crateIdx +1 {
					reverseStacks = append(reverseStacks, newCrateStack())
				}
				if(len(strings.Trim(value, " ")) > 0) {
					reverseStacks[crateIdx] = reverseStacks[crateIdx].push(value)
				}
			}
			crateIdx++
			startIdx += 4
		}
	}
	return reverseStacks
}

func buildMove(lineText string) move {
	fromIdx := strings.Index(lineText, "from")
	toIdx  := strings.Index(lineText, "to")
	quantity, _ := strconv.Atoi(lineText[5:fromIdx-1])
	from, _ := strconv.Atoi(lineText[fromIdx+5:toIdx-1])
	to, _ := strconv.Atoi(lineText[toIdx+3:])
	return move{quantity: quantity, from: from, to: to}
}

func isNumber(s string) bool {
	if _, err := strconv.Atoi(s); err == nil {
		return true
	} else {
		return false
	}
}