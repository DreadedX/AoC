package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
	"sort"
	"strings"
)

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
		subs := make(map[string]string, 0)

		input.Scan()
		polymer := input.Text()
		fmt.Printf("Template:\t%s\n", polymer)

		input.Scan()
		for input.Scan() {
			line := input.Text()
			s := strings.Split(line, " ")

			subs[s[0]] = s[2]
		}

		for step := 1; step <= 10; step++ {
			// First find all the matches
			indices := make(map[int]string, 0)
			for k, v := range subs {
				index := 0
				for true {
					i := strings.Index(polymer[index:], k)
					if i == -1 {
						break
					}

					index += i + 1
					indices[index] = v
				}
			}

			keys := make([]int, len(indices))
			i := 0
			for k := range indices {
				keys[i] = k
				i++
			}
			sort.Ints(keys)

			// Insert the new chars
			inserted := 0
			for _, k := range keys {
				index := k + inserted
				inserted++

				polymer = polymer[:index] + indices[k] + polymer[index:]
			}

			// fmt.Printf("Step %d:\t\t%s\n", step, polymer)
		}

		min := -1
		max := -1
		for _, v := range subs {
			c := strings.Count(polymer, v)

			if c < min || min == -1 {
				min = c
			}

			if c > max || max == -1 {
				max = c
			}
		}

		return max - min
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		subs := make(map[string]string, 0)

		input.Scan()
		polymer := input.Text()
		fmt.Printf("Template:\t%s\n", polymer)

		// Create the mapping
		input.Scan()
		for input.Scan() {
			line := input.Text()
			s := strings.Split(line, " ")

			subs[s[0]] = s[2]
		}

		// Find all different pairs in the initial polymer
		pairs := make(map[string]int, 0)
		for k := range subs {
			pairs[k] = 0

			index := 0
			for true {
				i := strings.Index(polymer[index:], k)
				if i == -1 {
					break
				}

				index += i + 1
				pairs[k]++
			}
		}

		counter := make(map[string]int, 0)
		for i := range polymer {
			counter[string(polymer[i])]++
		}

		// Execute each step
		for step := 1; step <= 40; step++ {
			pairsNew := make(map[string]int, 0)
			for p, c := range pairs {
				// Each pair will form a new 3 long segment
				n := string(p[0]) + subs[p] + string(p[1])
				counter[subs[p]] += c

				// Find the two pairs in this segment
				// These are added to the new list of pairs
				for k := range subs {
					index := 0
					for true {
						i := strings.Index(n[index:], k)
						if i == -1 {
							break
						}

						index += i + 1
						pairsNew[k] += c
					}
				}
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
	})
}
