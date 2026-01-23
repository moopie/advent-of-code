package main

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

type Box struct {
	x, y, z int
}

type Pair struct {
	i, j int
	dist int
}

func find(parent []int, x int) int {
	if parent[x] != x {
		parent[x] = find(parent, parent[x])
	}
	return parent[x]
}

func unite(parent, size []int, a, b int) {
	ra := find(parent, a)
	rb := find(parent, b)
	if ra == rb {
		return
	}
	if size[ra] < size[rb] {
		ra, rb = rb, ra
	}
	parent[rb] = ra
	size[ra] += size[rb]
}

func GetLengthFromWall(input string) int {
	// sanitize the input
	input = strings.TrimSpace(input)
	lines := strings.Split(input, "\n")
	for i, line := range lines {
		lines[i] = strings.TrimSpace(line)
	}

	boxes := []Box{}
	for _, line := range lines {
		parts := strings.Split(line, ",")
		if len(parts) != 3 {
			panic("Length of each box is 3")
		}
		a, err1 := strconv.Atoi(parts[0])
		b, err2 := strconv.Atoi(parts[1])
		c, err3 := strconv.Atoi(parts[2])

		if err1 != nil || err2 != nil || err3 != nil {
			panic("Something went wrong")
		}
		boxes = append(boxes, Box{a, b, c})
	}

	pairs := []Pair{}
	for i := range boxes {
		for j := i + 1; j < len(boxes); j++ {
			dx := boxes[i].x - boxes[j].x
			dy := boxes[i].y - boxes[j].y
			dz := boxes[i].z - boxes[j].z

			dist := dx*dx + dy*dy + dz*dz
			pairs = append(pairs, Pair{i, j, dist})
		}
	}

	sort.Slice(pairs, func(i, j int) bool {
		return pairs[i].dist < pairs[j].dist
	})

	n := len(boxes)
	parent := make([]int, n)
	size := make([]int, n)
	for i := range n {
		parent[i] = i
		size[i] = 1
	}

	components := n

	lasti, lastj := -1, -1

	for _, p := range pairs {
		ra := find(parent, p.i)
		rb := find(parent, p.j)

		if ra == rb {
			continue
		}

		unite(parent, size, ra, rb)
		components--

		lasti, lastj = p.i, p.j

		if components == 1 {
			break
		}
	}

	answer := boxes[lasti].x * boxes[lastj].x
	return answer
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

	fmt.Println("Calculated:", GetLengthFromWall(string(data)))
}
