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

func (p Program) Execute(input string) Memory {
	stream := NewStream(input)
	var mem Memory

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
	// previous := len(p)
	for counter := 0; counter < 4; counter++ {
		fmt.Printf("PRECOMP %d\n", counter)
		p = p.precomp()

		fmt.Printf("QUASI %d\n", counter)
		p = p.quasi()

		// if previous == len(p) {
		// 	fmt.Println("Done optimizing")
		// 	break
		// }

		// previous = len(p)
	}

	return p
}

func main() {
	challenge := aoc.New(2021, 24)

	// challenge.Test(`inp x
// mul x -1`, []int{-1, -1})

	// challenge.Test(`inp z
// inp x
// mul z 3
// eql z x`, []int{-1, -1})

	// challenge.Test(`inp w
// add z w
// mod z 2
// div w 2
// add y w
// mod y 2
// div w 2
// add x w
// mod x 2
// div w 2
// mod w 2`, []int{-1, -1})

	challenge.Test(`inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y`, []int{-1, -1})

	challenge.Test(`inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y`, []int{-1, -1})

	// challenge.Test(``, []int{-1, -1})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		program := Compile(input)

		fmt.Println("ORIGINAL")
		for _, i := range program {
			fmt.Printf("%v\n", i)
		}
		fmt.Println(len(program))

		fmt.Println("OPTIMIZED")
		opti := program.Optimize()
		for _, i := range opti {
			fmt.Printf("%v\n", i)
		}
		fmt.Println(len(opti))

		str := "25579246899999"
		fmt.Println(program.Execute(str))
		fmt.Println(opti.Execute(str))

		// fmt.Println(mem)

		return 0
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		return 0
	})
}
