package main

import (
	"AoC/2021/common"
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

type Card struct {
	numbers []int
	row [5]int
	column [5]int
}

// Remove empty entries
func filter(in []string) []string {
	n := 0
	for _, s := range in {
		if len(s) > 0 {
			in[n] = s
			n++
		}
	}
	return in[:n]
}

func findWinner(numbers []int, cards []Card) (int, []int) {
	for _, n := range numbers {
		for j := range cards {
			for k := range cards[j].numbers {
				if cards[j].numbers[k] == n {

					// Relpace the number to indicate is has been markeed
					cards[j].numbers[k] = -1

					row := k / 5
					column := k % 5

					cards[j].row[row]++
					cards[j].column[column]++

					if cards[j].row[row] == 5 || cards[j].column[column] == 5 {
						return n, cards[j].numbers
					}

					break
				}
			}
		}
	}

	fmt.Println("Failed to find a winner")

	return -1, nil
}

func findLosing(numbers []int, cards []Card) (int, []int) {
	for _, n := range numbers {
		i := 0
		for j := range cards {
			hasWon := false
			for k := range cards[j].numbers {
				if cards[j].numbers[k] == n {

					// Relpace the number to indicate is has been markeed
					cards[j].numbers[k] = -1

					row := k / 5
					column := k % 5

					cards[j].row[row]++
					cards[j].column[column]++

					if cards[j].row[row] == 5 || cards[j].column[column] == 5 {
						hasWon = true

						// If this is the last cardm then it is the one we are looking for
						if len(cards) == 1 {
							return n, cards[j].numbers
						}
					}
				}
			}
			
			// Keep the card if it has not won
			if !hasWon {
				cards[i] = cards[j]
				i++
			}
		}

		// Truncate the array of cards
		cards = cards[:i]
	}

	fmt.Println("Failed to find a winner")

	return -1, nil
}

func solution(selector func (number []int, cards []Card) (winning int, numbers []int)) (func (*bufio.Scanner) int) {
	return func (input *bufio.Scanner) int {
		// Read in the drawn numbers
		input.Scan()
		line := input.Text()
		var numbers []int
		for _, num := range strings.Split(line, ",") {
			n, err := strconv.Atoi(num)
			if err != nil {
				panic(err)
			}

			numbers = append(numbers, n)
		}

		// Read in all the cards
		var cards []Card
		var rowNumber int
		cardIndex := -1
		for input.Scan() {
			line = input.Text()

			// A new line means that we start a new card
			if len(line) == 0 {
				card := Card{}
				cards = append(cards, card)
				rowNumber = 0
				cardIndex++
			} else {
				row := filter(strings.Split(line, " "))

				for _, num := range row {
					n, err :=  strconv.Atoi(num)
					if err != nil {
						panic(err)
					}
					cards[cardIndex].numbers = append(cards[cardIndex].numbers, n)
				}

				rowNumber++
			}
		}

		// Play bingo and find the desired card and winnnig number
		winning, unmarked := selector(numbers, cards)

		// Calculate the sum of unmarked numbers
		sum := 0
		for _, n  := range unmarked {
			if n != -1 {
				sum += n
			}
		}

		return winning*sum
	}
}

func main() {
	aoc := aoc.New(2021, 4)

	aoc.Test(`
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
	`, []int{4512, 1924})

	aoc.Solution(1, solution(findWinner))
	aoc.Solution(2, solution(findLosing))
}
