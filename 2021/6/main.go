package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"strconv"
	"strings"
)

func main() {
	aoc := aoc.New(2021, 6)

	aoc.Test(`3,4,3,1,2`, []int{5934, 26984457539})

	// This is a very naive solution, it works for part 1 but not for part 2
	aoc.Solution(1, func (input *bufio.Scanner) int {
		var fish []int

		input.Scan()
		line := input.Text()
		numbers := strings.Split(line, ",")

		for _, str := range numbers {
			n, err := strconv.Atoi(str)
			if err != nil {
				panic(err)
			}

			fish = append(fish, n)
		}

		days := 80
		for day := 1; day <= days; day++ {
			add := 0

			for i := range fish {
				if fish[i] == 0 {
					add++
					fish[i] = 6
				} else {
					fish[i]--
				}
			}

			for i := 0; i < add; i++ {
				fish = append(fish, 8)
			}
		}

		return len(fish)
	})

	// This is a much more elegant solution, it can also be used to solve part 1
	aoc.Solution(2, func (input *bufio.Scanner) int {
		fish := make(map[int]int)
		input.Scan()
		line := input.Text()
		numbers := strings.Split(line, ",")

		for _, str := range numbers {
			n, err := strconv.Atoi(str)
			if err != nil {
				panic(err)
			}

			fish[n]++
		}

		days := 256
		for day := 1; day <= days; day++ {
			temp := make(map[int]int)

			for n, c := range fish {
				if n == 0 {
					temp[8] += c
					temp[6] += c
				} else {
					temp[n-1] += c
				}
			}

			fish = temp
		}

		sum := 0
		for _, c := range fish {
			sum += c
		}

		return sum
	})
}
