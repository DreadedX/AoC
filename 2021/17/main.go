package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
	"regexp"
	"strconv"
)

type vec struct {
	x int
	y int
}

type probe struct {
	x vec
	u vec
}

type target struct {
	xmin int
	xmax int
	ymin int
	ymax int
}

func (a *vec) add(b vec) {
	a.x += b.x
	a.y += b.y
}

func (v *vec) in(t target) bool {
	return v.x >= t.xmin && v.x <= t.xmax && v.y >= t.ymin && v.y <= t.ymax
}

func (v *vec) past(t target) bool {
	return v.x > t.xmax || v.y < t.ymin
}

func main() {
	challenge := aoc.New(2021, 17)

	challenge.Test(`target area: x=20..30, y=-10..-5`, []int{45, 112})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		input.Scan()
		line := input.Text()

		r := regexp.MustCompile("-?[0-9]+")

		var coords [4]int
		for i, c := range r.FindAllString(line, -1) {
			coords[i], _ = strconv.Atoi(c)
		}
		t := target{coords[0], coords[1], coords[2], coords[3]}

		max := 0
		velocity := vec{0, 0}
		for vx := -1000; vx < 1000; vx++ {
			for vy := -1000; vy < 1000; vy++ {
				p := probe{u: vec{vx, vy}}
				height := 0
				for step := 0; true; step++ {
					if p.x.in(t) {
						if height > max {
							max = height
							velocity = vec{vx, vy}
						}

						break
					}

					if p.x.past(t) {
						break
					}

					if p.x.y > height {
						height = p.x.y
					}

					p.x.add(p.u)

					if p.u.x > 0 {
						p.u.add(vec{-1, 0})
					} else if p.u.x < 0 {
						p.u.add(vec{1, 0})
					}

					p.u.add(vec{0, -1})
				}

			}
		}

		fmt.Printf("Max height %d is achieved using %v\n", max, velocity)

		return max
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		input.Scan()
		line := input.Text()

		r := regexp.MustCompile("-?[0-9]+")

		var coords [4]int
		for i, c := range r.FindAllString(line, -1) {
			coords[i], _ = strconv.Atoi(c)
		}
		t := target{coords[0], coords[1], coords[2], coords[3]}

		counter := 0
		for vx := -1000; vx < 1000; vx++ {
			for vy := -1000; vy < 1000; vy++ {
				p := probe{u: vec{vx, vy}}
				for step := 0; true; step++ {
					if p.x.in(t) {
						counter++
						break
					}

					if p.x.past(t) {
						break
					}

					p.x.add(p.u)

					if p.u.x > 0 {
						p.u.add(vec{-1, 0})
					} else if p.u.x < 0 {
						p.u.add(vec{1, 0})
					}

					p.u.add(vec{0, -1})
				}

			}
		}

		return counter
	})
}
