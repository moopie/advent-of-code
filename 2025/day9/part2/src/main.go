package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

type Point struct {
	x, y int
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

type Cell struct {
	x, y int
}

func floodFillExterior(grid [][]rune, width, height int) {
	queue := make([]Cell, 0, 1024)

	push := func(x, y int) {
		if x < 0 || y < 0 || x >= width || y >= height {
			return
		}
		if grid[y][x] != '.' {
			return
		}
		grid[y][x] = 'O'
		queue = append(queue, Cell{x, y})
	}

	// seed from borders
	for x := 0; x < width; x++ {
		push(x, 0)
		push(x, height-1)
	}
	for y := 0; y < height; y++ {
		push(0, y)
		push(width-1, y)
	}

	// BFS
	for len(queue) > 0 {
		c := queue[0]
		queue = queue[1:]

		push(c.x+1, c.y)
		push(c.x-1, c.y)
		push(c.x, c.y+1)
		push(c.x, c.y-1)
	}
}

func parseInput(input string) ([]Point, int, int) {
	fmt.Println("Parsing input...")
	start := time.Now()

	lines := strings.Split(strings.TrimSpace(input), "\n")
	points := []Point{}

	maxX, maxY := 0, 0
	for _, line := range lines {
		parts := strings.Split(strings.TrimSpace(line), ",")
		x, _ := strconv.Atoi(parts[0])
		y, _ := strconv.Atoi(parts[1])
		points = append(points, Point{x, y})
		maxX = max(maxX, x)
		maxY = max(maxY, y)
	}

	fmt.Println("Parsed", len(points), "red tiles in", time.Since(start))
	return points, maxX + 3, maxY + 3
}

func printGrid(grid [][]rune) {
	for _, row := range grid {
		for _, c := range row {
			fmt.Print(string(c))
		}
		fmt.Println()
	}
}

func drawLine(grid [][]rune, a, b Point) {
	if a.x == b.x {
		for y := min(a.y, b.y); y <= max(a.y, b.y); y++ {
			if grid[y][a.x] == '.' {
				grid[y][a.x] = 'X'
			}
		}
	} else if a.y == b.y {
		for x := min(a.x, b.x); x <= max(a.x, b.x); x++ {
			if grid[a.y][x] == '.' {
				grid[a.y][x] = 'X'
			}
		}
	}
}

func FindSeating(input string) int {
	totalStart := time.Now()

	reds, width, height := parseInput(input)

	fmt.Println("Creating grid:", width, "x", height)
	grid := make([][]rune, height)
	for y := range grid {
		grid[y] = make([]rune, width)
		for x := range grid[y] {
			grid[y][x] = '.'
		}
	}

	// draw boundary
	fmt.Println("Drawing green boundary...")
	start := time.Now()
	for i := 0; i < len(reds); i++ {
		a := reds[i]
		b := reds[(i+1)%len(reds)]
		drawLine(grid, a, b)
	}
	fmt.Println("Boundary drawn in", time.Since(start))

	// mark red tiles
	fmt.Println("Marking red tiles...")
	for _, p := range reds {
		grid[p.y][p.x] = '#'
	}

	fmt.Println("Grid after boundary construction:")
	//printGrid(grid)

	// flood-fill exterior
	fmt.Println("Flood-filling exterior...")
	start = time.Now()

	floodFillExterior(grid, width, height)

	fmt.Println("Flood fill completed in", time.Since(start))

	// interior becomes green
	fmt.Println("Marking interior as green...")
	for y := range grid {
		for x := range grid[y] {
			if grid[y][x] == '.' {
				grid[y][x] = 'X'
			}
		}
	}

	//fmt.Println("Final grid (red + green only):")
	//printGrid(grid)

	// find largest rectangle
	fmt.Println("Searching for largest valid rectangle...")
	start = time.Now()
	best := 0

	for i := 0; i < len(reds); i++ {
		fmt.Printf("Rectangle scan progress: %d / %d\n", i, len(reds))
		for j := i + 1; j < len(reds); j++ {
			a := reds[i]
			b := reds[j]

			dx := abs(a.x - b.x) + 1
			dy := abs(a.y - b.y) + 1
			if dx <= 1 || dy <= 1 {
				continue
			}

			x1, x2 := min(a.x, b.x), max(a.x, b.x)
			y1, y2 := min(a.y, b.y), max(a.y, b.y)

			ok := true
			for y := y1; y <= y2 && ok; y++ {
				for x := x1; x <= x2; x++ {
					if grid[y][x] != 'X' && grid[y][x] != '#' {
						ok = false
						break
					}
				}
			}

			if ok {
				area := dx * dy
				if area > best {
					best = area
				}
			}
		}
	}

	fmt.Println("Rectangle search completed in", time.Since(start))
	fmt.Println("Total runtime:", time.Since(totalStart))
	fmt.Println("Max area:", best)

	return best
}

func main() {
	exe, err := os.Executable()
	if err != nil {
		panic(err)
	}

	input, err := os.ReadFile(filepath.Join(filepath.Dir(exe), "input.txt"))
	if err != nil {
		panic(err)
	}

	fmt.Println("Answer:", FindSeating(string(input)))
}