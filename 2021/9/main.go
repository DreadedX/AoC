package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"sort"
	"strconv"
	"strings"
)

func processInput(input *bufio.Scanner) ([][]int, int, int) {
	var heightmap [][]int
	for input.Scan() {
		heights := strings.Split(input.Text(), "")

		var row []int
		for _, height := range heights {
			// Ignore any errors
			h, _ := strconv.Atoi(height)
			row = append(row, h)
		}

		heightmap = append(heightmap, row)
	}

	ymax := len(heightmap) - 1
	// This assumes that at least one row exists and that all rows have the same length
	xmax := len(heightmap[0]) - 1

	return heightmap, xmax, ymax
}

func floodFill(heightmap [][]int, marked []bool, size int, x int, y int, xmax int, ymax int) ([]bool, int) {
	if heightmap[y][x] != 9 && !marked[y*(xmax+1) + x] {
		marked[y*(xmax+1) + x] = true
		size++

		if (x != 0) {
			marked, size = floodFill(heightmap, marked, size, x-1, y, xmax, ymax)
		}

		if (x != xmax) {
			marked, size = floodFill(heightmap, marked, size, x+1, y, xmax, ymax)
		}

		if (y != 0) {
			marked, size = floodFill(heightmap, marked, size, x, y-1, xmax, ymax)
		}

		if (y != ymax) {
			marked, size = floodFill(heightmap, marked, size, x, y+1, xmax, ymax)
		}
	}

	return marked, size
}

func main() {
	challenge := aoc.New(2021, 9)

	challenge.Test(`2199943210
3987894921
9856789892
8767896789
9899965678`, []int{15, 1134})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		heightmap, xmax, ymax := processInput(input)

		sum := 0
		for y := 0; y <= ymax; y++ {
			for x := 0; x <= xmax; x++ {
				height := heightmap[y][x]
				if (x != 0 && heightmap[y][x-1] <= height) || (x != xmax && heightmap[y][x+1] <= height) {
					// Neighbour on the x-axis is lower
					continue
				}

				if (y != 0 && heightmap[y-1][x] <= height) || (y != ymax && heightmap[y+1][x] <= height) {
					// Neighbour on the y-axis is lower
					continue
				}

				// Found a low point
				sum += 1 + height
			}
		}

		return sum
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		heightmap, xmax, ymax := processInput(input)

		// Find the size of all basins
		var sizes []int
		for y := 0; y <= ymax; y++ {
			for x := 0; x <= xmax; x++ {
				height := heightmap[y][x]
				if (x != 0 && heightmap[y][x-1] <= height) || (x != xmax && heightmap[y][x+1] <= height) {
					// Neighbour on the x-axis is lower
					continue
				}

				if (y != 0 && heightmap[y-1][x] <= height) || (y != ymax && heightmap[y+1][x] <= height) {
					// Neighbour on the y-axis is lower
					continue
				}

				// Found a low point, find the size of the basin using flood fill
				_, size := floodFill(heightmap, make([]bool, (ymax+1)*(xmax+1)), 0, x, y, xmax, ymax)
				sizes = append(sizes, size)
			}
		}

		// Sort the basin sizes
		sort.Ints(sizes)

		// Multiply the largest three
		answer := 1
		for _, size := range sizes[len(sizes)-3:] {
			answer *= size
		}

		return answer
	})
}
