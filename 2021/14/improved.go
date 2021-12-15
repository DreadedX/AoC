package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"strings"
)

func synthesis(input *bufio.Scanner, steps int) int {
	// Read template
	input.Scan()
	polymer := input.Text()

	// Skip empty line
	input.Scan()

	// Create the mapping
	subs := make(map[string]string, 0)
	for input.Scan() {
		line := input.Text()
		s := strings.Split(line, " ")

		subs[s[0]] = s[2]
	}

	// Count the amount of each letter and pair in the template
	pairs := make(map[string]int, 0)
	counter := make(map[string]int, 0)
	for i := 0; i < len(polymer); i++ {
		if i < len(polymer)-1 {
			pairs[string(polymer[i]) + string(polymer[i+1])]++
		}
		counter[string(polymer[i])]++
	}

	// Execute each step
	for step := 1; step <= steps; step++ {
		pairsNew := make(map[string]int, 0)
		for p, c := range pairs {
			// Each pair will form two new pairs after insertion
			pairsNew[string(p[0]) + subs[p]] += c
			pairsNew[subs[p] + string(p[1])] += c
			counter[subs[p]] += c
		}

		pairs = pairsNew
	}


	min := -1
	max := -1
	for _, c := range counter {
		if c < min || min == -1 {
			min = c
		}

		if c > max || max == -1 {
			max = c
		}
	}

	return max - min
}

func main() {
	challenge := aoc.New(2021, 14)

	challenge.Test(`NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C`, []int{1588, 2188189693529})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		return synthesis(input, 10)
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		return synthesis(input, 40)
	})
}
