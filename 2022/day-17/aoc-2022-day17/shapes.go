package main

import "fmt"

const startingY = 2

type Shape struct {
	coords []Coordinates
	name   string
	number int
}

func newShape(coords []Coordinates, name string, number int) Shape {
	return Shape{coords: coords, name: name, number: number}
}

func getShape(row int, roundNumber int) Shape {
	switch roundNumber % 5 {
	case 1:
		return buildShapeHorizontalLine(row, roundNumber)
	case 2:
		return buildShapePlus(row, roundNumber)
	case 3:
		return buildShapeReverseL(row, roundNumber)
	case 4:
		return buildShapeVerticalLine(row, roundNumber)
	default:
		return buildShapeSquare(row, roundNumber)
	}
}

func buildShapeHorizontalLine(row int, number int) Shape {
	coords := []Coordinates{}
	coords = append(coords, newCoordinates(row, startingY))
	coords = append(coords, newCoordinates(row, startingY+1))
	coords = append(coords, newCoordinates(row, startingY+2))
	coords = append(coords, newCoordinates(row, startingY+3))
	return newShape(coords, "Horizontal Line", number)
}

func buildShapePlus(row int, number int) Shape {
	coords := []Coordinates{}
	coords = append(coords, newCoordinates(row, startingY+1))
	coords = append(coords, newCoordinates(row+1, startingY))
	coords = append(coords, newCoordinates(row+1, startingY+1))
	coords = append(coords, newCoordinates(row+1, startingY+2))
	coords = append(coords, newCoordinates(row+2, startingY+1))
	return newShape(coords, "Plus", number)
}

func buildShapeReverseL(row int, number int) Shape {
	coords := []Coordinates{}
	coords = append(coords, newCoordinates(row, startingY))
	coords = append(coords, newCoordinates(row, startingY+1))
	coords = append(coords, newCoordinates(row, startingY+2))
	coords = append(coords, newCoordinates(row+1, startingY+2))
	coords = append(coords, newCoordinates(row+2, startingY+2))
	return newShape(coords, "Reverse L", number)
}

func buildShapeVerticalLine(row int, number int) Shape {
	coords := []Coordinates{}
	coords = append(coords, newCoordinates(row, startingY))
	coords = append(coords, newCoordinates(row+1, startingY))
	coords = append(coords, newCoordinates(row+2, startingY))
	coords = append(coords, newCoordinates(row+3, startingY))
	return newShape(coords, "Vertical Line", number)
}

func buildShapeSquare(row int, number int) Shape {
	coords := []Coordinates{}
	coords = append(coords, newCoordinates(row, startingY))
	coords = append(coords, newCoordinates(row, startingY+1))
	coords = append(coords, newCoordinates(row+1, startingY))
	coords = append(coords, newCoordinates(row+1, startingY+1))
	return newShape(coords, "Square", number)
}

func (s *Shape) isAllowedToMove(mapOfRocks MapOfRocks, direction string) bool {
	result := true
	for _, coords := range s.coords {
		if !coords.isAllowedToMove(direction) || mapOfRocks.contains(coords.peek(direction)) {
			return false
		}
	}
	return result
}

func (s *Shape) performMove(direction string) {
	newCoords := []Coordinates{}
	for _, coords := range s.coords {
		switch direction {
		case Left:
			newCoords = append(newCoords, newCoordinates(coords.X, coords.Y-1))
		case Right:
			newCoords = append(newCoords, newCoordinates(coords.X, coords.Y+1))
		case Down:
			newCoords = append(newCoords, newCoordinates(coords.X-1, coords.Y))
		default:
		}
	}
	s.coords = newCoords
}

//lint:ignore U1000 Ignore unused function temporarily for debugging
func (s *Shape) print() {
	fmt.Printf("Shape number %v of type %v coordinates: %v \n", s.number, s.name, s.coords)
}