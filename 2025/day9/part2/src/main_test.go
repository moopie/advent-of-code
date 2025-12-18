package main

import "testing"

func TestTheater(t *testing.T) {
	input := `
	7,1
	11,1
	11,7
	9,7
	9,5
	2,5
	2,3
	7,3
	`
	expected := int64(24)

	result := FindSeating(input)

	if result != expected {
		t.Errorf("Expected %d to be equal to %d", result, expected)
	}
}
