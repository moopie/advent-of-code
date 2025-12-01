package main

import (
	"os"
	"strings"
	"strconv"
)

func main() {
	content, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	current_position := 50
	zeroes := 0
	lines := strings.SplitSeq(string(content), "\n")
	for line := range lines {
		if len(line) == 0 {
			continue
		}

		direction := line[:1]
		distance, err := strconv.Atoi(line[1:])

		if err != nil {
			panic(err)
		}

		for i := 0; i < distance; i++ {
			switch direction {
			case "L":
				current_position--
			case "R":
				current_position++
			}

			if (current_position < 0) {
				current_position = 99
			} else if (current_position > 99) {
				current_position = 0
			}

			if current_position == 0 {
				zeroes++
			}
		}
	}

	println("Total zeroes:", zeroes)
}