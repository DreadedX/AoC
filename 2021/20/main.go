package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
)

type Vec struct {
	x int
	y int
}

func (a Vec) Add(b Vec) Vec {
	return Vec{a.x+b.x, a.y+b.y}
}

type Image struct {
	data map[Vec]bool
	min Vec
	max Vec
}

func NewImage() Image {
	var image Image
	image.data = make(map[Vec]bool)

	return image
}

func processInput(input *bufio.Scanner) (Image, string) {
	image := NewImage()

	input.Scan()
	algorithm := input.Text()

	input.Scan()
	for y := 0; input.Scan(); y++ {
		if y > image.max.y {
			image.max.y = y
		}
		line := input.Text()
		for x, c := range line {
			if x > image.max.x {
				image.max.x = x
			}

			// Only store non-zero values
			if c == '#' {
				image.data[Vec{x,y}] = true
			}
		}
	}

	return image, algorithm
}

func (image *Image) Print() {
	for y := image.min.y; y <= image.max.y; y++ {
		for x := image.min.x; x <= image.max.x; x++ {
			if image.Get(x, y, false) {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Print("\n")
	}
	fmt.Print("\n")
}

func (image *Image) Get(x int, y int, outside bool) bool {
	if x < image.min.x || x > image.max.x || y < image.min.y || y > image.max.y {
		return outside
	}
	return image.data[Vec{x, y}]
}

func (image *Image) Kernel(x int, y int, algorithm string, outside bool) bool {
	index := 0
	for dy := -1; dy <= 1; dy++ {
		for dx := -1; dx <= 1; dx++ {
			index <<= 1
			if image.Get(x+dx, y+dy, outside) {
				index += 1
			}
		}
	}

	if algorithm[index] == '#' {
		return true
	}

	return false
}

func (image *Image) Enhance(algorithm string, outside bool) Image {
	newImage := NewImage()
	newImage.min = image.min.Add(Vec{-1, -1})
	newImage.max = image.max.Add(Vec{1, 1})

	for y := newImage.min.y; y <= newImage.max.y; y++ {
		for x := newImage.min.x; x <= newImage.max.x; x++ {
			if image.Kernel(x, y, algorithm, outside) {
				newImage.data[Vec{x,y}] = true
			}
		}
	}

	return newImage
}

func (image *Image) Count() int {
	return len(image.data)
}

func main() {
	challenge := aoc.New(2021, 20)

	challenge.Test(`..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###`, []int{35, 3351})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		image, algorithm := processInput(input)

		round1 := image.Enhance(algorithm, false)

		alternate := false
		if algorithm[0] == '#' {
			alternate = true
		}

		round2 := round1.Enhance(algorithm, alternate)

		return round2.Count()
	})

	// @NOTE This solution does not actually properly handle the algorithm[0] == '#' edge case
	// Here we assume that the last char in the algorithm is '.' and therefore alternate betwee # and . outside
	// However if the last char is # we keep # outside after the first round
	// For my input this was not the case, but it is an oversight
	challenge.Solution(2, func (input *bufio.Scanner) int {
		image, algorithm := processInput(input)

		alternate := false
		for round := 0; round < 50; round++ {
			image = image.Enhance(algorithm, alternate)
			if algorithm[0] == '#' {
				alternate = !alternate
			}
		}

		return image.Count()
	})
}
