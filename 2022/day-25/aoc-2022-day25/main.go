package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func main() {
	filename := "input.txt"
	nums := parseInputsBase5(readInputFile(filename))

	sum := 0
	for _, el := range nums {
		sum += el
	}
	result := parseBase10ToSnafu(sum)
	fmt.Println("Result SNAFU:", result)
}

func parseInputsBase5(inputLines []string) []int {
	parsedInputs := []int{}
	for _, st := range inputLines {
		line := strings.Trim(st, " ")
		result := 0
		lengthOfInput := len(line)-1
		for i := lengthOfInput; i>=0; i-- {
			var num int;
			if line[i] == '-' {
				num = -1
			} else if line[i] == '=' {
				num = -2
			} else {
				num  =int(line[i] - '0')
				
			}
			result += num * int(math.Pow(float64(5), float64(lengthOfInput - i)))
		} 
		parsedInputs = append(parsedInputs, result)
	}
	return parsedInputs
}

func parseBase10ToSnafu(numBase10 int) string {
	result := []string{}
	num := numBase10
	for {
		if num == 0 {
			break
		}
		temp := (num + 2) % 5 - 2
		switch temp {
		case -2:
			result = append(result, "=")
		case -1:
			result = append(result, "-")
		default:
			result = append(result, strconv.Itoa(temp))
		}
		num = (num +2) / 5
	}
	resultStr := ""
	for i:=len(result)-1; i>=0; i-- {
		resultStr+=result[i]
	}
	return resultStr
}