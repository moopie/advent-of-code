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
		idx := strings.Index(line, ":")
		label := strings.TrimSpace(line[:idx])
		outs := strings.Fields(strings.TrimSpace(line[idx+1:]))
		graph[label] = outs
	}

	canReach := func(target string) map[string]bool {
		reach := make(map[string]bool)
		var rev map[string][]string = make(map[string][]string)

		for from, tos := range graph {
			for _, to := range tos {
				rev[to] = append(rev[to], from)
			}
		}

		var dfs func(string)
		dfs = func(n string) {
			if reach[n] {
				return
			}
			reach[n] = true
			for _, prev := range rev[n] {
				dfs(prev)
			}
		}

		dfs(target)
		return reach
	}

	canReachOut := canReach("out")
	canReachDAC := canReach("dac")
	canReachFFT := canReach("fft")

	type State struct {
		node    string
		dacSeen bool
		fftSeen bool
	}

	dp := make(map[State]int)

	var dfs func(node string, dacSeen, fftSeen bool) int

	dfs = func(node string, dacSeen, fftSeen bool) int {
		if !canReachOut[node] {
			return 0
		}
		if !dacSeen && !canReachDAC[node] {
			return 0
		}
		if !fftSeen && !canReachFFT[node] {
			return 0
		}

		// update state
		if node == "dac" {
			dacSeen = true
		}
		if node == "fft" {
			fftSeen = true
		}

		if node == "out" {
			if dacSeen && fftSeen {
				return 1
			}
			return 0
		}

		st := State{node, dacSeen, fftSeen}
		if val, exists := dp[st]; exists {
			return val
		}

		total := 0
		for _, next := range graph[node] {
			total += dfs(next, dacSeen, fftSeen)
		}

		dp[st] = total
		return total
	}

	total := 0
	for _, start := range graph["svr"] {
		total += dfs(start, false, false)
	}

	return total
}

func main() {
	exe, _ := os.Executable()
	path := filepath.Join(filepath.Dir(exe), "input.txt")
	data, _ := os.ReadFile(path)

	fmt.Println("Calculated:", GetPaths(string(data)))
}