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
	eqstrs := map[int]string{}

	for line := range strings.SplitSeq(eqs, "\n") {
		parts := strings.Fields(line)

		for i, part := range parts {
			item, ok := eqstrs[i]
			if ok {
				eqstrs[i] = item + " " + part + " "
			} else {
				eqstrs[i] = part + " "
			}
		}
	}

	for _, eqstr := range eqstrs {
		numbers := []int{}
		operand := ""

		for _, part := range strings.Fields(eqstr) {
			n, err := strconv.Atoi(part)
			if err != nil {
				// we assume its an operand and there's only one per equation
				if operand != "" {
					panic("Something went wrong")
				}
				operand = part
			} else {
				numbers = append(numbers, n)
			}
		}

		eq := Equation{
			params: numbers,
			op:     operand,
		}

		equations = append(equations, eq)
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
