package main

type Coordinates struct {
	X int
	Y int
}

func newCoordinates(x int, y int) Coordinates {
	return Coordinates{X: x, Y: y}
}

func (c *Coordinates) peek(direction string) Coordinates {
	switch direction {
	case North:
		return newCoordinates(c.X-1, c.Y)
	case East:
		return newCoordinates(c.X, c.Y+1)
	case West:
		return newCoordinates(c.X, c.Y-1)
	case South:
		return newCoordinates(c.X+1, c.Y)
	case NorthWest:
		return newCoordinates(c.X-1, c.Y-1)
	case NorthEast:
		return newCoordinates(c.X-1, c.Y+1)
	case SouthWest:
		return newCoordinates(c.X+1, c.Y-1)
	case SouthEast:
		return newCoordinates(c.X+1, c.Y+1)
	default:
		return newCoordinates(-100000, -100000)
	}
}

func (c *Coordinates) getAllAdjacentCoords() []Coordinates {
	coords := []Coordinates{}
	coords = append(coords, c.peek(NorthWest))
	coords = append(coords, c.peek(North))
	coords = append(coords, c.peek(NorthEast))
	coords = append(coords, c.peek(East))
	coords = append(coords, c.peek(SouthEast))
	coords = append(coords, c.peek(South))
	coords = append(coords, c.peek(SouthWest))
	coords = append(coords, c.peek(West))
	return coords
}

func (c *Coordinates) getAdjacentCoords(direction string) []Coordinates {
	coords := []Coordinates{}
	switch direction {
	case North:
		coords = append(coords, c.peek(NorthWest))
		coords = append(coords, c.peek(North))
		coords = append(coords, c.peek(NorthEast))
	case East:
		coords = append(coords, c.peek(NorthEast))
		coords = append(coords, c.peek(East))
		coords = append(coords, c.peek(SouthEast))
	case South:
		coords = append(coords, c.peek(SouthEast))
		coords = append(coords, c.peek(South))
		coords = append(coords, c.peek(SouthWest))
	case West:
		coords = append(coords, c.peek(SouthWest))
		coords = append(coords, c.peek(West))
		coords = append(coords, c.peek(NorthWest))
	}
	return coords
}