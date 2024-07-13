package day08_test

import (
	"day08"
	"testing"
)

func TestProcessPart1(t *testing.T) {
	tests := []struct {
		input    string
		expected int
	}{
		{
			input:    `""`,
			expected: 2,
		},
		{
			input:    `"abc"`,
			expected: 2,
		},
		{
			input:    `"aaa\"aaa"`,
			expected: 3,
		},
		{
			input:    `"\x27"`,
			expected: 5,
		},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			result := day08.ProcessPart1(tt.input)
			if result != tt.expected {
				t.Errorf("ProcessPart1(%q) = %d; expected %d", tt.input, result, tt.expected)
			}
		})
	}
}
