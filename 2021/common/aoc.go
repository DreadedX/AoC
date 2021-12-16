package aoc

import (
	"bufio"
	"errors"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"strings"

	"github.com/joho/godotenv"
)

type testCase struct {
	input string
	results []int
}

type adventOfCode struct {
	year int
	day int
	testCases []testCase
}

func New(year int, day int) adventOfCode {
	err := godotenv.Load("../../.env")
	if err != nil {
		fmt.Println("Failed to open .env")
	}

	return adventOfCode{year, day, nil}
}

func (aoc adventOfCode) Solution(part int, f func(*bufio.Scanner) int) {
	fmt.Printf("AoC %d - Day %d - Part %d\n", aoc.year, aoc.day, part)
	if len(aoc.testCases) == 0 {
		fmt.Println("No testCases provided!")
	}

	failed := false
	for i, testCase := range aoc.testCases {
		if testCase.results[part-1] == -2 {
			fmt.Printf("\tTest %d skipped!\n", i+1)
			continue
		}

		input := bufio.NewScanner(strings.NewReader(testCase.input))
		result := f(input)

		if result != testCase.results[part-1] {
			fmt.Printf("\tTest %d failed, expected '%d', got '%d'\n", i+1, testCase.results[part-1], result)
			failed = true
		} else {
			fmt.Printf("\tTest %d passed!\n", i+1)
		}
	}
	if failed {
		return
	}

	input, err := aoc.getInput()
	if err != nil {
		panic(err)
	}

	result := f(input)
	fmt.Printf("\tSolution:\n\t\t%d\n\n", result)
}

func (aoc *adventOfCode) Test(input string, results []int) {
	aoc.testCases = append(aoc.testCases, testCase{strings.TrimSpace(input), results})
}

func (aoc adventOfCode) getInput() (*bufio.Scanner, error) {
	client := &http.Client{}

	url := fmt.Sprintf("https://adventOfCode.com/%d/day/%d/input", aoc.year, aoc.day)
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return nil, err
	}

	session := os.Getenv("SESSION")
	cookie := fmt.Sprintf("session=%s", session)
	req.Header.Set("Cookie", cookie)
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}

	if resp.StatusCode != 200 {
		message, err := ioutil.ReadAll(resp.Body)
		if err != err {
			return nil, err
		}
		return nil, errors.New(string(message))
	}

	scanner := bufio.NewScanner(resp.Body)
	return scanner, nil
}
