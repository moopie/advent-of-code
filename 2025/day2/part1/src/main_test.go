package main

import (
	"testing"
	"slices"
)

func TestIsValidNumber(t *testing.T) {
	validNumbers := []int{101, 110, 111, 112, 1234, 5678, 91011, 1213}
	invalidNumbers := []int{1111, 3434, 5656, 7878}

	for _, n := range validNumbers {
		if !isValidNumber(n) {
			t.Errorf("isValidNumber(%d) = false; want true", n)
		}
	}

	for _, n := range invalidNumbers {
		if isValidNumber(n) {
			t.Errorf("isValidNumber(%d) = true; want false", n)
		}
	}
}

func TestRange1(t *testing.T) {
	numRange := "95-115";
	expected := []int{99}

	result := parseRange(numRange)
	if !slices.Equal(result, expected) {
		t.Errorf("parseRange(%q) = %v; want %v", numRange, result, expected)
	}
}

func TestRange2(t *testing.T) {
	numRange := "998-1012";
	expected := []int{1010}

	result := parseRange(numRange)
	if !slices.Equal(result, expected) {
		t.Errorf("parseRange(%q) = %v; want %v", numRange, result, expected)
	}
}

func TestRange3(t *testing.T) {
	numRange := "38593856-38593862";
	expected := []int{38593859}

	result := parseRange(numRange)
	if !slices.Equal(result, expected) {
		t.Errorf("parseRange(%q) = %v; want %v", numRange, result, expected)
	}
}

func TestLine(t *testing.T) {
	line := "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
	expectedCount := 1227775554

	result := parseLine(line)
	if result != expectedCount {
		t.Errorf("parseLine(%q) = %d; want %d", line, result, expectedCount)
	}
}