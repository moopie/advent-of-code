package main

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

type Present struct {
	shape [3][3]rune
}

type Region struct {
	w, h     int
	presents []Present
}

func place(grid [][]rune, p Present, x, y int) {
	for dy := 0; dy < 3; dy++ {
		for dx := 0; dx < 3; dx++ {
			if p.shape[dy][dx] == '#' {
				grid[y+dy][x+dx] = '#'
			}
		}
	}
}

func remove(grid [][]rune, p Present, x, y int) {
	for dy := 0; dy < 3; dy++ {
		for dx := 0; dx < 3; dx++ {
			if p.shape[dy][dx] == '#' {
				grid[y+dy][x+dx] = '.'
			}
		}
	}
}

func (r Region) isValid() bool {
	totalArea := 0
	for _, p := range r.presents {
		for i := range 3 {
			for j := range 3 {
				if p.shape[i][j] == '#' {
					totalArea++
				}
			}
		}
	}
	if totalArea > r.w*r.h {
		return false
	}

	grid := make([][]rune, r.h)
	for y := 0; y < r.h; y++ {
		grid[y] = make([]rune, r.w)
		for x := 0; x < r.w; x++ {
			grid[y][x] = '.'
		}
	}

	var dfs func(idx int) bool

	dfs = func(idx int) bool {
		if idx == len(r.presents) {
			return true
		}

		p := r.presents[idx]

		for rot := 0; rot < 4; rot++ {
			variant := p.rotate(rot)

			for y := 0; y < r.h; y++ {
				for x := 0; x < r.w; x++ {
					if variant.canPlaceAt(grid, x, y) {
						place(grid, variant, x, y)

						if dfs(idx + 1) {
							return true
						}

						remove(grid, variant, x, y)
					}
				}
			}
		}
		return false
	}

	return dfs(0)
}

func (present Present) rotate(times int) Present {
	p := present

	for t := 0; t < times; t++ {
		var next Present
		for row := range 3 {
			for col := range 3 {
				next.shape[col][2-row] = p.shape[row][col]
			}
		}
		p = next
	}

	return p
}

func (p Present) canPlaceAt(grid [][]rune, x, y int) bool {
	hlen := len(grid)
	wlen := len(grid[0])

	if x < 0 || y < 0 || x+3 > wlen || y+3 > hlen {
		return false
	}

	for dy := range 3 {
		for dx := range 3 {
			if p.shape[dy][dx] == '#' && grid[y+dy][x+dx] == '#' {
				return false
			}
		}
	}
	return true
}

func parse(input string) ([]Region, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	presents := []Present{}
	regions := []Region{}
	buf := []rune{}
	mode := "shape"

	for _, line := range lines {
		line = strings.TrimSpace(line)

		if line == "" {
			if len(buf) == 9 {
				shape := [3][3]rune{}
				for col := range 3 {
					for row := range 3 {
						first := buf[0]
						shape[col][row] = first
						buf = buf[1:]
					}
				}
				presents = append(presents, Present{shape: shape})
				buf = buf[:0]
			}
			continue
		}

		if strings.Contains(line, "x") {
			mode = "region"
		}

		if mode == "shape" {
			if strings.HasSuffix(line, ":") {
				continue
			}
			for _, c := range line {
				str := string(c)
				switch c {
				case '#', '.':
					buf = append(buf, c)
				default:
					return nil, errors.New("invalid shape char" + str)
				}
			}
		} else {
			parts := strings.Split(line, ":")
			size := parts[0]
			counts := strings.Fields(parts[1])

			sizeDelim := strings.Split(size, "x")
			w, err := strconv.Atoi(sizeDelim[0])
			h, err := strconv.Atoi(sizeDelim[1])

			if err != nil {
				return nil, err
			}

			ps := []Present{}
			for i, c := range counts {
				n, _ := strconv.Atoi(c)
				for n > 0 {
					ps = append(ps, presents[i])
					n--
				}
			}

			regions = append(regions, Region{w: w, h: h, presents: ps})
		}
	}
	return regions, nil
}

func ValidRegions(input string) int {
	regions, err := parse(input)
	if err != nil {
		fmt.Println(err)
		return 0
	}
	validCount := 0
	for _, reg := range regions {
		v := reg.isValid()
		if v {
			validCount++
		}
	}
	return validCount
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

	regions, err := parse(string(data))
	if err != nil {
		panic(err)
	}

	count := 0
	for _, r := range regions {
		ok := r.isValid()
		if ok {
			count++
		}
	}
	fmt.Println("Answer:", count)
}
