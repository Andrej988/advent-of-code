package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func processInputFile(filename string) *Folder {
	root := NewEmptyFolder("/", nil)
	var currentFolder *Folder = &root

	readFile, err := os.Open(filename)
	if err != nil {
		fmt.Println(err)
	}

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		lineText := fileScanner.Text()
		if strings.HasPrefix(lineText, "$ cd") {
			changeTo := lineText[5:]
			if changeTo == "/" {
				currentFolder = &root
			} else if changeTo == ".." {
				currentFolder = currentFolder.Parent
			} else {
				currentFolder = currentFolder.getSubfolder(changeTo)
			}
		} else if strings.HasPrefix(lineText, "$ ls") {
			//Nothing to do here

		} else if strings.HasPrefix(lineText, "dir") {
			newSubfolder := NewEmptyFolder(strings.Trim(lineText[4:], " "), currentFolder)
			currentFolder.addSubfolder(&newSubfolder)

		} else if len(strings.Trim(lineText, " ")) > 0 {
			tokens := strings.Split(strings.Trim(lineText, " "), " ")
			size, _ := strconv.Atoi(tokens[0])
			newFile := NewEmptyFile(tokens[1], size)
			currentFolder.addFile(&newFile)
		}
	}

	readFile.Close()
	return &root
}
