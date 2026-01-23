package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

type Pair struct {
	low int;
	high int;
}

func InRange(a, b Pair) bool {
    return a.low <= b.high && a.high >= b.low
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func GetValidIngredientAmount(ranges []string) int {
	totalValid := 0
	validPairs := []*Pair{}

	for _, r := range ranges {
		bounds := strings.Split(r, "-")
		if len(bounds) > 2 {
			panic("Too many ranges!")
		}
		lowStr := bounds[0]
		highStr := bounds[1]

		low, err1 := strconv.Atoi(lowStr)
		high, err2 := strconv.Atoi(highStr)

		if err1 != nil || err2 != nil {
			panic("Numbers are not numbers!")
		}

		pair := Pair {
			low: low,
			high: high,
		}

		validPairs = append(validPairs, &pair)
	}

	rerun:
	overlaps := 0
	for i, a := range validPairs {
		if a == nil {
			continue
		}
		for j := i + 1; j < len(validPairs); j++ {
			b := validPairs[j]
			if b == nil {
				continue
			}
			if InRange(*a, *b) {
				validPairs[i].low = min(a.low, b.low)
				validPairs[i].high = max(a.high, b.high)
				validPairs[j] = nil
				overlaps++
			}
		}
	}

	if overlaps > 0 {
		goto rerun
	}

	for _, valid := range validPairs {
		if valid != nil {
			totalValid += valid.high - valid.low + 1
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

	validCount := GetValidIngredientAmount(validRanges)

	fmt.Println("Valid ingredients: ", validCount)
}
