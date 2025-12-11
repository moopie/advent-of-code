package main

import "testing"

func TestOutput1(t *testing.T) {
	input := `
	svr: aaa bbb
	aaa: fft
	fft: ccc
	bbb: tty
	tty: ccc
	ccc: ddd eee
	ddd: hub
	hub: fff
	eee: dac
	dac: fff
	fff: ggg hhh
	ggg: out
	hhh: out
	`
	expected := 2

	result := GetPaths(input)

	if result != expected {
		t.Errorf("Expected %d to equal %d", result, expected)
	}
}

func TestOutput2(t *testing.T) {
	input := `
	svr: aaa bbb ggg
	aaa: bbb
	bbb: aaa
	dac: fff
	fff: ccc
	ccc: ggg fff
	ggg: out
	`
	expected := 0

	result := GetPaths(input)

	if result != expected {
		t.Errorf("Expected %d to equal %d", result, expected)
	}
}