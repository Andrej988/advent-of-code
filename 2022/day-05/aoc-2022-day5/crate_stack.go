package main

import (
	"fmt"
	"os"
)

type crateStack []string

func (s crateStack) pop() (crateStack, string) {
	if len(s) == 0 {
		fmt.Println("Error: Stack is empty")
		os.Exit(1)
	}

	l := len(s)
	return s[:l-1], s[l-1]
}

func (s crateStack) push(value string) crateStack {
	return append(s, value)
}

func (s crateStack) peek() string {
	return s[len(s)-1]
}

func (s crateStack) print() {
	for i, crate := range s {
		fmt.Println(i, crate)
	}
}

func (s crateStack) numberOfItems() int {
	return len(s)
}

func newCrateStack() crateStack{
	return crateStack{}
}

func (s crateStack) reverse() crateStack {
	var value string
	reverseStack := crateStack{}
	for s.numberOfItems() > 0 {
		s, value = s.pop()
		reverseStack = reverseStack.push(value)
	}
	return reverseStack
}

func printPeeks(stacks []crateStack) {
	result := ""
	for _, stack := range stacks {
		result += stack.peek()
	}
	fmt.Println(result)
}