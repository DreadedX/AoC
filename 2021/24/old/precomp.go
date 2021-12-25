package main

// import "fmt"

type RegisterState struct {
	value int
	last int
	known bool
}

// @TODO Apply DRY
// The function will go through all the instruction and precompute as much as possible
func (p Program) precomp() Program {
	var np Program
	var r [4]RegisterState

	// Initalize the registers
	for i := range r {
		r[i].known = true
	}

	for _, i := range p {
		// fmt.Println(i)
		switch i.op {
		case INP:
			r[i.a].known = false
			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case ADD:
			if r[i.a].known {
				r[i.a].value += i.b
				// fmt.Printf("- \t%v\n", r)
				continue
			}
			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case ADDR:
			if r[i.a].known && r[i.b].known {
				r[i.a].value += r[i.b].value
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.a].known {
				if r[i.a].value - r[i.a].last != 0 {
					np = append(np, Instruction{ADD, i.a, r[i.a].value - r[i.a].last})
					r[i.a].last = r[i.a].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			if r[i.b].known {
				if r[i.b].value - r[i.b].last != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value - r[i.b].last})
					r[i.b].last = r[i.b].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			r[i.a].known = false
			// fmt.Printf("- \t%v\n", r)
			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case MUL:
			if i.b == 0 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].last = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if i.b == 1 {
				continue
			}

			if r[i.a].known {
				r[i.a].value *= i.b
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case MULR:
			if r[i.a].known && r[i.a].value == 0 {
				r[i.a].value = 0
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.b].known && r[i.b].value == 0 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].last = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.b].known && r[i.b].value == 1 {
				continue
			}

			if r[i.a].known && r[i.b].known {
				r[i.a].value *= r[i.b].value
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.a].known {
				if r[i.a].value - r[i.a].last != 0 {
					np = append(np, Instruction{ADD, i.a, r[i.a].value - r[i.a].last})
					r[i.a].last = r[i.a].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			if r[i.b].known {
				if r[i.b].value - r[i.b].last != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value - r[i.b].last})
					r[i.b].last = r[i.b].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			r[i.a].known = false
			// fmt.Printf("- \t%v\n", r)
			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case DIV:
			if i.b == 1 {
				continue
			}

			if r[i.a].known {
				r[i.a].value /= i.b
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case DIVR:
			if r[i.a].known && r[i.a].value == 0 {
				r[i.a].value = 0
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.b].known && r[i.b].value == 1 {
				continue
			}

			if r[i.a].known && r[i.b].known {
				r[i.a].value /= r[i.b].value
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.a].known {
				if r[i.a].value - r[i.a].last != 0 {
					np = append(np, Instruction{ADD, i.a, r[i.a].value - r[i.a].last})
					r[i.a].last = r[i.a].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			if r[i.b].known {
				if r[i.b].value - r[i.b].last != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value - r[i.b].last})
					r[i.b].last = r[i.b].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			r[i.a].known = false
			// fmt.Printf("- \t%v\n", r)
			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case MOD:
			if i.b == -1 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].last = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if i.b == 1 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].last = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.a].known {
				r[i.a].value %= i.b
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case MODR:
			if r[i.a].known && r[i.a].value == 0 {
				r[i.a].value = 0
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.b].known && r[i.b].value == -1 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].last = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.b].known && r[i.b].value == 1 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].last = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.a].known && r[i.b].known {
				r[i.a].value %= r[i.b].value
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.a].known {
				if r[i.a].value - r[i.a].last != 0 {
					np = append(np, Instruction{ADD, i.a, r[i.a].value - r[i.a].last})
					r[i.a].last = r[i.a].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			if r[i.b].known {
				if r[i.b].value - r[i.b].last != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value - r[i.b].last})
					r[i.b].last = r[i.b].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			r[i.a].known = false
			// fmt.Printf("- \t%v\n", r)
			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case EQL:
			if r[i.a].known {
				val := 0
				if r[i.a].value == i.b {
					val = 1
				}
				r[i.a].value = val
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		case EQLR:
			if i.a == i.b {
				r[i.a].value = 1
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].last = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.a].known && r[i.b].known {
				val := 0
				if r[i.a].value == r[i.b].value {
					val = 1
				}
				r[i.a].value = val
				// fmt.Printf("- \t%v\n", r)
				continue
			}

			if r[i.a].known {
				if r[i.a].value - r[i.a].last != 0 {
					np = append(np, Instruction{ADD, i.a, r[i.a].value - r[i.a].last})
					r[i.a].last = r[i.a].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			if r[i.b].known {
				if r[i.b].value - r[i.b].last != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value - r[i.b].last})
					r[i.b].last = r[i.b].value
					// fmt.Printf("- \t%v\n", np[len(np)-1])
				}
			}

			r[i.a].known = false
			// fmt.Printf("- \t%v\n", r)
			np = append(np, i)
			// fmt.Printf("- \t%v\n", np[len(np)-1])

		}
	}

	// Emit all the non-zero known registers
	for i := range r {
		if r[i].known && r[i].value != 0 {
			np = append(np, Instruction{ADD, i, r[i].value})
		}
	}

	return np
}
