package main

import (
	"math"
)

func maxInt(num1 int, num2 int) int {
	return int(math.Max(float64(num1), float64(num2)))
}

func minInt(num1 int, num2 int) int {
	return int(math.Min(float64(num1), float64(num2)))
}

func maxOfInts(num1 int, num2 int, num3 int, num4 int) int {
	max := int(math.Max(float64(num1), float64(num2)))
	max = int(math.Max(float64(max), float64(num3)))
	max = int(math.Max(float64(max), float64(num4)))
	return max
}