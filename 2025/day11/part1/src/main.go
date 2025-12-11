package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func GetPaths(input string) int {
	graph := map[string][]string{}

	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		idx := strings.Index(line, ":")
		label := strings.TrimSpace(line[:idx])
		rest := strings.TrimSpace(line[idx+1:])
		outs := strings.Fields(rest)

		graph[label] = outs
	}

	totalPaths := 0

	var dfs func(node string)
	dfs = func(node string) {
		if node == "out" {
			totalPaths++
			return
		}

		for _, next := range graph[node] {
			dfs(next)
		}
	}

	for _, start := range graph["you"] {
		dfs(start)
	}

	return totalPaths
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

	fmt.Println("Calculated:", GetPaths(string(data)))
}