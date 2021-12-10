package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"sort"
)

func getScoreCorrupted(line string, index int) (int, int) {
	c := line[index]
	if !(c == '(' || c == '{' || c == '[' || c == '<') {
		return 0, index
	}

	counter := 0
	for i := index; i < len(line); i++ {
		found := line[i]

		switch found {
		case '(', '{', '[', '<':
			counter++
		case ')', '}', ']', '>':
			counter--
		}

		if counter == 0 {
			if c == '(' && found == ')' {
				return 0, i
			} else if c == '{' && found == '}' {
				return 0, i
			} else if c == '[' && found == ']' {
				return 0, i
			} else if c == '<' && found == '>' {
				return  0, i
			} else {
				switch found {
				case ')':
					return 3, i
				case ']':
					return 57, i
				case '}':
					return 1197, i
				case '>':
					return 25137, i
				}
			}
		}

	}

	return 0, index
}

func getScoreIncomplete(line string, index int) (bool, int) {
	c := line[index]
	if !(c == '(' || c == '{' || c == '[' || c == '<') {
		return true, 0
	}

	counter := 0
	for i := index; i < len(line); i++ {
		found := line[i]

		switch found {
		case '(', '{', '[', '<':
			counter++
		case ')', '}', ']', '>':
			counter--
		}

		if counter == 0 {
			if c == '(' && found == ')' {
				break;
			} else if c == '{' && found == '}' {
				break
			} else if c == '[' && found == ']' {
				break
			} else if c == '<' && found == '>' {
				break
			} else {
				// Indicate that the line is corrupted
				return false, 0
			}
		}
	}

	if counter != 0 {
		switch c {
		case '(':
			return true, 1
		case '[':
			return true, 2
		case '{':
			return true, 3
		case '<':
			return true, 4
		}
	}

	return true, 0
}

func main() {
	challenge := aoc.New(2021, 10)

	challenge.Test(`[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`, []int{26397, 288957})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		score := 0
		for ln := 0; input.Scan(); ln++ {
			line := input.Text()

			fi := len(line)
			fs := 0

			for i := range line {
				s, index := getScoreCorrupted(line, i)
				if s != 0 && index < fi {
					fi = index
					fs = s
				}
			}

			score += fs
		}
		return score
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		var scores []int
		for ln := 0; input.Scan(); ln++ {
			line := input.Text()

			score := 0
			// We need to itterate in reverse because otherwise the multiplication goes in the wrong order
			for i := len(line)-1; i >= 0; i-- {
				v, s := getScoreIncomplete(line, i)
				if !v {
					score = 0
					break
				} else if s > 0 {
					score *= 5
					score += s
				}
			}

			if score > 0 {
				scores = append(scores, score)
			}
		}
		sort.Ints(scores)

		return scores[len(scores)/2]
	})
}
