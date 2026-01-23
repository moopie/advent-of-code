package main

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func FindSeating(input string) int {
	// sanitize input
	lines := strings.Split(strings.TrimSpace(input), "\n")

	tiles := [][2]int{}
	for _, line := range lines {
		line = strings.TrimSpace(line)
		nums := strings.Split(line, ",")
		n1, err1 := strconv.Atoi(nums[0])
		n2, err2 := strconv.Atoi(nums[1])
		if err1 != nil || err2 != nil {
			panic("Should be two numbers per tile")
		}

		tiles = append(tiles, [2]int{n1, n2})
	}

	areaSize := []int{}
	for i := range len(tiles) {
		for j := i + 1; j < len(tiles); j++ {
			tile1 := tiles[i]
			tile2 := tiles[j]

			x := abs(tile2[0] - tile1[0])
			y := abs(tile2[1] - tile1[1])

			if x == 0 || y == 0 {
				continue
			}

			areaSize = append(areaSize, (x+1)*(y+1))
		}
	}
	sort.Ints(areaSize)
	return areaSize[len(areaSize)-1]
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

	fmt.Println("Calculated:", FindSeating(string(data)))
}
