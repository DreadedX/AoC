package main

import "fmt"

type RegisterQuasiState struct {
	min int
	max int
}

type SubProgram struct {
	program Program
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
func (p Program) quasi(prnt bool) (Program, []SubProgram) {
	var np Program
	var sp []SubProgram
	var r [4]RegisterQuasiState

	counter1 := 0
	counter2 := 0
	broken := false

	for index, i := range p {
		if !broken {
			switch i.op {
			case INP:
				counter2 = counter1
				counter1 = index

				sp = append(sp, SubProgram{p[counter2:counter1], r[3].min, r[3].max})

				if prnt {
					fmt.Println("")
				}
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

			case DIV:
				r[i.a].min /= i.b
				r[i.a].max /= i.b

				if r[i.a].min > r[i.a].max {
					r[i.a].min, r[i.a].max = r[i.a].max, r[i.a].min
				}

			case DIVR:
				min1 := r[i.a].min / r[i.b].min
				min2 := r[i.a].min / r[i.b].max
				max1 := r[i.a].max / r[i.b].min
				max2 := r[i.a].max / r[i.b].max

				r[i.a].min = minv(min1, minv(min2, minv(max1, max2)))
				r[i.a].max = maxv(min1, maxv(min2, maxv(max1, max2)))

			case EQL:
				if i.b < r[i.a].min || i.b > r[i.a].max {
					r[i.a].min = 0
					r[i.a].max = 0

					if prnt {
						fmt.Printf("- %v \t %v\n", i, r)
					}
					np = append(np, Instruction{MUL, i.a, 0})
					if prnt {
						fmt.Printf("+ %v\n", np[len(np)-1])
					}
					continue
				}

				if r[i.a].min == r[i.a].max {
					val := 0
					if r[i.a].min == i.b {
						val = 1
					}

					r[i.a].min = val
					r[i.a].max = val

					if prnt {
						fmt.Printf("- %v \t %v\n", i, r)
					}
					np = append(np, Instruction{MUL, i.a, 0})
					if prnt {
						fmt.Printf("+ %v\n", np[len(np)-1])
					}
					np = append(np, Instruction{ADD, i.a, 1})
					if prnt {
						fmt.Printf("+ %v\n", np[len(np)-1])
					}
					continue
				}

				r[i.a].min = 0
				r[i.a].max = 1

			case EQLR:
				if r[i.b].max < r[i.a].min || r[i.b].min > r[i.a].max {
					r[i.a].min = 0
					r[i.a].max = 0

					if prnt {
						fmt.Printf("- %v \t %v\n", i, r)
					}
					np = append(np, Instruction{MUL, i.a, 0})
					if prnt {
						fmt.Printf("+ %v\n", np[len(np)-1])
					}
					continue
				}

				if r[i.a].min == r[i.a].max && r[i.b].min == r[i.b].max {
					val := 0
					if r[i.a].min == r[i.b].min {
						val = 1
					}

					r[i.a].min = val
					r[i.a].max = val

					if prnt {
						fmt.Printf("- %v \t %v\n", i, r)
					}
					np = append(np, Instruction{MUL, i.a, 0})
					if prnt {
						fmt.Printf("+ %v\n", np[len(np)-1])
					}
					np = append(np, Instruction{ADD, i.a, 1})
					if prnt {
						fmt.Printf("+ %v\n", np[len(np)-1])
					}
					continue
				}

				r[i.a].min = 0
				r[i.a].max = 1

			case MOD:
				if r[i.a].max <= 0 || r[i.a].max >= i.b {
					r[i.a].max = i.b - 1
				}

				if r[i.a].min <= 0 || r[i.a].min >= i.b {
					r[i.a].min = 0

				}

				// r[i.a].min = 0
				// r[i.a].max = i.b-1

				if r[i.a].min > r[i.a].max {
					r[i.a].min, r[i.a].max = r[i.a].max, r[i.a].min
				}


			default:
				if prnt {
					fmt.Printf("\tBROKE: %v\n", i.op)
				}
				broken = true

			}
		}

		if prnt {
			fmt.Printf("%v \t %v\n", i, r)
		}

		np = append(np, i)
	}

	sp = append(sp, SubProgram{p[counter1:], r[3].min, r[3].max})

	return np, sp[1:]
}
