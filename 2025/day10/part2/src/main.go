package main

import (
	"fmt"
	"math"
	"math/bits"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"unicode"
)

type Machine struct {
	Buttons []uint64 // bit i => affects counter i
	Joltage []int
	Solver  *ParitySolver
}

func parseMachines(input string) ([]*Machine, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	var machines []*Machine

	for _, line := range lines {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		lc := strings.Index(line, "{")
		rc := strings.Index(line, "}")
		jolts := parseIntList(line[lc+1 : rc])
		m := &Machine{Joltage: jolts}

		n := len(jolts)

		rb := strings.Index(line, "]")
		rest := line[rb+1:]

		for {
			l := strings.Index(rest, "(")
			r := strings.Index(rest, ")")
			if l == -1 || r == -1 {
				break
			}
			btn := parseIntList(rest[l+1 : r])
			var mask uint64
			for _, i := range btn {
				mask |= 1 << uint(i) // bit i = counter i
			}
			m.Buttons = append(m.Buttons, mask)
			rest = rest[r+1:]
		}

		m.Solver = NewParitySolver(m.Buttons, n)
		machines = append(machines, m)
	}

	return machines, nil
}

func parseIntList(s string) []int {
	var res []int
	num := ""
	for _, r := range s {
		if unicode.IsDigit(r) {
			num += string(r)
		} else if num != "" {
			v, _ := strconv.Atoi(num)
			res = append(res, v)
			num = ""
		}
	}
	if num != "" {
		v, _ := strconv.Atoi(num)
		res = append(res, v)
	}
	return res
}

// Parity solver (GF(2))
type ParitySolver struct {
	m, n int
	rows []uint64 // row i: which buttons affect counter i
}

func NewParitySolver(buttons []uint64, m int) *ParitySolver {
	n := len(buttons)
	rows := make([]uint64, m)
	for c := 0; c < m; c++ {
		for j := 0; j < n; j++ {
			if (buttons[j]>>uint(c))&1 == 1 {
				rows[c] |= 1 << uint(j)
			}
		}
	}
	return &ParitySolver{m: m, n: n, rows: rows}
}

// returns all x such that A*x = rhs (mod 2)
func (ps *ParitySolver) AllSolutions(rhs uint64) []uint64 {
	A := append([]uint64(nil), ps.rows...)
	b := make([]uint8, ps.m)
	for i := 0; i < ps.m; i++ {
		if (rhs>>uint(i))&1 == 1 {
			b[i] = 1
		}
	}

	pivotCol := make([]int, ps.m)
	for i := range pivotCol {
		pivotCol[i] = -1
	}

	rank := 0
	for col := 0; col < ps.n && rank < ps.m; col++ {
		pivot := -1
		for r := rank; r < ps.m; r++ {
			if (A[r]>>uint(col))&1 == 1 {
				pivot = r
				break
			}
		}
		if pivot == -1 {
			continue
		}

		A[rank], A[pivot] = A[pivot], A[rank]
		b[rank], b[pivot] = b[pivot], b[rank]
		pivotCol[rank] = col

		for r := 0; r < ps.m; r++ {
			if r != rank && ((A[r]>>uint(col))&1) == 1 {
				A[r] ^= A[rank]
				b[r] ^= b[rank]
			}
		}
		rank++
	}

	for r := 0; r < ps.m; r++ {
		if A[r] == 0 && b[r] == 1 {
			return nil // no solution
		}
	}

	isPivot := make([]bool, ps.n)
	for r := 0; r < rank; r++ {
		isPivot[pivotCol[r]] = true
	}

	var free []int
	for j := 0; j < ps.n; j++ {
		if !isPivot[j] {
			free = append(free, j)
		}
	}

	var particular uint64
	for r := rank - 1; r >= 0; r-- {
		col := pivotCol[r]
		sum := bits.OnesCount64(A[r]&particular) & 1
		if uint8(sum) != b[r] {
			particular |= 1 << uint(col)
		}
	}

	nulls := []uint64{}
	for _, f := range free {
		v := uint64(1) << uint(f)
		for r := rank - 1; r >= 0; r-- {
			col := pivotCol[r]
			if bits.OnesCount64(A[r]&v)&1 == 1 {
				v |= 1 << uint(col)
			}
		}
		nulls = append(nulls, v)
	}

	sols := []uint64{particular}
	for _, n := range nulls {
		cur := len(sols)
		for i := 0; i < cur; i++ {
			sols = append(sols, sols[i]^n)
		}
	}
	return sols
}

// Solver
func parityMask(j []int) uint64 {
	var m uint64
	for i, v := range j {
		if v&1 == 1 {
			m |= 1 << uint(i)
		}
	}
	return m
}

func key(j []int) string {
	var sb strings.Builder
	for i, v := range j {
		if i > 0 {
			sb.WriteByte(',')
		}
		sb.WriteString(strconv.Itoa(v))
	}
	return sb.String()
}

func solveMachine(m *Machine) int {
	cache := map[string]int{}

	var dfs func([]int) int
	dfs = func(j []int) int {
		k := key(j)
		if v, ok := cache[k]; ok {
			return v
		}

		allZero := true
		for _, x := range j {
			if x != 0 {
				allZero = false
				break
			}
		}
		if allZero {
			return 0
		}

		best := math.MaxInt
		solutions := m.Solver.AllSolutions(parityMask(j))
		if solutions == nil {
			cache[k] = math.MaxInt
			return math.MaxInt
		}

		for _, sol := range solutions {
			b2 := append([]int(nil), j...)
			for i := 0; i < len(m.Buttons); i++ {
				if (sol>>uint(i))&1 == 1 {
					btn := m.Buttons[i]
					for c := range b2 {
						if (btn>>uint(c))&1 == 1 {
							b2[c]--
						}
					}
				}
			}

			ok := true
			for i := range b2 {
				if b2[i] < 0 || b2[i]&1 == 1 {
					ok = false
					break
				}
				b2[i] >>= 1
			}
			if !ok {
				continue
			}

			res := dfs(b2)
			if res != math.MaxInt {
				cost := bits.OnesCount64(sol) + 2*res
				if cost < best {
					best = cost
				}
			}
		}

		cache[k] = best
		return best
	}

	ans := dfs(m.Joltage)
	if ans == math.MaxInt {
		return 0
	}
	return ans
}

func ToggleLights(input string) int {
	machines, _ := parseMachines(input)
	sum := 0
	for _, m := range machines {
		sum += solveMachine(m)
	}
	return sum
}

func main() {
	exe, _ := os.Executable()
	path := filepath.Join(filepath.Dir(exe), "input.txt")
	data, _ := os.ReadFile(path)
	fmt.Println("Calculated:", ToggleLights(string(data)))
}