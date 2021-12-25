package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

type Operator byte
const (
	INP Operator = iota
	ADD
	ADDR
	MUL
	MULR
	DIV
	DIVR
	MOD
	MODR
	EQL
	EQLR
)

func (op Operator) String() string {
	switch op {
	case INP:
		return "INP"
	case ADD, ADDR:
		return "ADD"
	case MUL, MULR:
		return "MUL"
	case DIV, DIVR:
		return "DIV"
	case MOD, MODR:
		return "MOD"
	case EQL, EQLR:
		return "EQL"
	default:
		return fmt.Sprintf("%d", int(op))
	}
}

type Instruction struct {
	op Operator
	a int
	b int
}

func (i Instruction) String() string {
	if i.op == INP {
		return fmt.Sprintf("%v\t[%s]", i.op, registerToString(i.a))
	}

	if i.op % 2 == 0 {
		return fmt.Sprintf("%v\t[%s],\t[%s]", i.op, registerToString(i.a), registerToString(i.b))
	}

	return fmt.Sprintf("%v\t[%s],\t %d", i.op, registerToString(i.a), i.b)
}

type Memory [4]int

func (m Memory) String() string {
	return fmt.Sprintf("w: %d, x: %d, y: %d, z: %d", m[0], m[1], m[2], m[3])
}

type Stream struct {
	stream []int
	counter int
}

func (s *Stream) Get() int {
	if s.counter == len(s.stream) {
		panic("End of stream reached")
	}

	val := s.stream[s.counter]
	s.counter++
	return val
}

func NewStream(input string) Stream {
	var s Stream
	s.stream = make([]int, len(input))

	for i, b := range input {
		n, err := strconv.Atoi(string(b))
		if err != nil {
			panic(err)
		}

		if n == 0 {
			panic("0 is not a valid input")
		}

		s.stream[i] = n
	}

	return s
}

type Program []Instruction

func (p Program) ExecuteMem(input string, mem Memory) Memory {
	stream := NewStream(input)

	for _, i := range p {
		switch i.op {
		case INP:
			mem[i.a] = stream.Get()
		case ADD:
			mem[i.a] += i.b
		case ADDR:
			mem[i.a] += mem[i.b]
		case MUL:
			mem[i.a] *= i.b
		case MULR:
			mem[i.a] *= mem[i.b]
		case DIV:
			mem[i.a] /= i.b
		case DIVR:
			mem[i.a] /= mem[i.b]
		case MOD:
			mem[i.a] %= i.b
		case MODR:
			mem[i.a] %= mem[i.b]
		case EQL:
			val := 0
			if mem[i.a] == i.b {
				val = 1
			}
			mem[i.a] = val
		case EQLR:
			val := 0
			if mem[i.a] == mem[i.b] {
				val = 1
			}
			mem[i.a] = val
		default:
			panic("Unknown operator")
		}

		// fmt.Println(mem)
	}

	return mem
}

func (p Program) Execute(input string) Memory {
	var mem Memory

	return p.ExecuteMem(input, mem)
}

func Compile(input *bufio.Scanner) Program {
	var program Program

	for input.Scan() {
		line := input.Text()
		inst := strings.Split(line, " ")

		var instruction Instruction

		// Parse the operator
		switch inst[0] {
		case "inp":
			instruction.op = INP
		case "add":
			instruction.op = ADD
		case "mul":
			instruction.op = MUL
		case "div":
			instruction.op = DIV
		case "mod":
			instruction.op = MOD
		case "eql":
			instruction.op = EQL
		default:
			panic("Unknown instruction")
		}

		// Parse the first parameter, this is always a register
		instruction.a = parseRegister(inst[1])

		// All instructions, except INP, require a second parameter
		if instruction.op != INP {
			b := inst[2]
			if val, err := strconv.Atoi(b); err == nil {
				// If the parameter is a number we use that as the second parameter
				instruction.b = val
			} else {
				// Otherwise the parameter is a register, so we parse it and set it to the register
				instruction.b = parseRegister(b)
				// We also increment the operator by one to indicate that the second parameter is a register
				instruction.op++
			}
		}

		program = append(program, instruction)
	}

	return program
}

func (p Program) Optimize() Program {
	fmt.Printf("QUASI\n")
	p, _ = p.quasi(false)

	fmt.Printf("PRECOMP\n")
	p = p.precomp()

	return p
}

// func (p Program) Solver() {
// 	// There should be a total of 14 blocks
// 	mem := [4]int{3, 0, 0, 0}
// 	for i := range p {
// 		if p[i].op == INP {
// 			end := len(p)
// 			for j := i+1; j < len(p); j++ {
// 				if p[j].op == INP {
// 					end = j
// 					break
// 				}
// 			}

// 			tp := p[i+1:end]
// 			tp.Execute("", [4]int{w, 0, 0, 0}))

// 			break
// 		}
// 	}
// }

func step(w int, z int, a int, b int, pop bool) int {
	temp := z 

	if pop {
		// Pop value from stack
		z /= 26
	}

	if (temp % 26) + a != w {
		// Push new value to stack
		z *= 26
		z += w+b
	}

	return z
}

func reverse(w int, z int, a int, b int, pop bool) []int {
	var valid []int
	for zc := 0; zc < 1000; zc++ {
		res := step(w, zc, a, b, pop)
		if res == z {
			valid = append(valid, zc)
		}
	}

	return valid
}


func main() {
	challenge := aoc.New(2021, 24)

	challenge.Solution(1, func (input *bufio.Scanner) int {
		program := Compile(input)

		mem := program.Execute("13579246899999")
		fmt.Println(mem)

		w := []int{1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9}
		z := step(w[0], 0, 14, 1, false)
		z = step(w[1], z, 15, 7, false)
		z = step(w[2], z, 15, 13, false)
		z = step(w[3], z, -6, 10, true)
		z = step(w[4], z, 14, 0, false)
		z = step(w[5], z, -4, 13, true)
		z = step(w[6], z, 15, 11, false)
		z = step(w[7], z, 15, 6, false)
		z = step(w[8], z, 11, 1, false)
		z = step(w[9], z, 0, 7, true)
		z = step(w[10], z, 0, 11, true)
		z = step(w[11], z, -3, 14, true)
		z = step(w[12], z, -9, 4, true)
		z = step(w[13], z, -9, 10, true)
		fmt.Println(z)

		// solution := make(map[int]int)
		for w := 1; w <= 9; w++ {
			valid := reverse(w, 0, -9, 10, true)
			fmt.Println(w, valid)
			for _, v := range valid {
				check := step(w, v, -9, 10, true)

				if check != 0 {
					panic("CHECK FAILED")
				}
			}
		}


		for w := 1; w <= 9; w++ {
			fmt.Println(w, reverse(w, w+9, -9, 4, true))
		}

		// for _, i := range program {
		// 	fmt.Println(i)
		// }

		return -1
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		return 0
	})
}
