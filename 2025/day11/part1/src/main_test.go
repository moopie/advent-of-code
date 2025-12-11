package main

import "testing"

func TestOutput(t *testing.T) {
	input := `
	aaa: you hhh
	you: bbb ccc
	bbb: ddd eee
	ccc: ddd eee fff
	ddd: ggg
	eee: out
	fff: out
	ggg: out
	hhh: ccc fff iii
	iii: out
	`
	expected := 5

	result := GetPaths(input)

	if result != expected {
		t.Errorf("Expected %d to equal %d", result, expected)
	}
}