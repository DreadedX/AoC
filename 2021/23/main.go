package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
	"regexp"
)

type Amphipod int

const (
	Empty  Amphipod = 0
	Amber           = 1
	Bronze          = 10
	Copper          = 100
	Desert          = 1000
)

func GetAmphipod(s string) Amphipod {
	switch s {
	case "A":
		return Amber
	case "B":
		return Bronze
	case "C":
		return Copper
	case "D":
		return Desert
	default:
		panic("Unknown type of Amphipod")
	}
}

func (a Amphipod) Letter() string {
	switch a {
	case Amber:
		return "A"
	case Bronze:
		return "B"
	case Copper:
		return "C"
	case Desert:
		return "D"
	default:
		return "."
	}
}

func (a Amphipod) Destination() (int, error) {
	switch a {
	case Amber:
		return 0, nil
	case Bronze:
		return 1, nil
	case Copper:
		return 2, nil
	case Desert:
		return 3, nil
	}

	return -1, fmt.Errorf("Amphipod has no destination")
}

type Room struct {
	position int
	occupants [4]Amphipod
}

func (r Room) ValidDesination(a Amphipod) (int, error) {
	depth := -1
	for i := range r.occupants {
		if r.occupants[i] == Empty {
			depth++
		} else if r.occupants[i] != a {
			return -1, fmt.Errorf("Invalid destination")
		}
	}

	return depth, nil

}

func (r Room) InCorrectRoom(index int, a Amphipod) bool {
	b := Empty
	switch index {
	case 0:
		b = Amber
	case 1:
		b = Bronze
	case 2:
		b = Copper
	case 3:
		b = Desert
	default:
		return false
	}

	return a == b
}

func (r Room) IsAmphipodDone(index int, depth int, a Amphipod) bool {
	if !r.InCorrectRoom(index, a) {
		return false
	}

	// Check if the Amphipods below are correct
	for i := depth; i < len(r.occupants); i++ {
		if r.occupants[i] != a {
			return false
		}
	}

	return true
}

func (r Room) Solved(index int) bool {
	for i := range r.occupants {
		if !r.InCorrectRoom(index, r.occupants[i]) {
			return false
		}
	}

	return true
}

func (r Room) Top() (Amphipod, int) {
	for i := range r.occupants {
		if r.occupants[i] != Empty {
			return r.occupants[i], i
		}
	}

	return r.occupants[0], 0
}

// Create a new room with the top entry removed
func (r Room) Pop() (Room, int) {
	for i := range r.occupants {
		if r.occupants[i] == Empty {
			continue
		}

		r.occupants[i] = Empty
		return r, i
	}

	return r, 0
}

type Puzzle struct {
	corridor [11]Amphipod

	rooms [4]Room
}

func (p Puzzle) CheckCorridor(start int, end int) (bool, int) {
	cost := 0
	for k := start; k <= end; k++ {
		cost++
		if p.corridor[k] != Empty {
			// If there is an non empty tile along the way we move on to the next amphipod
			// We do not have to check the other rooms as there can only ever be at most one valid room
			return false, 0
		}
	}

	return true, cost
}

func (p Puzzle) Print() {
	fmt.Println("#############")
	fmt.Print("#")
	for _, a := range p.corridor {
		fmt.Printf("%s", a.Letter())
	}
	fmt.Print("#\n")

	for i := 0; i < len(p.rooms[0].occupants); i++ {
		filler := "  "
		if i == 0 {
			filler = "##"
		}
		fmt.Printf("%s", filler)
		for _, r := range p.rooms {
			fmt.Printf("#%s", r.occupants[i].Letter())
		}
		fmt.Printf("#%s\n", filler)
	}
	fmt.Println("  #########  ")
}

func (p Puzzle) Solved() bool {
	for i := 0; i < len(p.rooms[0].occupants); i++ {
		if p.rooms[0].occupants[i] != Amber {
			return false
		}
		if p.rooms[1].occupants[i] != Bronze {
			return false
		}
		if p.rooms[2].occupants[i] != Copper {
			return false
		}
		if p.rooms[3].occupants[i] != Desert {
			return false
		}
	}

	return true
}

