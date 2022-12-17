package main

const minY = 0
const maxY = 6

type Coordinates struct {
	X int
	Y int
}

func newCoordinates(x int, y int) Coordinates {
	return Coordinates{x, y}
}

func (c *Coordinates) isAllowedToMove(direction string) bool {
	switch direction {
	case Left:
		return c.Y > minY
	case Right:
		return c.Y < maxY
	case Down:
		return c.X > 0
	default:
		return false
	}
}

func (c *Coordinates) peek(direction string) Coordinates {
	switch direction {
	case Left:
		return newCoordinates(c.X, c.Y-1)
	case Right:
		return newCoordinates(c.X, c.Y+1)
	case Down:
		return newCoordinates(c.X-1, c.Y)
	default:
		return newCoordinates(-100, -100)
	}
}
