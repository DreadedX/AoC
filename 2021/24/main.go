package main

import (
	"fmt"
)

func step(w int, z int, a int, b int, pop bool) int {
	temp := z

	if pop {
		// Pop value from stack
		z /= 26
	}

	if (temp%26)+a != w {
		// Push new value to stack
		z *= 26
		z += w + b
	}

	return z
}

func main() {
	// for w1 := 1; w1 <= 9; w1++ {
	// 	z1 := step(w1, 0, 14, 1, false)
	// 	for w2 := 1; w2 <= 9; w2++ {
	// 		z2 := step(w2, z1, 15, 7, false)
	// 		for w3 := 1; w3 <= 9; w3++ {
	// 			z3 := step(w3, z2, 15, 13, false)
	// 			for w4 := 1; w4 <= 9; w4++ {
	// 				z4 := step(w4, z3, -6, 10, true)
	// 				for w5 := 1; w5 <= 9; w5++ {
	// 					fmt.Println(w1, w2, w3, w4, w5)
	// 					z5 := step(w5, z4, 14, 0, false)
	// 					for w6 := 1; w6 <= 9; w6++ {
	// 						z6 := step(w6, z5, -4, 13, true)
	// 						for w7 := 1; w7 <= 9; w7++ {
	// 							z7 := step(w7, z6, 15, 11, false)
	// 							for w8 := 1; w8 <= 9; w8++ {
	// 								z8 := step(w8, z7, 15, 6, false)
	// 								for w9 := 1; w9 <= 9; w9++ {
	// 									z9 := step(w9, z8, 11, 1, false)
	// 									for w10 := 1; w10 <= 9; w10++ {
	// 										z10 := step(w10, z9, 0, 7, true)
	// 										for w11 := 1; w11 <= 9; w11++ {
	// 											z11 := step(w11, z10, 0, 11, true)
	// 											for w12 := 1; w12 <= 9; w12++ {
	// 												z12 := step(w12, z11, -3, 14, true)
	// 												for w13 := 1; w13 <= 9; w13++ {
	// 													z13 := step(w13, z12, -9, 4, true)

	// 													w14 := z13 - 9
	// 													if w14 <= 0 || w14 > 9 {
	// 														continue
	// 													}

	// 													z14 := step(w14, z13, -9, 10, true)

	// 													if z14 == 0 {
	// 														fmt.Println("SOLUTION FOUND:", w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14)
	// 														return
	// 													}
	// 												}
	// 											}
	// 										}
	// 									}
	// 								}
	// 							}
	// 						}
	// 					}
	// 				}
	// 			}
	// 		}
	// 	}
	// }

	for w1 := 1; w1 <= 9; w1++ {
		z1 := step(w1, 0, 14, 1, false)
		for w2 := 1; w2 <= 9; w2++ {
			z2 := step(w2, z1, 15, 7, false)
			for w3 := 1; w3 <= 9; w3++ {
				z3 := step(w3, z2, 15, 13, false)
				for w4 := 1; w4 <= 9; w4++ {
					z4 := step(w4, z3, -6, 10, true)
					for w5 := 1; w5 <= 9; w5++ {
						fmt.Println(w1, w2, w3, w4, w5)
						z5 := step(w5, z4, 14, 0, false)
						for w6 := 1; w6 <= 9; w6++ {
							z6 := step(w6, z5, -4, 13, true)
							for w7 := 1; w7 <= 9; w7++ {
								z7 := step(w7, z6, 15, 11, false)
								for w8 := 1; w8 <= 9; w8++ {
									z8 := step(w8, z7, 15, 6, false)
									for w9 := 1; w9 <= 9; w9++ {
										z9 := step(w9, z8, 11, 1, false)
										for w10 := 1; w10 <= 9; w10++ {
											z10 := step(w10, z9, 0, 7, true)
											for w11 := 1; w11 <= 9; w11++ {
												z11 := step(w11, z10, 0, 11, true)
												for w12 := 1; w12 <= 9; w12++ {
													z12 := step(w12, z11, -3, 14, true)
													for w13 := 1; w13 <= 9; w13++ {
														z13 := step(w13, z12, -9, 4, true)

														w14 := z13 - 9
														if w14 <= 0 || w14 > 9 {
															continue
														}

														z14 := step(w14, z13, -9, 10, true)

														if z14 == 0 {
															fmt.Println("SOLUTION FOUND:", w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14)
															return
														}
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}
}
