package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func readInputFile(filename string) []string {
	var lines []string

	readFile, err := os.Open(filename)
	if err != nil {
		fmt.Println(err)
	}

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		lineText := fileScanner.Text()
		if len(strings.Trim(lineText, " ")) > 0 {
			lines = append(lines, lineText)
		}
	}

	readFile.Close()
	return lines
}