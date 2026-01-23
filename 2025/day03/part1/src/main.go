package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func FindJoltage(batteries string) int {
	max := 0
	for i := 0; i < len(batteries) - 1; i++ {
		first := string(batteries[i])

		for j := len(batteries) - 1; j > i; j-- {
			second := string(batteries[j])

			numRes := first + second

			numi, err := strconv.Atoi(numRes)
			if err != nil {
				panic(err)
			}

			if numi > max {
				max = numi
			}
		}
	}
	return max
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

	batteries := strings.SplitSeq(string(data), "\n")

	out := 0
	for battery := range batteries {
		out += FindJoltage(battery)
	}

	fmt.Println("Joltage:", out)
}
