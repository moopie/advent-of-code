package main

import "testing"

func Test(t *testing.T) {
	input := `
	..@@.@@@@.
	@@@.@.@.@@
	@@@@@.@.@@
	@.@@@@..@.
	@@.@@@@.@@
	.@@@@@@@.@
	.@.@.@.@@@
	@.@@@.@@@@
	.@@@@@@@@.
	@.@.@@@.@.
	`
	expected := 13

	result := CountForklifts(input)
	if result != expected {
		t.Errorf("Expected %d, but got %d", expected, result)
	}
}
