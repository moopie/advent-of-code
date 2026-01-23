package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func FindJoltage(batteries string) int {
	if len(batteries) == 0 {
		return 0
	}

	total := 12
    remove := len(batteries) - total
    stack := make([]int, 0, len(batteries))

    for i := 0; i < len(batteries); i++ {
        c, err := strconv.Atoi(string(batteries[i]))
		if err != nil {
			panic(err)
		}
        for len(stack) > 0 && remove > 0 && stack[len(stack)-1] < c {
            stack = stack[:len(stack)-1]
            remove--
        }
        stack = append(stack, c)
    }

	result := 0
	for _, n := range stack[:total] {
		result = result*10 + n
	}

	return result
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
