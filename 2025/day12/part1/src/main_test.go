package main

import "testing"

func TestInput1(t *testing.T) {
	input := `
	0:
	###
	##.
	##.

	1:
	###
	##.
	.##

	2:
	.##
	###
	##.

	3:
	##.
	###
	##.

	4:
	###
	#..
	###

	5:
	###
	.#.
	###

	4x4: 0 0 0 0 2 0
	12x5: 1 0 1 0 2 2
	12x5: 1 0 1 0 3 2
	`
	expected := 2
	result := ValidRegions(input)

	if result != expected {
		t.Errorf("Expected %d but got %d", expected, result)
	}
}

func TestInput2(t *testing.T) {
	input := `
	0:
	###
	##.
	##.

	1:
	###
	##.
	.##

	2:
	.##
	###
	##.

	3:
	##.
	###
	##.

	4:
	###
	#..
	###

	5:
	###
	.#.
	###

	4x4: 0 0 0 0 2 0
	`
	expected := 1
	result := ValidRegions(input)

	if result != expected {
		t.Errorf("Expected %d but got %d", expected, result)
	}

}