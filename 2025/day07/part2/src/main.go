package main

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func findTheBeam(line string) (int, error) {
	for i, char := range line {
		if char == 'S' {
			return i, nil
		}
	}
	return 0, errors.New("couldn't find the beam")
}

func countTimelines(lines []string, x, y int, memo map[[2]int]int) int {
    if y >= len(lines) || x < 0 || x >= len(lines[0]) {
        return 1
    }

    key := [2]int{x, y}
    if val, ok := memo[key]; ok {
        return val
    }

    cell := lines[y][x]

    var result int
    switch cell {
    case '.','S':
        result = countTimelines(lines, x, y+1, memo)

    case '^':
        left  := countTimelines(lines, x-1, y+1, memo)
        right := countTimelines(lines, x+1, y+1, memo)
        result = left + right
    }

    memo[key] = result
    return result
}

func FireTheBeam(input string) int {
    lines := strings.Split(strings.TrimSpace(input), "\n")
    for i := range lines {
        lines[i] = strings.TrimSpace(lines[i])
    }

    startX, err := findTheBeam(lines[0])
    if err != nil {
        panic("Couldn't find the beam")
    }

    memo := make(map[[2]int]int)
    return countTimelines(lines, startX, 0, memo)
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

	fmt.Println("Calculated:", FireTheBeam(string(data)))
}