func (p Puzzle) Solve(iteration int) (bool, int) {
	// First check if we have already calculated the state
	// if cost, ok := cache[p]; ok {
	// 	return cost
	// }

	// fmt.Println("Depth:", iteration)
	// p.Print()

	cost := 0

	// If possible move Amphipod to destination room
	resolve:
	for true {
		// fmt.Println("Checking: ROOM => ROOM movement")
		for i, r := range p.rooms {
			// Check if the room is already solved
			if r.Solved(i) {
				continue
			}

			a, adepth := r.Top()

			if a == Empty {
				continue
			}

			if r.IsAmphipodDone(i, adepth, a) {
				continue
			}

			d, _ := a.Destination()
			if edepth, err := p.rooms[d].ValidDesination(a); err == nil {
				// The room is free and valid
				start := r.position
				end := p.rooms[d].position
				if start > end {
					temp := start
					start = end
					end = temp
				}
				free, potentialCost := p.CheckCorridor(start, end)
				if !free {
					continue
				}

				// fmt.Println(potentialCost, r.position, p.rooms[d].position)

				// fmt.Printf("\tMoving %v from %d to %d\n", a.Letter(), i, d)

				potentialCost += edepth
				var sdepth int
				p.rooms[i], sdepth = r.Pop()
				p.rooms[d].occupants[edepth] = a
				potentialCost += sdepth
				potentialCost += 1

				// fmt.Printf("\t\t%d %d\n", sdepth, edepth)

				potentialCost *= int(a)

				// fmt.Printf("\t\tCost: %d\n", potentialCost)

				cost += potentialCost

				continue resolve
			}
		}


		// fmt.Println("Checking: CORRIDOR => ROOM movement")
		// For every space in the corridor we check if there is a Amphipod that could move
		for i, a := range p.corridor {
			if a == Empty {
				continue
			}

			// Get the ID of the destination room
			j, _ := a.Destination()

			// Check if the destination room is free
			if depth, err := p.rooms[j].ValidDesination(a); err == nil {
				// Get the start and end position
				start := i+1
				end := p.rooms[j].position
				if start > end {
					temp := start-2
					start = end
					end = temp
				}

				free, potentialCost := p.CheckCorridor(start, end)
				if !free {
					continue
				}
				// We need to add one since we start with a move to the left or the right
				potentialCost += depth + 1
				potentialCost *= int(a) 

				// fmt.Printf("\tMoving %s from %d to %d\n", a.Letter(), i, j)
				// fmt.Printf("\t\tCost: %d\n", potentialCost)

				// Update state
				p.corridor[i] = Empty
				p.rooms[j].occupants[depth] = a

				cost += potentialCost

				// Restart looping over all spaces in the corridor as another amphipod might now be able to move
				continue resolve
			}
		}

		break
	}

	if p.Solved() {
		return true, cost
	}

	// Generate future states
	minCost := -1
	for i, r := range p.rooms {
		// Check if the room is already solved
		if r.Solved(i) {
			continue
		}

		// Get the top amphipod in the room
		a, adepth := r.Top()

		// The room is empty, move on to the next room
		if a == Empty {
			continue
		}

		if r.IsAmphipodDone(i, adepth, a) {
			continue
		}

		// Move the amphipod to the corridor
		outer:
		for j, c := range p.corridor {
			// The amphipod can not stop in from of the room
			// We also can not stop in another amphipod
			if c != Empty {
				continue
			}

			for _, rr := range p.rooms {
				if j == rr.position {
					continue outer
				}
			}

			// Get the start and end point
			start := r.position
			end := j
			if start > end {
				temp := start
				start = end
				end = temp
			}

			free, potentialCost := p.CheckCorridor(start, end)
			if !free {
				continue
			}

			// Create the new state
			np := p
			nr, depth := r.Pop()
			np.rooms[i] = nr
			np.corridor[j] = a

			solved, nextCost := np.Solve(iteration+1)

			if solved {
				nextCost += (potentialCost + depth) * int(a)

				if nextCost < minCost || minCost == -1 {
					minCost = nextCost
				}
			}
		}
	}

	if minCost == -1 {
		return false, 10000000
	}

	return true, cost + minCost
}

func NewPuzzle1(input *bufio.Scanner) Puzzle {
	input.Scan()
	input.Scan()

	r := regexp.MustCompile("[A-Z]")

	var p Puzzle
	for i := 0; i < 2; i++ {
		input.Scan()
		line := r.FindAllString(input.Text(), 4)

		for j, a := range line {
			p.rooms[j].occupants[i] = GetAmphipod(a)
		}
	}

	// Pad the bottom out so it is compatible with part 2
	p.rooms[0].occupants[2] = Amber
	p.rooms[0].occupants[3] = Amber
	p.rooms[1].occupants[2] = Bronze
	p.rooms[1].occupants[3] = Bronze
	p.rooms[2].occupants[2] = Copper
	p.rooms[2].occupants[3] = Copper
	p.rooms[3].occupants[2] = Desert
	p.rooms[3].occupants[3] = Desert

	for i := range p.rooms {
		p.rooms[i].position = 2 + 2*i
	}

	return p
}

func NewPuzzle2(input *bufio.Scanner) Puzzle {
	input.Scan()
	input.Scan()

	r := regexp.MustCompile("[A-Z]")

	var p Puzzle
	for i := 0; i < 2; i++ {
		input.Scan()
		line := r.FindAllString(input.Text(), 4)

		for j, a := range line {
			p.rooms[j].occupants[i*3] = GetAmphipod(a)
		}
	}

	// Fill in the middle lines
	p.rooms[0].occupants[1] = Desert
	p.rooms[0].occupants[2] = Desert
	p.rooms[1].occupants[1] = Copper
	p.rooms[1].occupants[2] = Bronze
	p.rooms[2].occupants[1] = Bronze
	p.rooms[2].occupants[2] = Amber
	p.rooms[3].occupants[1] = Amber
	p.rooms[3].occupants[2] = Copper

	for i := range p.rooms {
		p.rooms[i].position = 2 + 2*i
	}

	return p
}

// The code to solve this is a complete mess
// BUT it works, and that is all that matters for right now
// Should really write an improved version of this though...
func main() {
	challenge := aoc.New(2021, 23)

	challenge.Test(`#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
`, []int{12521, 44169})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		puzzle := NewPuzzle1(input)

		_, cost := puzzle.Solve(0)

		return cost
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		puzzle := NewPuzzle2(input)

		_, cost := puzzle.Solve(0)

		return cost
	})
}
