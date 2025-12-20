package main

import (
	"testing"
)

func TestToggle0(t *testing.T) {
	input := `
		[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
		[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
		[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
	`
	expected := 33

	result := ToggleLights(input)

	if result != expected {
		t.Errorf("Expected %d to be equal to %d", result, expected)
	}
}

func TestToggle1(t *testing.T) {
	input := `
		[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
	`
	expected := 10

	result := ToggleLights(input)

	if result != expected {
		t.Errorf("Expected %d to be equal to %d", result, expected)
	}
}

func TestToggle2(t *testing.T) {
	input := `
		[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
	`
	expected := 12

	result := ToggleLights(input)

	if result != expected {
		t.Errorf("Expected %d to be equal to %d", result, expected)
	}
}

func TestToggle3(t *testing.T) {
	input := `
		[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
	`
	expected := 11

	result := ToggleLights(input)

	if result != expected {
		t.Errorf("Expected %d to be equal to %d", result, expected)
	}
}

func TestToggleReal1(t *testing.T) {
	input := `
  	[##....##.#] (2,5,6,8) (1,4,6,8) (0,1,2,3,4,5,6,9) (0,1,4) (1,2,3,4,5,6,7,8) (1,5) (5,7) (4,6,7,8,9) (2,9) (0,3,4,5,8,9) (0,1,4,5,6,7,9) (1,2,6,8) (3,4,9) {25,44,44,48,60,66,32,20,47,58}
	`
	expected := 111

	result := ToggleLights(input)

	if result != expected {
		t.Errorf("Expected %d to be equal to %d", result, expected)
	}
}
