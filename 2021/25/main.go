package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
)

type Cucumber int

const (
	Empty Cucumber = iota
	East
	South
)

func (c Cucumber) String() string {
	switch c {
	case Empty:
		return "."
	case East:
		return ">"
	case South:
		return "v"
	default:
		return fmt.Sprintf("%d", c)
	}
}

type Grid [][]Cucumber

func (g Grid) Print() {
	for _, row := range g {
		for _, c := range row {
			fmt.Print(c)
		}
		fmt.Print("\n")
	}
}

func (g Grid) stepEast() (Grid, int) {
	counter := 0
	ng := make(Grid, len(g))
	for i := range g {
		ng[i] = make([]Cucumber, len(g[i]))
		for j := range g[i] {
			ng[i][j] = g[i][j]
		}
	}

	for i := range g {
		if g[i][len(g[i])-1] == East && g[i][0] == Empty {
			ng[i][0] = East
			ng[i][len(g[i])-1] = Empty
			counter++
		}

		for j := len(g[i]) - 2; j >= 0; j-- {
			if g[i][j] == East && g[i][j+1] == Empty {
				ng[i][j+1] = East
				ng[i][j] = Empty
				counter++
			}
		}
	}

	return ng, counter
}

func (g Grid) stepSouth() (Grid, int) {
	counter := 0
	ng := make(Grid, len(g))
	for i := range g {
		ng[i] = make([]Cucumber, len(g[i]))
		for j := range g[i] {
			ng[i][j] = g[i][j]
		}
	}

	for j := range g[0] {
		if g[len(g)-1][j] == South && g[0][j] == Empty {
			ng[0][j] = South
			ng[len(g)-1][j] = Empty
			counter++
		}
	}

	for i := len(g)-2; i >= 0; i-- {
		for j := range g[i] {
			if g[i][j] == South && g[i+1][j] == Empty {
				ng[i+1][j] = South
				ng[i][j] = Empty
				counter++
			}
		}
	}

	return ng, counter
}

func (g Grid) Step() (Grid, int) {
	var a, b int
	g, a = g.stepEast()
	g, b = g.stepSouth()
	return g, a+b
}

func main() {
	challenge := aoc.New(2021, 25)

	challenge.Test(`v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>`, []int{58, -1})

	challenge.Solution(1, func(input *bufio.Scanner) int {
		var grid Grid

		for input.Scan() {
			line := input.Text()

			row := make([]Cucumber, len(line))
			for i, c := range line {
				switch c {
				case '>':
					row[i] = East
				case 'v':
					row[i] = South
				default:
					row[i] = Empty
				}
			}

			grid = append(grid, row)
		}

		// grid.Print()

		noMovementAfter := 1
		for ; true; noMovementAfter++ {
			var moved int
			grid, moved = grid.Step()

// 			fmt.Println("\nSTEP", noMovementAfter+1)
// 			grid.Print()

			if moved == 0 {
				break
			}
		}

		grid.Print()

		return noMovementAfter
	})

	challenge.Solution(2, func(input *bufio.Scanner) int {
		return 0
	})
}
