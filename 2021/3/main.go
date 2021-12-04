package main

import (
	"AoC/2021/common"
	"bufio"
	"strconv"
)

func filter(entries []int, length int, findCommon bool) int {
	e := make([]int, len(entries))
	copy(e, entries)
	for j := length-1; j >= 0; j-- {
		var count int
		for _, num := range e {
			count += num >> j & 1
		}

		var common int
		if !findCommon {
			common = 1
		}
		if float64(count)/float64(len(e)) >= 0.5 {
			if findCommon {
				common = 1
			} else {
				common = 0
			}
		}

		n := 0
		for _, num := range e {
			if num >> j & 1 == common {
				e[n] = num
				n++
			}
		}
		e = e[:n]

		if len(e) == 1 {
			break
		}
	}

	return e[0]
}

func main() {
	aoc := aoc.New(2021, 3)

	aoc.Test(`
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
	`, []int{198, 230})

	aoc.Solution(1, func (input *bufio.Scanner) int {
		length := 0

		var entries []int

		for input.Scan() {
			line := input.Text()
			if len(line) > length {
				length = len(line)
			}

			num, err := strconv.ParseInt(line, 2, 64)
			if err != nil {
				panic(err)
			}

			entries = append(entries, int(num))
		}

		gamma := 0
		epsilon := 0
		for j := 0; j < length; j++ {
			var count int
			for _, num := range entries {
				count += num >> j & 1
			}

			if float64(count)/float64(len(entries)) >= 0.5 {
				gamma += 1 << j
			} else {
				epsilon += 1 << j
			}
		}

		return gamma*epsilon
	})

	aoc.Solution(2, func (input *bufio.Scanner) int {
		length := 0

		var entries []int

		for input.Scan() {
			line := input.Text()
			if len(line) > length {
				length = len(line)
			}

			num, err := strconv.ParseInt(line, 2, 64)
			if err != nil {
				panic(err)
			}

			entries = append(entries, int(num))
		}

		oxygen := filter(entries, length, true)
		co2 := filter(entries, length, false)

		return int(oxygen * co2)
	})
}
