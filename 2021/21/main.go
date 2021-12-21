package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
	"regexp"
	"strconv"
)

type DeterministicDie struct {
	counter int
}

func (d DeterministicDie) Roll(times int) (DeterministicDie, int) {
	s := 0

	for i := 0; i < times; i++ {
		s += d.counter + 1
		d.counter++
		d.counter %= 100
	}

	return d, s
}

type Player struct {
	position int
	score int
	rolls int
}

func NewPlayer(position int) Player {
	var p Player
	p.position = position - 1
	return p
}

func (p Player) Move(spaces int) Player {
	p.position += spaces
	p.position %= 10

	p.score += p.Position()

	p.rolls += 3

	return p
}

func (p Player) Position() int {
	// We use a postion from 0 to 9 internally as it is easier to work with
	return p.position + 1
}

func (p Player) Score() int {
	return p.score
}

func (p Player) Rolls() int {
	return p.rolls
}

type GameState struct {
	pos [2]int
	score [2]int
	turn int
}

func main() {
	challenge := aoc.New(2021, 21)

	challenge.Test(`Player 1 starting position: 4
Player 2 starting position: 8`, []int{739785, 444356092776315})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		r := regexp.MustCompile("[0-9]+")

		var players [2]Player
		for i := range players {
			input.Scan()
			position, _ := strconv.Atoi(r.FindAllString(input.Text(), 2)[1])
			players[i] = NewPlayer(position)
		}

		var die DeterministicDie
		game:
		for i := 0; true; i++ {
			for j, p := range players {
				var rolled int
				die, rolled = die.Roll(3)
				players[j] = p.Move(rolled)

				if players[j].Score() >= 1000 {
					break game
				}
			}
		}

		score := 0
		rolls := 0
		for _, p := range players {
			rolls += p.Rolls()
			if p.Score() < 1000 {
				score = p.Score()
			}
		}

		fmt.Println(score, rolls)

		return score*rolls
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		r := regexp.MustCompile("[0-9]+")

		var players [2]int
		for i := range players {
			input.Scan()
			position, _ := strconv.Atoi(r.FindAllString(input.Text(), 2)[1])
			players[i] = position-1
		}

		// List of all current states
		states := make(map[GameState]int)
		states[GameState{players, [2]int{}, 0}] = 1

		// Pre compute all possible outcomes and their frequency
		outcomes := make(map[int]int)
		maxNum := 3
		for r := 1; r <= maxNum; r++ {
			for s := 1; s <= maxNum; s++ {
				for t := 1; t <= maxNum; t++ {
					outcomes[r + s + t]++
				}
			}
		}

		var victories [2]int
		// Keep going as long as there are unfinished games
		for i := 0; len(states) > 0; i++ {
			statesNext := make(map[GameState]int)

			// Update all states
			for state, occurs := range states {
				// Create all possible outcomes
				for outcome, num := range outcomes {
					stateNext := state

					stateNext.pos[state.turn] += outcome
					stateNext.pos[state.turn] %= 10
					stateNext.score[state.turn] += stateNext.pos[state.turn] + 1

					if stateNext.score[state.turn] >= 21 {
						// Player has won
						victories[state.turn] += occurs*num
					} else {
						stateNext.turn = 1 - state.turn
						statesNext[stateNext] += occurs*num
					}
				} 

			}
			states = statesNext
		}
		
		fmt.Println(victories)
		if victories[1] > victories[0] {
			return victories[1]
		}

		return victories[0]
	})
}
