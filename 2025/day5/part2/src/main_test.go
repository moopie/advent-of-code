package main

import "testing"

func TestExample1(t *testing.T) {
	ranges := []string{
		"3-5",
		"10-14",
		"16-20",
		"12-18",
	}

	result := GetValidIngredientAmount(ranges)
	expected := 14

	if result != expected {
		t.Errorf("Expected %d to be equal %d", result, expected)
	}
}

func TestExample2(t *testing.T) {
	ranges := []string{
		"16-20",
		"12-18",
	}

	result := GetValidIngredientAmount(ranges)
	expected := 9

	if result != expected {
		t.Errorf("Expected %d to be equal %d", result, expected)
	}
}

func TestRange1(t *testing.T) {
	ranges := []Pair{
		Pair{low: 10, high: 20},
		Pair{low: 8, high: 15},
	}
	expected := true

	result := InRange(ranges[0], ranges[1])

	if result != expected {
		t.Errorf("Expected %t but got %t", expected, result)
	}
}

func TestRange2(t *testing.T) {
	ranges := []Pair{
		Pair{low: 8, high: 12},
		Pair{low: 8, high: 15},
	}
	expected := true

	result := InRange(ranges[0], ranges[1])

	if result != expected {
		t.Errorf("Expected %t but got %t", expected, result)
	}
}

func TestRange3(t *testing.T) {
	ranges := []Pair{
		Pair{low: 8, high: 12},
		Pair{low: 13, high: 15},
	}
	expected := false

	result := InRange(ranges[0], ranges[1])

	if result != expected {
		t.Errorf("Expected %t but got %t", expected, result)
	}
}