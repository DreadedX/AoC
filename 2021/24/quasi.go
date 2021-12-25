package main

import "fmt"

type RegisterQuasiState struct {
	min int
	max int
}

func minv(a int, b int) int {
	if a < b {
		return a
	}
	return b
}

func maxv(a int, b int) int {
	if a > b {
		return a
	}
	return b
}

// This function will try to eliminate eql statements
func (p Program) quasi() Program {
	var np Program
	var r [4]RegisterQuasiState

	broken := false

	for _, i := range p {
		if !broken {
			switch i.op {
			case INP:
				r[i.a].min = 1
				r[i.a].max = 9

			case ADD:
				r[i.a].min += i.b
				r[i.a].max += i.b

			case ADDR:
				r[i.a].min = r[i.a].min + r[i.b].min
				r[i.a].max = r[i.a].max + r[i.b].max

			case MUL:
				r[i.a].min *= i.b
				r[i.a].max *= i.b

				if r[i.a].min > r[i.a].max {
					r[i.a].min, r[i.a].max = r[i.a].max, r[i.a].min
				}

			case MULR:
				min1 := r[i.a].min * r[i.b].min
				min2 := r[i.a].min * r[i.b].max
				max1 := r[i.a].max * r[i.b].min
				max2 := r[i.a].max * r[i.b].max

				r[i.a].min = minv(min1, minv(min2, minv(max1, max2)))
				r[i.a].max = maxv(min1, maxv(min2, maxv(max1, max2)))

			case EQL:
				if i.b < r[i.a].min || i.b > r[i.a].max {
					r[i.a].min = 0
					r[i.a].max = 0

					np = append(np, Instruction{MUL, i.a, 0})
					fmt.Println("\tReplace EQL with mul 0")
					continue
				}

				if r[i.a].min == r[i.a].max {
					val := 0
					if r[i.a].min == i.b {
						val = 1
					}

					r[i.a].min = val
					r[i.a].max = val

					np = append(np, Instruction{MUL, i.a, 0})
					np = append(np, Instruction{ADD, i.a, 1})
					fmt.Println("\tReplace EQL with mul 0, add 1")
					continue
				}

				r[i.a].min = 0
				r[i.a].max = 1

			case EQLR:
				if r[i.b].max < r[i.a].min || r[i.b].min > r[i.a].max {
					r[i.a].min = 0
					r[i.a].max = 0

					np = append(np, Instruction{MUL, i.a, 0})
					fmt.Println("\tReplace EQLR with mul 0")
					continue
				}

				if r[i.a].min == r[i.a].max && r[i.b].min == r[i.b].max {
					val := 0
					if r[i.a].min == r[i.b].min {
						val = 1
					}

					r[i.a].min = val
					r[i.a].max = val

					np = append(np, Instruction{MUL, i.a, 0})
					np = append(np, Instruction{ADD, i.a, 1})
					fmt.Println("\tReplace EQLR with mul 0, add 1")
					continue
				}

				r[i.a].min = 0
				r[i.a].max = 1

			case MOD:
				r[i.a].min = 0
				r[i.a].max = i.b

				if r[i.a].min > r[i.a].max {
					r[i.a].min, r[i.a].max = r[i.a].max, r[i.a].min
				}


			default:
				fmt.Println("\tBROKE")
				broken = true

			}
		}

		np = append(np, i)
	}


	return np
}
