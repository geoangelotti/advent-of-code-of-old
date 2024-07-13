package day08

import (
	"fmt"
	"strconv"
	"strings"
)

func unescape(input string) string {
	unescaped, err := strconv.Unquote(input)
	if err != nil {
		return ""
	}
	return unescaped
}

func ProcessPart1(input string) int {
	var acc int
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		unescaped := unescape(line)
		fmt.Println(line, len(line), len(unescaped))
		acc += len(line) - len(unescaped)
	}
	return acc
}
