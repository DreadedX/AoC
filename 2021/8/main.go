package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"strings"
)

func inSlice(a string, slice []string) bool {
	for _, b := range slice {
		if a == b {
			return true
		}
	}

	return false
}

// Returns elements that are present in a or b
func union(a []string, b []string) []string {
	for _, c := range b {
		if !inSlice(c, a) {
			a = append(a, c)
		}
	}

	return a
}

// Return all elements that are present in a, but not in b
func complement(a []string, b []string) []string {
	var temp []string
	for _, c := range a {
		if !inSlice(c, b) {
			temp = append(temp, c)
		}
	} 

	return temp
}

func match(a []string, b []string) bool {
	if len(a) != len(b) {
		return false
	}

	for _, s := range a {
		if !inSlice(s, b) {
			return false
		}
	}

	return true
}

func main() {
	aoc := aoc.New(2021, 8)

	aoc.Test(`acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf`, []int{0, 5353})

	aoc.Test(`be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
	edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
	fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
	fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
	aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
	fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
	dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
	bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
	egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
	gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
	`, []int{26, 61229})

	aoc.Solution(1, func(input *bufio.Scanner) int {
		count := 0
		for input.Scan() {
			line := input.Text()
			lineOutput := strings.TrimSpace(strings.Split(line, "|")[1])
			outputs := strings.Split(lineOutput, " ")

			for _, output := range outputs {
				// Detect unique digits 1, 4 ,7 and 8
				if len(output) == 2 || len(output) == 3 || len(output) == 4 || len(output) == 7 {
					count++
				}
			}
		}

		return count
	})

	aoc.Solution(2, func(input *bufio.Scanner) int {
		sum := 0
		for input.Scan() {
			line := input.Text()
			parts := strings.Split(line, "|")

			lineDigits := strings.TrimSpace(parts[0])
			digits := strings.Split(lineDigits, " ")

			mapping := make(map[int] []string)
			count := make(map[string] int)

			for _, digit := range digits {
				// Determine part of the mappnig using unique characters
				if len(digit) == 2 {
					mapping[1] = strings.Split(digit, "")
				} else if len(digit) == 3 {
					mapping[7] = strings.Split(digit, "")
				} else if len(digit) == 4 {
					mapping[4] = strings.Split(digit, "")
				} else if len(digit) == 7 {
					mapping[8] = strings.Split(digit, "")
				}

				// Count how often each segment appears
				for _, c := range digit {
					count[string(c)]++
				}
			}

			// Figure out what some of the elements map to
			{
				// b is the only segments that appears 6 times
				var b string
				for k, v := range count {
					if v == 6 {
						b = k
						break
					}
				}

				// e is the only segments that appears 4 times
				var e string
				for k, v := range count {
					if v == 4 {
						e = k
						break
					}
				}

				// f is the only segment that appears 9 times
				var f string
				for k, v := range count {
					if v == 9 {
						f = k
						break
					}
				}

				// Both a and c appear 8 times, however using the known characters we can find a
				a := complement(union(mapping[4], mapping[7]), mapping[4])[0]

				// Both a and c appear 8 times, however since we know a we can find c
				var c string
				for k, v := range count {
					if v == 8 && k != a {
						c = k
						break
					}
				}

				// Both d and g apear 7 times, however using known characrers and the fact that we know e we can still find g
				eg := complement(mapping[8], union(mapping[4], mapping[7]))
				var g string
				for _, c := range eg {
					if c != e {
						g = c
						break
					}
				}

				// Both d and g apear 7 times, however using known characrers and the fact that we know e we can still find d
				bd := complement(mapping[4], mapping[1])
				var d string
				for _, c := range bd {
					if c != b {
						d = c
						break
					}
				}

				// All segments are now known so we can fill in the rest of the map
				mapping[0] = []string{a, b, c, e, f, g}
				mapping[2] = []string{a, c, d, e, g}
				mapping[3] = []string{a, c, d, f, g}
				mapping[5] = []string{a, b, d, f, g}
				mapping[6] = []string{a, b, d, e, f, g}
				mapping[9] = []string{a, b, c, d, f, g}
			}

			lineOutput := strings.TrimSpace(parts[1])
			outputs := strings.Split(lineOutput, " ")

			// Using the completed mapping determine what the display is showing
			display := 0
			for _, output := range outputs {
				for k, v := range mapping {
					if !match(strings.Split(output, ""), v) {
						continue
					}

					display *= 10
					display += k
				}
			}

			sum += display
		}

		return sum
	})
}
