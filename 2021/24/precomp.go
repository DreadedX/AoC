package main

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
		switch i.op {
		case INP:
			r[i.a].known = false
			np = append(np, i)

		case ADD:
			if r[i.a].known {
				r[i.a].value += i.b
				continue
			}
			np = append(np, i)

		case ADDR:
			if r[i.a].known && r[i.b].known {
				r[i.a].value += r[i.b].value
				continue
			}

			if r[i.a].known {
				np = append(np, Instruction{MUL, i.a, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.a].value})
				}
			}

			if r[i.b].known {
				np = append(np, Instruction{MUL, i.b, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value})
				}
			}

			r[i.a].known = false
			np = append(np, i)

		case MUL:
			if i.b == 0 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
				}
				continue
			}

			if i.b == 1 {
				continue
			}

			if r[i.a].known {
				r[i.a].value *= i.b
				continue
			}

			np = append(np, i)

		case MULR:
			if r[i.a].known && r[i.a].value == 0 {
				r[i.a].value = 0
				continue
			}

			if r[i.b].known && r[i.b].value == 0 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
				}
				continue
			}

			if r[i.b].known && r[i.b].value == 1 {
				continue
			}

			if r[i.a].known && r[i.b].known {
				r[i.a].value *= r[i.b].value
				continue
			}

			if r[i.a].known {
				np = append(np, Instruction{MUL, i.a, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.a].value})
				}
			}

			if r[i.b].known {
				np = append(np, Instruction{MUL, i.b, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value})
				}
			}

			r[i.a].known = false
			np = append(np, i)

		case DIV:
			if i.b == 1 {
				continue
			}

			if r[i.a].known {
				r[i.a].value /= i.b
				continue
			}

			np = append(np, i)

		case DIVR:
			if r[i.a].known && r[i.a].value == 0 {
				r[i.a].value = 0
				continue
			}

			if r[i.b].known && r[i.b].value == 1 {
				continue
			}

			if r[i.a].known && r[i.b].known {
				r[i.a].value /= r[i.b].value
				continue
			}

			if r[i.a].known {
				np = append(np, Instruction{MUL, i.a, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.a].value})
				}
			}

			if r[i.b].known {
				np = append(np, Instruction{MUL, i.b, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value})
				}
			}

			r[i.a].known = false
			np = append(np, i)

		case MOD:
			if i.b == -1 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
				}
				continue
			}

			if i.b == 1 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
				}
				continue
			}

			if r[i.a].known {
				r[i.a].value %= i.b
				continue
			}

			np = append(np, i)

		case MODR:
			if r[i.a].known && r[i.a].value == 0 {
				r[i.a].value = 0
				continue
			}

			if r[i.b].known && r[i.b].value == -1 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
				}
				continue
			}

			if r[i.b].known && r[i.b].value == 1 {
				r[i.a].value = 0
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
				}
				continue
			}

			if r[i.a].known && r[i.b].known {
				r[i.a].value %= r[i.b].value
				continue
			}

			if r[i.a].known {
				np = append(np, Instruction{MUL, i.a, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.a].value})
				}
			}

			if r[i.b].known {
				np = append(np, Instruction{MUL, i.b, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value})
				}
			}

			r[i.a].known = false
			np = append(np, i)

		case EQL:
			if r[i.a].known {
				val := 0
				if r[i.a].value == i.b {
					val = 1
				}
				r[i.a].value = val
				continue
			}

			np = append(np, i)

		case EQLR:
			if i.a == i.b {
				r[i.a].value = 1
				if !r[i.a].known {
					r[i.a].value = 0
					r[i.a].known = true
					np = append(np, Instruction{MUL, i.a, 0})
				}
				continue
			}

			if r[i.a].known && r[i.b].known {
				val := 0
				if r[i.a].value == r[i.b].value {
					val = 1
				}
				r[i.a].value = val
				continue
			}

			if r[i.a].known {
				np = append(np, Instruction{MUL, i.a, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.a].value})
				}
			}

			if r[i.b].known {
				np = append(np, Instruction{MUL, i.b, 0})
				if r[i.b].value != 0 {
					np = append(np, Instruction{ADD, i.b, r[i.b].value})
				}
			}

			r[i.a].known = false
			np = append(np, i)

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
