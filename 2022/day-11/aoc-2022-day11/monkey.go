package main

import "fmt"



type Operation struct {
	item1 string
	operator string
	item2 string
}

type Test struct {
	conditionDivisibleBy int
	throwToMonkeyIfTrue  int
	throwToMonkeyIfFalse int
}

type Monkey struct {
	id                int
	inspectionCounter int
	items             []int64
	operation         Operation
	test              Test
}

func (m *Monkey) Print() {
	fmt.Println("----------------")
	fmt.Println("Monkey ID:", m.id)
	fmt.Println("Inspection counter:", m.inspectionCounter)
	fmt.Println("Items:", m.items)
	fmt.Println("Operation:", m.operation)
	fmt.Println("Test condition:", m.test.conditionDivisibleBy)
	fmt.Println("Throw if true:", m.test.throwToMonkeyIfTrue)
	fmt.Println("Throw if false:", m.test.throwToMonkeyIfFalse)
	fmt.Println("----------------")
}

func printMonkies(monkies *[]Monkey) {
	for _, monkey := range *monkies {
		monkey.Print()
	} 
}

func (m *Monkey) addItem(item int64) {
	m.items = append(m.items, item)
}

func (m *Monkey) increaseInspectionCounter() {
	m.inspectionCounter += 1
}
