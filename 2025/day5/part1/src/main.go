package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func TestIngredient(ranges []string, ingredient int) bool {
	for _, r := range ranges {
		validRanges := strings.Split(r, "-")

		if len(validRanges) != 2 {
			panic("Array should contain two numbers")
		}

		low, err1 := strconv.Atoi(validRanges[0])
		high, err2 := strconv.Atoi(validRanges[1])

		if err1 != nil || err2 != nil {
			panic("Both ranges should be numbers")
		}

		if ingredient >= low && ingredient <= high {
			return true
		}
	}
	return false
}

func GetValidIngredientAmount(ranges []string, ingredients []int) int {
	totalValid := 0
	for _, ingredient := range ingredients {
		if TestIngredient(ranges, ingredient) {
			totalValid++
		}
	}
	return totalValid
}

func main() {
	exe, err := os.Executable()
	if err != nil {
		panic(err)
	}

	exeDir := filepath.Dir(exe)

	path := filepath.Join(exeDir, "input.txt")

	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}

	validRanges := []string{}
	ingredients := []int{}

	lines := strings.SplitSeq(string(data), "\n")

	for line := range lines {
		if len(line) == 0 {
			continue
		}
		if strings.ContainsRune(line, '-') {
			validRanges = append(validRanges, line)
		} else {
			ing, err := strconv.Atoi(line)

			if err != nil {
				panic(err)
			}

			ingredients = append(ingredients, ing)
		}
	}

	validCount := GetValidIngredientAmount(validRanges, ingredients)

	fmt.Println("Valid ingredients: ", validCount)
}
