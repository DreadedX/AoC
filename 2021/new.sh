#!/bin/bash
mkdir -p $year/$1
cd $year/$1
cat > go.mod << EOM
module AoC/$year/$1

require AoC/$year/common v0.0.0

require github.com/joho/godotenv v1.4.0 // indirect

replace AoC/$year/common v0.0.0 => ../common

go 1.17
EOM

go get AoC/$1/common@v0.0.0

cat > main.go << EOM
package main

import (
	aoc "AoC/2021/common"
	"bufio"
)

func main() {
	challenge := aoc.New($year, $1)

	challenge.Test(\`\`, []int{-1, -1})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		return 0
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		return 0
	})
}
EOM
