package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

type Equation struct {
	params []int
	op     string
}

func (eq Equation) Calculate() int {
	collector := 0
	for _, n := range eq.params {
		if collector == 0 {
			collector = n
			continue
		}
		switch eq.op {
		case "+":
			collector = collector + n
		case "*":
			collector = collector * n
		default:
			panic("Don't know by what to aggregate all these params!")
		}
	}

	return collector
}

func CalculateEqs(eqs string) int {
	equations := []Equation{}

	lines := strings.Split(eqs, "\n")

	slen := 0
	for _, line := range lines {
		if len(line) > slen {
			slen = len(line)
		}
	}

	stack := []int{}
	for i := slen - 1; i >= 0; i-- {
		numbers := []int{}
		operator := ""
		for j := range lines {
			if len(lines[j]) == 0 || len(lines[j]) < slen {
				continue
			}
			char := lines[j][i]

			switch string(char) {
			case " ":
				continue
			case "\t":
				continue
			case ".":
				continue
			case "*":
				operator = "*"
			case "+":
				operator = "+"
			default:
				n, err := strconv.Atoi(string(char))
				if err != nil {
					panic(err)
				}
				numbers = append(numbers, n)
			}
		}

		total := 0
		for _, num := range numbers {
			total = total*10 + num
		}
		stack = append(stack, total)

		if operator != "" {
			nums := []int{}
			for _, num := range stack {
				if num != 0 {
					nums = append(nums, num)
				}
			}
			equations = append(equations, Equation{
				params: nums,
				op:     operator,
			})
			clear(stack)
		}
	}

	aggregator := 0
	for _, eq := range equations {
		aggregator += eq.Calculate()
	}

	return aggregator
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

	fmt.Println("Calculated:", CalculateEqs(string(data)))
}
