package main

import (
	"fmt"
	"strconv"
)

func parseRegister(r string) int {
	switch r {
	case "w":
		return 0
	case "x":
		return 1
	case "y":
		return 2
	case "z":
		return 3
	default:
		panic("Unknown register")
	}
}

func registerToString(r int) string {
	switch r {
	case 0:
		return "w"
	case 1:
		return "x"
	case 2:
		return "y"
	case 3:
		return "z"
	default:
		return fmt.Sprintf("%d", r)
	}
}

func isNumber(s string) bool {
	_, err := strconv.Atoi(s)

	return err == nil
}

