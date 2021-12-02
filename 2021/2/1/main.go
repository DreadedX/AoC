package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	horizontal := 0
	depth := 0

	input, err := os.Open("input")
	if err != nil {
		panic(err)
	}
	defer input.Close()

	scanner := bufio.NewScanner(input)

	for scanner.Scan() {
		line := scanner.Text()
		re := regexp.MustCompile("[0-9]+")
		if !re.MatchString(line) {
			panic("Instruction does not contain number")
		}

		number, err := strconv.Atoi(re.FindString(line))

		if err != nil {
			panic(err)
		}


		if strings.HasPrefix(line, "forward") {
			horizontal += number
		} else if strings.HasPrefix(line, "up") {
			depth -= number
		} else if strings.HasPrefix(line, "down") {
			depth += number
		} else {
			panic("Unknown instruction")
		}
	}

	fmt.Println(horizontal, depth, horizontal*depth)

	if err := scanner.Err(); err != nil {
		panic(err)
	}
}

