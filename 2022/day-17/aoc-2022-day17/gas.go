package main

import (
	"strings"
)

type Gas struct {
	gas []string
	idx int
}

func parseGas(inputLines []string) Gas {
	gasDirections := []string{}
	gasSigns := strings.Split(inputLines[0], "")
	for _, gs := range gasSigns {
		gasDirections = append(gasDirections, gasToDirection(gs))
	}
	return Gas{gas: gasDirections, idx: 0}
}

func gasToDirection(gas string) string {
	if gas == "<" {
		return Left
	} else {
		return Right
	}
}

func (g *Gas) getNextDirection() string {
	direction := g.gas[g.idx]
	if g.idx + 1 == len(g.gas) {
		g.idx = 0
	} else {
		g.idx += 1
	}
	return direction
}