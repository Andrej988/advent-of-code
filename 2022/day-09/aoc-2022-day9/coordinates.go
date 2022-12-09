package main

import "fmt"

type Coordinates struct {
	X, Y int
}

func NewCoordinates(x int, y int) Coordinates {
	return Coordinates{
		X: x,
		Y: y,
	}
}

func (c *Coordinates) MoveUp() {
	c.X++
}

func (c *Coordinates) MoveDown() {
	c.X--
}

func (c *Coordinates) MoveRight() {
	c.Y++
}

func (c *Coordinates) MoveLeft() {
	c.Y--
}

func (c *Coordinates) Move(direction string) {
	switch direction {
	case "U":
		c.MoveUp()
	case "D":
		c.MoveDown()
	case "R":
		c.MoveRight()
	case "L":
		c.MoveLeft()
	}
}

func (c *Coordinates) Print(id string) {
	fmt.Printf("Coordinates of %v are: X: %v Y: %v \n", id, c.X, c.Y)
}

func (c *Coordinates) isAdjacent(other Coordinates) bool {
	return (other.X >= c.X -1 && other.X <= c.X +1) && (other.Y >= c.Y -1 && other.Y <= c.Y +1)
}


func (c *Coordinates) Follow (other Coordinates) {
	if !c.isAdjacent(other) {
		if other.X > c.X {
			c.MoveUp()
		} else if other.X < c.X{
			c.MoveDown()
		}
		
		if other.Y > c.Y {
			c.MoveRight()
		} else if other.Y < c.Y{
			c.MoveLeft()
		}
	}
}
