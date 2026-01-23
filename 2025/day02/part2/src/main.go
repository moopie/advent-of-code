package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
	"path/filepath"
)

func isValidNumber(n int) bool {
    numStr := strconv.Itoa(n)
    strLen := len(numStr)

    for patLen := 1; patLen <= strLen/2; patLen++ {
        if strLen%patLen != 0 {
            continue
        }

        ok := true
        for i := patLen; i < strLen; i++ {
            if numStr[i] != numStr[i%patLen] {
                ok = false
                break
            }
        }

        if ok {
            return false
        }
    }

    return true
}

func parseRange(r string) []int {
	nums := []int{}

	parts := strings.Split(r, "-")

	if len(parts) != 2 {
		return []int{}
	}

	start, err1 := strconv.Atoi(parts[0])
	end, err2 := strconv.Atoi(parts[1])

	if err1 != nil || err2 != nil {
		return []int{}
	}

	for i := start; i <= end; i++ {
		if !isValidNumber(i) {
			nums = append(nums, i)
		}
	}

	return nums
}

func parseLine(line string) int {
	ranges := strings.SplitSeq(line, ",")
	results := []int{}
	accum  := 0

	for r := range ranges {
		res := parseRange(r)
		results = append(results, res...)
	}

	for _, n := range results {
		accum += n
	}

	return accum
}

func main() {
	// path to the running binary
	exe, err := os.Executable()
	if err != nil {
		panic(err)
	}

	// directory where the binary lives
	exeDir := filepath.Dir(exe)

	// build absolute path to your file
	path := filepath.Join(exeDir, "input.txt")

	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}

	result := parseLine(strings.TrimSpace(string(data)));

	fmt.Println("Result:", result)
}
