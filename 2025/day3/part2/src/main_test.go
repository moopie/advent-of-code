package main

import (
	"testing"
)

func TestJoltage1(t *testing.T) {
	espected := 987654321111
	input := "987654321111111"

	result := FindJoltage(input)

	if result != espected {
		t.Errorf("FindJoltage(%q) = %d; want %d", input, result, espected)
	}
}

func TestJoltage2(t *testing.T) {
	espected := 811111111119
	input := "811111111111119"

	result := FindJoltage(input)

	if result != espected {
		t.Errorf("FindJoltage(%q) = %d; want %d", input, result, espected)
	}
}

func TestJoltage3(t *testing.T) {
	espected := 434234234278
	input := "234234234234278"

	result := FindJoltage(input)

	if result != espected {
		t.Errorf("FindJoltage(%q) = %d; want %d", input, result, espected)
	}
}

func TestJoltage4(t *testing.T) {
	espected := 888911112111
	input := "818181911112111"

	result := FindJoltage(input)

	if result != espected {
		t.Errorf("FindJoltage(%q) = %d; want %d", input, result, espected)
	}
}
