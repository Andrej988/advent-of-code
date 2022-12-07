package main

import "fmt"

type Folder struct {
	Name    string
	Parent  *Folder
	Files   []File
	Folders []*Folder
}

func NewEmptyFolder(name string, parent *Folder) Folder {
	return Folder{
		Name:    name,
		Parent:  parent,
		Files:   []File{},
		Folders: []*Folder{},
	}
}

func (f *Folder) addSubfolder(newFolder *Folder) {
	f.Folders = append(f.Folders, newFolder)
}

func (f *Folder) addFile(newFile *File) {
	f.Files = append(f.Files, *newFile)
}

func (f *Folder) getSubfolder(name string) *Folder {
	for _, subfolder := range f.Folders {
		if subfolder.Name == name {
			return subfolder
		}
	}
	return nil
}

func (f *Folder) print(level int) {
	if level == 0 {
		fmt.Printf("%v- %v (size: %v) \n", getNumberOfLeadingBlankSpaces(level), f.Name, f.getSize())
	}

	for _, subfolder := range f.Folders {
		fmt.Printf("%v- %v (size: %v) \n", getNumberOfLeadingBlankSpaces(level), subfolder.Name, subfolder.getSize())
		subfolder.print(level + 1)
	}

	for _, file := range f.Files {
		fmt.Printf("%v- %v (%v) \n", getNumberOfLeadingBlankSpaces(level), file.Name, file)
	}
}

func getNumberOfLeadingBlankSpaces(level int) string {
	leadingSpaces := ""
	for i := 0; i < level; i++ {
		leadingSpaces += "  "
	}
	return leadingSpaces
}

func (f *Folder) getSize() int {
	size := 0

	for _, subfolder := range f.Folders {
		size += subfolder.getSize()
	}

	for _, file := range f.Files {
		size += file.Size
	}

	return size
}

func (f *Folder) calculateTotal(maxThreshold int) int {
	total := 0
	size := f.getSize()
	if size < maxThreshold {
		total += size
	}

	for _, subfolder := range f.Folders {
		total += subfolder.calculateTotal(maxThreshold)
	}
	return total
}

func (f *Folder) findBestMatchingFolder(minSize int, bestMatch int) int {
	if f.getSize() > minSize && f.getSize() < bestMatch {
		bestMatch = f.getSize()
	}

	for _, subfolder := range f.Folders {
		bestMatch = subfolder.findBestMatchingFolder(minSize, bestMatch)
	}
	return bestMatch
}
