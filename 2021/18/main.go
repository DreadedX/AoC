package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
	"math"
	"strconv"
)

type snailNumber struct {
	child [2]*snailNumber
	value [2]int

	parent *snailNumber
}

func (s *snailNumber) print() {
	depth := s.depth()
	for i := 0; i < depth-1; i++ {
		fmt.Print("    ")
	}
	fmt.Printf("%d[\n", depth)

	if s.child[0] == nil {
		for i := 0; i < depth; i++ {
			fmt.Print("    ")
		}
		fmt.Printf("%d", s.value[0])
	} else {
		s.child[0].print()
	}
	fmt.Print("\n")
	if s.child[1] == nil {
		for i := 0; i < depth; i++ {
			fmt.Print("    ")
		}
		fmt.Printf("%d", s.value[1])
	} else {
		s.child[1].print()
	}

	fmt.Print("\n")
	for i := 0; i < depth-1; i++ {
		fmt.Print("    ")
	}
	fmt.Print("]")

	// We have reached the top level
	if s.parent == nil {
		fmt.Print("\n")
	}
}

func (s *snailNumber) depth() int {
	depth := 1
	{
		number := s
		for number.parent != nil {
			number = number.parent
			depth++
		}
	}

	return depth
}

func (s *snailNumber) explode() bool {
	depth := s.depth()

	// Check if the number needs to explode
	if depth > 4 {
		// Set this entry in the paren to 0
		side := -1
		for i := 0; i < 2; i++ {
			if s.parent.child[i] == s {
				s.parent.child[i] = nil
				s.parent.value[i] = 0
				side = i
			}
		}

		// We know that there is atleat one parent since we are >4 deep
		if s.parent.child[1-side] == nil {
			s.parent.value[1-side] += s.value[1-side]
		} else {
			num := s.parent.child[1-side]
			for true {
				if num.child[side] == nil {
					num.value[side] += s.value[1-side]
					break
				}
				num = num.child[side]
			}
		}

		num := s.parent
		for true {
			if num.parent == nil {
				break
			}

			ps := -1
			for i := 0; i < 2; i++ {
				if num.parent.child[i] == num {
					ps = i
				}
			}

			if ps == 1-side {
				num = num.parent

				if num.child[side] == nil {
					num.value[side] += s.value[side]
				} else {
					num := num.child[side]
					for true {
						if num.child[1-side] == nil {
							num.value[1-side] += s.value[side]
							break
						}
						num = num.child[1-side]
					}
				}
				break
			}


			num = num.parent
		}

		return true
	}

	for i := 0; i < 2; i++ {
		if s.child[i] != nil {
			if s.child[i].explode() {
				return true
			}
		}
	}

	return false
}

func (s *snailNumber) split() bool {
	for i := 0; i < 2; i++ {
		if s.child[i] == nil {
			if s.value[i] > 9 {
				lhs := math.Floor(float64(s.value[i]) / 2)
				rhs := math.Ceil(float64(s.value[i]) / 2)
				s.value[i] = 0
				s.child[i] = &snailNumber{value: [2]int{int(lhs), int(rhs)}, parent: s}

				return true
			}
		} else {
			if s.child[i].split() {
				return true
			}
		}
	}
	return false
}

func (s *snailNumber) magnitude() int {
	var partial [2]int
	for i := 0; i < 2; i++ {
		if s.child[i] == nil {
			partial[i] = s.value[i]
		} else {
			partial[i] = s.child[i].magnitude()
		}
	}

	return 3 * partial[0] + 2 * partial[1]
}

func (s *snailNumber) reduce() {
	for true {
		if s.explode() {
			// fmt.Println("After explode: "); top.print()
			continue
		}

		if s.split() {
			// fmt.Println("After split:   "); top.print()
			continue
		}

		break
	}
}

func add(a *snailNumber, b *snailNumber) *snailNumber {
	top := &snailNumber{}

	top.child[0] = a
	a.parent = top

	top.child[1] = b
	b.parent = top

	top.reduce()

	return top
}

func parseSnailNumber(line string) *snailNumber {
	var top snailNumber
	number := &top
	side := 0
	for _, c := range line {
		switch c {
		case '[':
			number.child[side] = &snailNumber{parent: number}
			number = number.child[side]
			side = 0
		case ']':
			number = number.parent
		case ',':
			side = 1
		default:
			value, _ := strconv.Atoi(string(c))
			number.value[side] = value
		}
	}

	top.child[0].parent = nil

	top.child[0].reduce()

	return top.child[0]
}

func main() {
	challenge := aoc.New(2021, 18)

	challenge.Test(`[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]`, []int{4140, 3993})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		var top *snailNumber
		for input.Scan() {
			line := input.Text()
			num := parseSnailNumber(line)

			if top == nil {
				top = num
			} else {
				top = add(top, num)
			}
		}

		return top.magnitude()
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		var lines []string
		for input.Scan() {
			lines = append(lines, input.Text())

		}

		largest := 0
		for _, li := range lines {
			for _, lj := range lines {
				if li != lj {
					numi := parseSnailNumber(li)
					numj := parseSnailNumber(lj)

					magnitude := add(numi, numj).magnitude()
					if magnitude > largest {
						largest = magnitude
					}
				}
			}
		}

		return largest
	})
}
