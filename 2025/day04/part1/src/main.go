package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func CountPaper(lines []string, row, col int) int {
	indexesToSearch := [][2]int{
		{-1, -1}, {-1, 0}, {-1, 1},
		{0, -1}, {0, 1},
		{1, -1}, {1, 0}, {1, 1},
	}
	papers := 0
	for _, offset := range indexesToSearch {
		newRow := row + offset[0]
		newCol := col + offset[1]
		if newRow < 0 || newRow >= len(lines) || newCol < 0 || newCol >= len(lines[0]) {
			continue
		}

		loc := string(lines[newRow][newCol])
		switch loc {
		case "@":
			papers++
		default:
			continue
		}
	}
	return papers
}

func CountForklifts(input string) int {
	forklifts := 0
	lines := strings.Split(strings.TrimSpace(input), "\n")
	for i, line := range lines {
		lines[i] = strings.TrimSpace(line)
	}
	rows := len(lines)
	cols := len(lines[0])

	for i := range rows {
		for j := range cols {
			location := string(lines[i][j])

			switch location {
			case ".":
				continue
			case "@":
				papers := CountPaper(lines, i, j)
				if papers < 4 {
					forklifts++
				}
			default:
				panic("Unknown location: " + location)
			}
		}
	}
	return forklifts
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

	forklifts := CountForklifts(string(data))

	fmt.Println("Forklifts:", forklifts)
}
