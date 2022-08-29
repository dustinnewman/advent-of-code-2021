package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func simulate(fishes [9]int, days int) int {
	for i := 0; i < days; i++ {
		pregnant := fishes[0]
		for j := 1; j < 9; j++ {
			fishes[j-1] = fishes[j]
		}
		fishes[8] = pregnant
		fishes[6] += pregnant
	}
	sum := 0
	for _, count := range fishes {
		sum += count
	}
	return sum
}

func main() {
	lines := strings.Split(input, ",")
	fishes := [9]int{0}
	for _, line := range lines {
		i, _ := strconv.Atoi(line)
		fishes[i]++
	}
	fmt.Println("Part 1:", simulate(fishes, 80))
	fmt.Println("Part 2:", simulate(fishes, 256))
}
