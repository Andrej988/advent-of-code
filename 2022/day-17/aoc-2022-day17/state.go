package main

import "strconv"

type State struct {
	gasIdx      int
	shapeName   string
	heightState string
}

func newState(gasIdx int, shapeName string, heightState []int) State {
	heightStateString := ""
	for _, i := range heightState {
		heightStateString = heightStateString + strconv.Itoa(i) + ";"
	}

	return State{
		gasIdx:      gasIdx,
		shapeName:   shapeName,
		heightState: heightStateString,
	}
}