package main

import (
	"fmt"
	"math"
)

func solveFirstChallenge(lines []string) {
	elves := parseLines(lines)

	moves := []string{}
	moves = append(moves, North)
	moves = append(moves, South)
	moves = append(moves, West)
	moves = append(moves, East)
	
	round := 0

	for {
		round++

		elvesAdjecant := false
		proposedMoves := make(map[Coordinates][]Coordinates)
		for coords := range elves {
			hasToMove := false
			for _, adjCoord := range coords.getAllAdjacentCoords() {
				if isFieldTaken(elves, adjCoord) {
					hasToMove = true
					elvesAdjecant = true
					break
				}
			}

			if hasToMove {
				for _, moveDirection := range moves {
					fieldsBlocked := false
					for _, adjCoord := range coords.getAdjacentCoords(moveDirection) {
						if isFieldTaken(elves, adjCoord) {
							fieldsBlocked = true
							break
						}
					}
					if !fieldsBlocked {
						destCoords := coords.peek(moveDirection)
						sourceCoords, ok := proposedMoves[destCoords]
						if !ok {
							sourceCoords = []Coordinates{}
						} 
						sourceCoords = append(sourceCoords, coords)
						proposedMoves[destCoords] = sourceCoords
						break;
					}
				}
			}
		}

		if !elvesAdjecant {
			break
		}

		for destinationCoords, sourceCoordsArr := range proposedMoves {
			if len(sourceCoordsArr) == 1 {
				sourceCoords := sourceCoordsArr[0]
				delete(elves, sourceCoords)
				elves[destinationCoords] = true
			}
		}

		moves = shuffleMoves(moves)

		if round == 10 {
			break
		}
	}

	result := calculateResult(elves)
	fmt.Println("Result: ", result)
}

func isFieldTaken(elves map[Coordinates]bool, coords Coordinates) bool {
	_, ok := elves[coords]
	return ok
}

func shuffleMoves(moves []string) []string {
	firstMove := moves[0]
	moves = moves[1:]
	moves = append(moves, firstMove)
	return moves
}

func calculateResult(elves map[Coordinates]bool) int {
	min_x := 1000000000
	max_x := -1000000000
	min_y := 1000000000
	max_y := -1000000000

	for coords := range elves {
		min_x = int(math.Min(float64(min_x), float64(coords.X)))
		max_x = int(math.Max(float64(max_x), float64(coords.X)))
		min_y = int(math.Min(float64(min_y), float64(coords.Y)))
		max_y = int(math.Max(float64(max_y), float64(coords.Y)))
	}
	
	return ((max_x - min_x) +1) * ((max_y - min_y) +1)  - len(elves)
}