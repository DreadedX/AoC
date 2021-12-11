package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"strconv"
)

func processInput(input *bufio.Scanner) [10][10]int {
	var octopuses [10][10]int

	for y := 0; input.Scan(); y++ {
		line := input.Text()

		for x, c := range line {
			n, _ := strconv.Atoi(string(c))

			octopuses[y][x] = n
		}
	}

	return octopuses
}

func simulate(octopuses [10][10]int) ([10][10]int, int) {
	// Start with increasing all the energy levels by one
	counter := 0
	for y := 0; y < 10; y++ {
		for x := 0; x < 10; x++ {
			octopuses[y][x]++
			if octopuses[y][x] > 9 {
				counter++
			}
		}
	}

	// Increase the energy levels of neighbours if a flash has to occur
	// We keep doing this until the number of octopuses ready to flash does not increase
	flashes := 0 
	for counter != 0 {
		for y := 0; y < 10; y++ {
			for x := 0; x < 10; x++ {
				if octopuses[y][x] > 9 {
					octopuses[y][x] = 0
					flashes++
					if x != 0 && octopuses[y][x-1] != 0 {
						octopuses[y][x-1]++
					}
					if x != 9 && octopuses[y][x+1] != 0 {
						octopuses[y][x+1]++
					}
					if y != 0 && octopuses[y-1][x] != 0 {
						octopuses[y-1][x]++
					}
					if y != 9 && octopuses[y+1][x] != 0 {
						octopuses[y+1][x]++
					}
					if x != 0 && y != 0 && octopuses[y-1][x-1] != 0 {
						octopuses[y-1][x-1]++
					}
					if x != 9 && y != 0 && octopuses[y-1][x+1] != 0 {
						octopuses[y-1][x+1]++
					}
					if x != 0 && y != 9 && octopuses[y+1][x-1] != 0 {
						octopuses[y+1][x-1]++
					}
					if x != 9 && y != 9 && octopuses[y+1][x+1] != 0 {
						octopuses[y+1][x+1]++
					}
				}
			}
		}

		counter = 0
		for y := 0; y < 10; y++ {
			for x := 0; x < 10; x++ {
				if octopuses[y][x] > 9 {
					counter++
				}
			}
		}
	}

	return octopuses, flashes
}

func main() {
	challenge := aoc.New(2021, 11)

	challenge.Test(`5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526`, []int{1656, 195})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		octopuses := processInput(input)

		sum := 0
		for step := 0; step < 100; step++ {
			var s int
			octopuses, s = simulate(octopuses)
			sum += s
		}

		return sum
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		octopuses := processInput(input)

		for step := 0; step < 1000; step++ {
			var s int
			octopuses, s = simulate(octopuses)
			// Return the first step where all octopuses flash at the same time
			if s == 100 {
				// +1 is needed as we start counting at 0
				return step + 1
			}
		}

		return -2
	})
}
