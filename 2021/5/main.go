package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"regexp"
	"strconv"
)

type line struct {
	x1, y1, x2, y2 int
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func findIntersection(diagonal bool) func (*bufio.Scanner) int {
	return func (input *bufio.Scanner) int {
		var lines []line
		var xmax, ymax int

		re := regexp.MustCompile("[0-9]+")
		for input.Scan() {
			text := input.Text()
			numbers := re.FindAllString(text, -1)

			if len(numbers) != 4 {
				panic("Did not find 4 numbers")
			}

			var coords [4]int
			for i, n := range numbers {
				c, err := strconv.Atoi(n)
				if err != nil {
					panic(err)
				}
				coords[i] = c
			}

			xmax = max(xmax, max(coords[0], coords[2]))
			ymax = max(ymax, max(coords[1], coords[3]))

			// Only consider horizontal and vertical line if diagonal is false
			if diagonal || coords[0] == coords[2] || coords[1] == coords[3] {
				lines = append(lines, line{coords[0], coords[1], coords[2], coords[3]})
			}
		}

		diagram := make([][]int, ymax+1)
		for y := 0; y <= ymax; y++ {
			diagram[y] = make([]int, xmax+1)
		}

		for _, a := range lines {
			// Vertical line
			if a.x1 == a.x2 {
				for y := min(a.y1, a.y2); y <= max(a.y1, a.y2); y++ {
					diagram[y][a.x1]++
				}
				continue
			}

			// Horizontal line
			if a.y1 == a.y2 {
				for x := min(a.x1, a.x2); x <= max(a.x1, a.x2); x++ {
					diagram[a.y1][x]++
				}
				continue
			}

			// Otherwise we have a diagonal line
			length := max(a.x1, a.x2) - min(a.x1, a.x2) + 1
			for i := 0; i < length; i++ {
				x := a.x1 + i
				if (a.x1 > a.x2) {
					x = a.x1 - i
				}

				y := a.y1 + i
				if (a.y1 > a.y2) {
					y = a.y1 - i
				}

				diagram[y][x]++
			}
		}


		points := 0
		for y := 0; y <= ymax; y++ {
			for x := 0; x <= xmax; x++ {
				if diagram[y][x] > 1 {
					points++
				}
			}
		}

		return points
	}
}

func main() {
	aoc := aoc.New(2021, 5)

	aoc.Test(`0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`, []int{5, 12})

	aoc.Solution(1, findIntersection(false))
	aoc.Solution(2, findIntersection(true))
}
