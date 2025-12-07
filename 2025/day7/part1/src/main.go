package main

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

func findTheBeam(line string) (int, error) {
	for i, char := range line {
		if char == 'S' {
			return i, nil
		}
	}
	return 0, errors.New("couldn't find the beam")
}

func FireTheBeam(input string) int {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	for i, line := range lines {
		lines[i] = strings.TrimSpace(line)
	}
	posx, err := findTheBeam(lines[0])
	beams := []int{posx}
	if err != nil {
		panic("Couldn't find the beam")
	}
	splits := 0
	for posy := range len(lines) {
		newbeams := []int{}
		beamstoremove := []int{}
		for _, beam := range beams {
			str := string(lines[posy][beam])
			switch str {
			case "^":
				splits++
				beamstoremove = append(beamstoremove, beam)
				newbeams = append(newbeams, beam-1)
				newbeams = append(newbeams, beam+1)
			default:
				continue
			}
		}
		if len(beamstoremove) != 0 {
			for _, beam := range beamstoremove {
				for j, oldbeam := range beams {
					if beam == oldbeam {
						beams = append(beams[:j], beams[j+1:]...)
					}
				}
			}
		}
		if len(newbeams) != 0 {
			for _, newbeam := range newbeams {
				if !slices.Contains(beams, newbeam) {
					beams = append(beams, newbeam)
				}
			}
		}
	}
	return splits
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

	fmt.Println("Calculated:", FireTheBeam(string(data)))
}
