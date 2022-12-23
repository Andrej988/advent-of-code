package main

func parseLines(inputLines []string) map[Coordinates]bool {
	elves := make(map[Coordinates]bool)
	for x, line := range inputLines {
		for y, ch := range line {
			if ch == '#' {
				elves[newCoordinates(x, y)] = true
			}
		}
	}
	return elves
}