package main

import "testing"

func TestRange1(t *testing.T) {
	ranges := []string{"3-5"}
	ingredient := 1

	result := TestIngredient(ranges, ingredient)
	expected := false

	if result != expected {
		t.Errorf("Expected %t to be equal %t", result, expected)
	}
}

func TestRange2(t *testing.T) {
	ranges := []string{"3-5"}
	ingredient := 5

	result := TestIngredient(ranges, ingredient)
	expected := true

	if result != expected {
		t.Errorf("Expected %t to be equal %t", result, expected)
	}
}

func TestRange3(t *testing.T) {
	ranges := []string{"3-5", "10-14"}
	ingredient := 8

	result := TestIngredient(ranges, ingredient)
	expected := false

	if result != expected {
		t.Errorf("Expected %t to be equal %t", result, expected)
	}
}

func TestRange4(t *testing.T) {
	ranges := []string{"10-14"}
	ingredient := 11

	result := TestIngredient(ranges, ingredient)
	expected := true

	if result != expected {
		t.Errorf("Expected %t to be equal %t", result, expected)
	}
}

func TestRange5(t *testing.T) {
	ranges := []string{"16-20"}
	ingredient := 17

	result := TestIngredient(ranges, ingredient)
	expected := true

	if result != expected {
		t.Errorf("Expected %t to be equal %t", result, expected)
	}
}

func TestRange6(t *testing.T) {
	ranges := []string{"12-18"}
	ingredient := 32

	result := TestIngredient(ranges, ingredient)
	expected := false

	if result != expected {
		t.Errorf("Expected %t to be equal %t", result, expected)
	}
}

func TestExample(t *testing.T) {
	ranges := []string{
		"3-5",
		"10-14",
		"16-20",
		"12-18",
	}
	ingredients := []int{
		1,
		5,
		8,
		11,
		17,
		32,
	}

	result := GetValidIngredientAmount(ranges, ingredients)
	expected := 3

	if result != expected {
		t.Errorf("Expected %d to be equal %d", result, expected)
	}
}
