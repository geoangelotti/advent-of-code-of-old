package day08

import (
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
		acc += len(line) - len(unescaped)
	}
	return acc
}
