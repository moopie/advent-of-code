package main

import (
	"fmt"
	"math"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

type Light struct {
	mask    int
	presses []int
}

func (l Light) GetSteps() int {
	best := math.MaxInt
	visited := map[int]int{}

	var dfs func(state int, idx int, presses int)

	dfs = func(state int, idx int, presses int) {
		if state == l.mask {
			if presses < best {
				best = presses
			}
			return
		}

		if presses >= best {
			return
		}

		if prev, ok := visited[state]; ok && prev <= presses {
			return
		}
		visited[state] = presses

		for i := idx; i < len(l.presses); i++ {
			dfs(state^l.presses[i], i+1, presses+1)
		}
	}

	dfs(0, 0, 0)

	if best == math.MaxInt {
		return -1
	}
	return best
}

func ToggleLights(input string) int {
	lights := []Light{}
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		line = strings.TrimSpace(line)
		expectedMask := []int{}
		lastLoc := 0
		for i, char := range line {
			switch char {
			case ']':
				lastLoc = i + 1
				goto exit
			case '[':
				continue
			case '.':
				expectedMask = append(expectedMask, 0)
			case '#':
				expectedMask = append(expectedMask, 1)
			}
		}
	exit:
		mask := 0
		for i, b := range expectedMask {
			if b == 1 {
				mask |= 1 << i
			}
		}

		presses := []int{}
		numbers := []int{}
		state := 0
		for i, char := range line {
			if i <= lastLoc {
				continue
			}

			switch char {
			case '{', '}':
				state = 0
				continue
			case '(':
				state = 1
				continue
			case ',':
				state = 2
				continue
			case ')':
				state = 3
				continue
			}

			switch state {
			case 0:
				continue
			case 1, 2:
				n, err := strconv.Atoi(string(char))
				if err != nil {
					panic(err)
				}
				numbers = append(numbers, n)
			case 3:
				pMask := 0
				for _, num := range numbers {
					pMask |= 1 << num
				}
				if pMask != 0 {
					presses = append(presses, pMask)
				}
				numbers = numbers[:0]
			}
		}
		lights = append(lights, Light{
			mask:    mask,
			presses: presses,
		})
	}

	result := 0
	for _, light := range lights {
		result += light.GetSteps()
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

	fmt.Println("Calculated:", ToggleLights(string(data)))
}
