package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
	"sort"
	"strconv"
)

type Coord struct {
	x int;
	y int;
}

func printGrid(grid [][]int) {
	for y := range grid {
		for _, c := range grid[y] {
			if c == 10000000 {
				fmt.Print("--- ")
			} else {
				fmt.Printf("%3d ", c)
			}
		}
		fmt.Print("\n")
	}
} 

func printSolution(weights [][]int) {
	current := Coord{len(weights[0])-1, len(weights)-1}

	route := make([][]string, len(weights))
	for i := range weights {
		row := make([]string, len(weights[i]))
		for j := range row {
			row[j] = "."
		}
		route[i] = row
	}

	route[current.y][current.x] = "x"

	value := 10000000
	for !(current.x == 0 && current.y == 0) {
		neighbours := [4]Coord{{current.x+1, current.y}, {current.x-1, current.y}, {current.x, current.y+1}, {current.x, current.y-1}}
		for _, neighbour := range neighbours {
			if neighbour.y < 0 || neighbour.y >= len(weights) || neighbour.x < 0 || neighbour.x >= len(weights[neighbour.y]) {
				continue
			}

			if weights[neighbour.y][neighbour.x] < value {
				value = weights[neighbour.y][neighbour.x]
				current = neighbour
			}
		}

		route[current.y][current.x] = "#"
	}

	route[0][0] = "x"

	for y := range route {
		for _, c := range route[y] {
			fmt.Printf("%s", c)
		}
		fmt.Printf("\n")
	}
}

func updateWeights(grid [][]int, weights [][]int, visited [][]bool, current Coord) ([][]int, []Coord) {
	neighbours := [4]Coord{{current.x+1, current.y}, {current.x-1, current.y}, {current.x, current.y+1}, {current.x, current.y-1}}
	var validNeighbours []Coord
	for _, neighbour := range neighbours {
		if neighbour.y < 0 || neighbour.y >= len(grid) || neighbour.x < 0 || neighbour.x >= len(grid[neighbour.y]) {
			continue
		}

		if visited[neighbour.y][neighbour.x] {
			continue
		}

		validNeighbours = append(validNeighbours, neighbour)

		cost := weights[current.y][current.x] + grid[neighbour.y][neighbour.x]

		if cost < weights[neighbour.y][neighbour.x] {
			weights[neighbour.y][neighbour.x] = cost
		}
	}

	return weights, validNeighbours
}

func minCost(grid [][]int, src Coord, dest Coord) int {
	// Create a grid of weights
	weights := make([][]int, len(grid))
	for i := range grid {
		row := make([]int, len(grid[i]))
		for j := range row {
			row[j] = 10000000
		}
		weights[i] = row
	}

	visited := make([][]bool, len(grid))
	for i := range grid {
		row := make([]bool, len(grid[i]))
		for j := range row {
			row[j] = false
		}
		visited[i] = row
	}

	// Set the cost of the src to 0
	weights[src.y][src.x] = 0

	targets := []Coord{src}
	iters := 0
	for !(targets[0].x == dest.x && targets[0].y == dest.y) {
		var neighbours []Coord
		weights, neighbours = updateWeights(grid, weights, visited, targets[0])
		visited[targets[0].y][targets[0].x] = true

		targets = append(targets[1:], neighbours...)

		n := 0
		for _, t := range targets {
			if !visited[t.y][t.x] {
				targets[n] = t
				n++
			}
		}
		targets = targets[:n]

		sort.Slice(targets, func(i, j int) bool {
			return weights[targets[i].y][targets[i].x] < weights[targets[j].y][targets[j].x]
		})

		iters++
	}

	// printSolution(weights)
	// fmt.Printf("Took %d iterations\n", iters)

	return weights[dest.y][dest.x]
}

func main() {
	challenge := aoc.New(2021, 15)

	challenge.Test(`1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581`, []int{40, 315})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		var grid [][]int
		for y := 0; input.Scan(); y++ {
			line := input.Text()

			row := make([]int, len(line))

			for i, c := range line {
				n, _ := strconv.Atoi(string(c))
				row[i] = n
			}

			grid = append(grid, row)
		}

		// printGrid(grid)

		return minCost(grid, Coord{0, 0}, Coord{len(grid[0])-1, len(grid)-1})
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		var grid [][]int
		for y := 0; input.Scan(); y++ {
			line := input.Text()

			row := make([]int, len(line)*5)

			for i, c := range line {
				n, _ := strconv.Atoi(string(c))
				for j := 0; j < 5; j++ {
					v := n + j
					if v > 9 {
						v -= 9
					}
					row[i+j*len(line)] = v
				}
			}

			grid = append(grid, row)
		}

		height := len(grid)
		for j := 1; j < 5; j++ {
			for y := 0; y < height; y++ {
				row := make([]int, len(grid[y]))

				for x, n := range grid[y] {
					v := n + j
					if v > 9 {
						v -= 9
					}
					row[x] = v
				}

				grid = append(grid, row)
			}
		}

		// printGrid(grid)

		return minCost(grid, Coord{0, 0}, Coord{len(grid[0])-1, len(grid)-1})
	})
}
