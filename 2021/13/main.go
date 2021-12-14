package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

type coordinate struct {
	x int;
	y int;
}

func printGrid(grid [][]bool) {
	// Print grind
	for _, line := range grid {
		for _, pos := range line {
			if pos {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Print("\n")
	}
}

func fold(input *bufio.Scanner, once bool) int {
	height := 0
	width := 0

	var coords []coordinate

	// Construct initial grid
	for input.Scan() {
		line := input.Text()
		if len(line) == 0 {
			break
		}

		c := strings.Split(line, ",")

		x, _ := strconv.Atoi(c[0])
		y, _ := strconv.Atoi(c[1])

		if x+1 > width {
			width = x+1
		}

		if y+1 > height {
			height = y+1
		}

		coords = append(coords, coordinate{x,y})
	}

	grid := make([][]bool, height)
	for i := range grid {
		grid[i] = make([]bool, width)
	}

	for _, c := range coords {
		grid[c.y][c.x] = true
	}

	// printGrid(grid)

	// Fold grid
	for input.Scan() {
		line := input.Text()

		fmt.Println("Fold", line)
		instruction := strings.Split(strings.Split(line, " ")[2], "=")

		axis, _ := strconv.Atoi(instruction[1])

		switch instruction[0] {
		case "x":
			fmt.Printf("vertical fold along %d\n", axis)
			for y := range grid {
				for x := 1; x <= axis; x++ {
					grid[y][axis-x] = grid[y][axis-x] || grid[y][axis+x]
					grid[y][axis+x] = false
				}

				grid[y] = grid[y][:axis]
			}

		case "y":
			fmt.Printf("horizontal fold along %d\n", axis)
			for y := 1; y <= axis; y++ {
				for x := range grid[y] {
					grid[axis-y][x] = grid[axis-y][x] || grid[axis+y][x]
					grid[axis+y][x] = false
				}
			}

			grid = grid[:axis]
		}

		// We only do the first instruction
		if once {
			break
		}
	}

	// For part two the grid forms 8 letters
	if !once {
		printGrid(grid)
	}

	// Count dots
	sum := 0
	for y := range grid {
		for x := range grid[y] {
			if grid[y][x] {
				sum++
			}
		}
	}

	return sum
}

func main() {
	challenge := aoc.New(2021, 13)

	challenge.Test(`6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5`, []int{17, 16})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		return fold(input, true)
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		return fold(input, false)
	})
}
