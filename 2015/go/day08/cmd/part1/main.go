package main

import (
	"day08"
	"fmt"
	"os"
)

func main() {
	content, err := os.ReadFile("input.txt")
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(day08.ProcessPart1(string(content)))
}
