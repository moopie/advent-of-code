package main

import (
	"testing"
)

func TestSimplePuzzle(t *testing.T) {
	input := `
		123 328  51  64
		45  64   387 23
		6   98   215 314
		*   +    *   +
	`
	expected := 4277556

	result := CalculateEqs(input)

	if result != expected {
		t.Errorf("Expected %d to be equal to %d", result, expected)
	}
}

func TestEq1(t *testing.T) {
	input := `
		123
		45
		6
		*
	`
	expected := 33210

	result := CalculateEqs(input)

	if result != expected {
		t.Errorf("Expected %d to be equal to %d", result, expected)
	}
}
