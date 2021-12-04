package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"log"
	"strconv"
)

func count(entries []string, width int) [2][]int {
	counts := [2][]int{make([]int, width), make([]int, width)}

	for _, entry := range entries {
		for i, c := range entry {
			if string(c) == "0" {
				counts[0][i]++
			} else if string(c) == "1" {
				counts[1][i]++
			} else {
				log.Fatalf("Unknown character '%s'\n", string(c))
			}
		}
	}

	return counts
}

func filter(entries []string, i int, value string) []string {
	n := 0
	for _, num := range entries {
		if string(num[i]) == value {
			entries[n] = num
			n++
		}
	}

	return entries[:n]
}

func process(input *bufio.Scanner) ([]string, int) {
	var numbers []string
	width := 0

	for i :=0; input.Scan(); i++ {
		line := input.Text()
		numbers = append(numbers, line)

		if len(line) > width {
			width  = len(line)
		}
	}

	return numbers, width
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
		entries, width := process(input)
		counts := count(entries, width)

		gamma := 0
		epsilon := 0
		for i := 0; i < width; i++ {
			if counts[0][i] > counts[1][i] {
				epsilon += 1 << (width - 1 - i)
			} else {
				gamma += 1 << (width - 1 - i)
			}
		}

		return gamma * epsilon
	})

	aoc.Solution(2, func (input *bufio.Scanner) int {
		oxygen, width := process(input)
		oxygenCounts := count(oxygen, width)

		co2 := make([]string, len(oxygen)); copy(co2, oxygen)
		co2Counts := count(co2, width)

		for i := 0; i < width; i++ {
			f1 := "1"
			if oxygenCounts[0][i] > oxygenCounts[1][i] {
				f1 = "0"
			}
			if len(oxygen) > 1 {
				oxygen = filter(oxygen, i, f1)
				oxygenCounts = count(oxygen, width)
			}

			f2 := "0"
			if co2Counts[0][i] > co2Counts[1][i] {
				f2 = "1"
			}
			if len(co2) > 1 {
				co2 = filter(co2, i, f2)
				co2Counts = count(co2, width)
			}
		}

		oxygenNum, err := strconv.ParseInt(oxygen[0], 2, 0)
		if err != nil {
			panic(err)
		}

		co2Num, err := strconv.ParseInt(co2[0], 2, 0)
		if err != nil {
			panic(err)
		}

		return int(oxygenNum) * int(co2Num)
	})
}
