package main

type File struct {
	Name string
	Size int
}

func NewEmptyFile(name string, size int) File {
	return File{
		Name: name,
		Size: size,
	}
}
