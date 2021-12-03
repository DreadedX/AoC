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
	result int
}

type adventOfCode struct {
	year int
	day int
	testCases []testCase
}

func New(year int, day int) adventOfCode {
	err := godotenv.Load()
	if err != nil {
		fmt.Println("Failed to open .env")
	}

	return adventOfCode{year, day, nil}
}

func (aoc adventOfCode) Solution(f func(*bufio.Scanner) int) {
	if len(aoc.testCases) == 0 {
		fmt.("No testCases provided!\n")
	}

	failed := false
	for i, testCase := range aoc.testCases {
		input := bufio.NewScanner(strings.NewReader(testCase.input))
		result := f(input)

		if result != testCase.result {
			fmt.Printf("Test %d failed, expected '%d', got '%d'\n", i+1, testCase.result, result)
			failed = true
		} else {
			fmt.Printf("Test %d passed!\n", i+1)
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
	fmt.Printf("Solution: %d\n", result)
}

func (aoc *adventOfCode) Test(input string, result int) {
	aoc.testCases = append(aoc.testCases, testCase{strings.TrimSpace(input), result})
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
