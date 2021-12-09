#!/bin/bash
mkdir -p $1/$2
cd $1/$2
cat > go.mod << EOM
module AoC/$1/$2

require AoC/$1/common v0.0.0

require github.com/joho/godotenv v1.4.0 // indirect

replace AoC/$1/common v0.0.0 => ../common

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
	challenge := aoc.New($1, $2)

	challenge.Test(\`\`, []int{-1, -1})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		return 0
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		return 0
	})
}
EOM
