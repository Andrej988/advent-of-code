package main

import "fmt"

func main() {
	filename := "input.txt"
	lines := readInputFile(filename)

	fmt.Println("First challenge:")
	packets := processInputData(lines)
	solveFirstChallenge(packets)

	fmt.Println("-------------------")

	fmt.Println("Second challenge:") 
	solveSecondChallenge(lines)

}

func processInputData(lines []string) []PacketComparison {
	data := []PacketComparison{}
	for i:=0; i< len(lines); i+=2 {
		packet := PacketComparison{
			left: lines[i],
			right: lines[i+1],
		}
		data = append(data, packet)
	} 
	return data
}
