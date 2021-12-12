package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"strings"
)

type cave struct {
	large bool
	connected []string
}

func connect(system map[string] cave, a string, b string) map[string] cave {
	if _, exists := system[a]; !exists {
		c := cave{strings.ToUpper(a) == a, make([]string, 0)}
		system[a] = c
	}

	c := system[a]
	c.connected = append(c.connected, b)
	system[a] = c

	return system
}

func processInput(input *bufio.Scanner) map[string] cave {
	system := make(map[string] cave)

	for input.Scan() {
		line := input.Text()

		connection := strings.Split(line, "-")

		system = connect(system, connection[0], connection[1])
		system = connect(system, connection[1], connection[0])

	}

	return system
}

func router1(system map[string] cave, visited []string, current string) int {
	if current == "end" {
		return 1
	}

	sum := 0
	outer:
	for _, c := range system[current].connected {
		for _, v := range visited {
			if v == c {
				if !system[c].large {
					continue outer
				}
				break
			}
		}

		var s int
		s = router1(system, append(visited, c), c)
		sum += s
	}

	return sum
}

func router2(system map[string] cave, visited []string, current string, twiced bool) int {
	if current == "end" {
		return 1
	}

	sum := 0
	outer:
	for _, c := range system[current].connected {
		t := twiced
		for _, v := range visited {
			if v == c {
				if !system[c].large {
					if twiced || c == "start" {
						continue outer
					}
					t = true
				}
				break
			}
		}

		s := router2(system, append(visited, c), c, t)
		sum += s
	}

	return sum
}

func main() {
	challenge := aoc.New(2021, 12)

	challenge.Test(`start-A
start-b
A-c
A-b
b-d
A-end
b-end`, []int{10, 36})

	challenge.Test(`dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc`, []int{19, 103})

	challenge.Test(`fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW`, []int{226, 3509})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		return router1(processInput(input), []string{"start"}, "start")
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		return router2(processInput(input), []string{"start"}, "start", false)
	})
}
