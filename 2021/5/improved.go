package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"regexp"
	"strconv"
)

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
		size := 1000
		diagram := make([][]int, size)
		for y := 0; y < size; y++ {
			diagram[y] = make([]int, size)
		}

		re := regexp.MustCompile("[0-9]+")
		for input.Scan() {
			text := input.Text()
			numbers := re.FindAllString(text, -1)

			if len(numbers) != 4 {
				panic("Did not find 4 numbers")
			}

			x1, _ := strconv.Atoi(numbers[0])
			y1, _ := strconv.Atoi(numbers[1])
			x2, _ := strconv.Atoi(numbers[2])
			y2, _ := strconv.Atoi(numbers[3])

			// Optionally skip lines that are not horizontal or vertical
			if !diagonal && !(x1 == x2 || y1 == y2) {
				continue
			}

			dx := 1
			dy := 1
			length := max(max(x1, x2) - min(x1, x2) + 1, max(y1, y2) - min(y1, y2) + 1)

			if x1 == x2 {
				dx = 0
			} else if x1 > x2 {
				dx = -dx
			}

			if y1 == y2 {
				dy = 0
			} else if y1 > y2 {
				dy = -dy
			}

			for i := 0; i < length; i++ {
				diagram[y1 + i*dy][x1 + i*dx]++
			}
		}

		points := 0
		for y := 0; y < size; y++ {
			for x := 0; x < size; x++ {
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
