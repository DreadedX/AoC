package main

import (
	"AoC/2021/common"
	"bufio"
	"encoding/hex"
)

// Also implements a special operator -1, this operator will sum the versions of all (sub)packages
func parsePackage(data []int, offset int, operator int, numberOfPackets int, lengthOfPackets int) (int, int) {
	total := 0
	// If we are going to multiply we need to start with 1
	if operator == 1 {
		total = 1
	}

	start := offset
	for packetNumber := 0; (packetNumber < numberOfPackets || lengthOfPackets > offset - start); packetNumber++ {
		version := 0
		for i := 0; i < 3; i++ {
			version <<= 1
			version += data[offset + i]
		}
		offset += 3

		typeID := 0
		for i := 0; i < 3; i++ {
			typeID <<= 1
			typeID += data[offset + i]
		}
		offset += 3

		value := 0
		if typeID == 4 {
			done := false
			for !done {
				done = data[offset + 0] == 0
				offset++
				for i := 0; i < 4; i++ {
					value <<= 1
					value += data[offset + i]
				}
				offset += 4
			}

			if operator == -1 {
				value = version
			}
		} else {
			lengthTypeID := data[offset + 0]
			offset++

			lop := 0
			nop := 0
			if lengthTypeID == 0 {
				for i := 0; i < 15; i++ {
					lop <<= 1
					lop += data[offset + i]
				}
				offset += 15
			} else if lengthTypeID == 1 {
				for i := 0; i < 11; i++ {
					nop <<= 1
					nop += data[offset + i]
				}
				offset += 11
			} else {
				panic("Unknown lengthTypeID")
			}

			if operator == -1 {
				typeID = -1
			}

			offset, value = parsePackage(data, offset, typeID, nop, lop)

			if operator == -1 {
				value += version
			}
		}

		switch operator {
		case -1, 0:
			total += value
		case 1:
			total *= value
		case 2:
			if value < total || packetNumber == 0 {
				total = value
			}
		case 3:
			if value > total || packetNumber == 0 {
				total = value
			}
		case 5:
			if packetNumber == 0 {
				total = value
			} else if packetNumber == 1 {
				if total > value {
					total = 1
				} else {
					total = 0
				}
			} else {
				panic("To many sub packets for 'greater than' operator")
			}
		case 6:
			if packetNumber == 0 {
				total = value
			} else if packetNumber == 1 {
				if total < value {
					total = 1
				} else {
					total = 0
				}
			} else {
				panic("To many sub packets for 'less than' operator")
			}
		case 7:
			if packetNumber == 0 {
				total = value
			} else if packetNumber == 1 {
				if total == value {
					total = 1
				} else {
					total = 0
				}
			} else {
				panic("To many sub packets for 'equal to' operator")
			}
		}
	}

	return offset, total
}

func parseInput(input *bufio.Scanner) []int {
	input.Scan()
	in, _ := hex.DecodeString(input.Text())

	var data []int
	for _, b := range in {
		for i := 7; i >= 0; i-- {
			data = append(data, int((b >> i) & 1))
		}
	}

	return data
}

func main() {
	challenge := aoc.New(2021, 16)

	challenge.Test(`D2FE28`, []int{6, -2})
	challenge.Test(`8A004A801A8002F478`, []int{16, -2})
	challenge.Test(`620080001611562C8802118E34`, []int{12, -2})
	challenge.Test(`C0015000016115A2E0802F182340`, []int{23, -2})
	challenge.Test(`A0016C880162017C3686B18A3D4780`, []int{31, -2})

	challenge.Test(`C200B40A82`, []int{-2, 3})
	challenge.Test(`04005AC33890`, []int{-2, 54})
	challenge.Test(`880086C3E88112`, []int{-2, 7})
	challenge.Test(`CE00C43D881120`, []int{-2, 9})
	challenge.Test(`D8005AC2A8F0`, []int{-2, 1})
	challenge.Test(`F600BC2D8F`, []int{-2, 0})
	challenge.Test(`9C005AC2F8F0`, []int{-2, 0})
	challenge.Test(`9C0141080250320F1802104A08`, []int{-2, 1})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		data := parseInput(input)
		_, sum := parsePackage(data, 0, -1, 1, 0)

		return sum
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		data := parseInput(input)
		_, sum := parsePackage(data, 0, 0, 1, 0)

		return sum
	})
}
