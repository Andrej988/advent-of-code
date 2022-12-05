package main

type move struct {
	quantity int
	from     int
	to       int
}

type moves []move

func newMoves() moves {
	return moves{}
}