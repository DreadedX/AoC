package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"strconv"
	"strings"
)

func findMinFuel(cf func (distance int) int) func (*bufio.Scanner) int {
	return func (input *bufio.Scanner) int {
		max := 0
		input.Scan()
		crabs := make(map[int] int)
		for _, s := range strings.Split(input.Text(), ",") {
			n, err := strconv.Atoi(s)
			if err != nil {
				panic(err)
			}

			if n > max {
				max = n
			}

			crabs[n]++
		}

		// Loop over all possible options
		costmin := -1
		for i := 0; i <= max; i++ {
			cost := 0
			for p, n := range crabs {
				distance := p - i
				if distance < 0 {
					distance = -distance
				}

				cost += cf(distance) * n
			}

			if cost < costmin || costmin == -1 {
				costmin = cost
			}
		}

		return costmin
	}
}

func main() {
	aoc := aoc.New(2021, 7)
	
	aoc.Test(`16,1,2,0,4,2,7,1,2,14`, []int{37, 168})

	aoc.Solution(1, findMinFuel(func (distance int) int {
		return distance
	}))

	aoc.Solution(2, findMinFuel(func (distance int) int {
		return distance * (distance + 1) / 2
	}))
}
