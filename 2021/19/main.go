package main

import (
	aoc "AoC/2021/common"
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

type Vec struct {
	X int
	Y int
	Z int
}

func intcos(i int) int {
	var cos int
	switch i {
	case 0:
		cos = 1
	case 1,3:
		cos = 0
	case 2:
		cos = -1
	default:
		panic("Invalid rotation")
	}

	return cos
}

func intsin(i int) int {
	var sin int
	switch i {
	case 0, 2:
		sin = 0
	case 1:
		sin = 1
	case 3:
		sin = -1
	default:
		panic("Invalid rotation")
	}

	return sin
}

// We only allow 90 degree rotations
func (v Vec) rotate(r Vec) Vec {
	var out Vec

	cosX := intcos(r.X)
	sinX := intsin(r.X)

	cosY := intcos(r.Y)
	sinY := intsin(r.Y)

	cosZ := intcos(r.Z)
	sinZ := intsin(r.Z)

	Axx := cosZ*cosY
	Axy := cosZ*sinY*sinX - sinZ*cosX
	Axz := cosZ*sinY*cosX + sinZ*sinX

	Ayx := sinZ*cosY
	Ayy := sinZ*sinY*sinX + cosZ*cosX
	Ayz := sinZ*sinY*cosX - cosZ*sinX

	Azx := -sinY
	Azy := cosY*sinX
	Azz := cosY*cosX

	out.X = Axx*v.X + Axy*v.Y + Axz*v.Z
	out.Y = Ayx*v.X + Ayy*v.Y + Ayz*v.Z
	out.Z = Azx*v.X + Azy*v.Y + Azz*v.Z

	return out
}

func (a Vec) add(b Vec) Vec {
	var out Vec

	out.X = a.X + b.X
	out.Y = a.Y + b.Y
	out.Z = a.Z + b.Z

	return out
}

func (a Vec) subtract(b Vec) Vec {
	var out Vec

	out.X = a.X - b.X
	out.Y = a.Y - b.Y
	out.Z = a.Z - b.Z

	return out
}

func (a Vec) equals(b Vec) bool {
	return a.X == b.X && a.Y == b.Y && a.Z == b.Z
}

func (a Vec) distance(b Vec) int {
	x := a.X - b.X
	if x < 0 {
		x = -x
	}

	y := a.Y - b.Y
	if y < 0 {
		y = -y
	}

	z := a.Z - b.Z
	if z < 0 {
		z = -z
	}

	return x + y + z
}

type Scanner struct {
	ID int
	Position Vec
	Beacons []Vec
}

func mapScanners(input *bufio.Scanner) []Scanner {
	var scanners []Scanner
	var scanner *Scanner
	for input.Scan() {
		line := input.Text()
		if len(line) == 0 {
			continue
		}

		if strings.Contains(line, "scanner") {
			id := len(scanners)
			scanners = append(scanners, Scanner{ID: id})
			scanner = &scanners[id]
		} else {
			parts :=  strings.Split(line, ",")
			var coords [3]int
			for i, p := range parts {
				coords[i], _ = strconv.Atoi(p)
			}

			scanner.Beacons = append(scanner.Beacons, Vec{X: coords[0], Y: coords[1], Z: coords[2]})
		}
	}

	var known []*Scanner
	known = append(known, &scanners[0])

	for j := 0; j < len(known); j++ {
		ref := known[j]
		fmt.Printf("Scanner %d is reference\n", ref.ID)

		outer:
		for i := range scanners {
			for k := range known {
				if scanners[i].ID == known[k].ID {
					continue outer
				}
			}

			matcher:
			// Pick a reference point for each beacon
			for _, pref := range ref.Beacons {
				for _, pi := range scanners[i].Beacons {
					counter := 0

					// Go through all the points in the beacons
					for _, bref := range ref.Beacons {
						for _, bi := range scanners[i].Beacons {
							// If the distance to the respective beacon is the same it is an indaction that they might be the same point
							if bref.distance(pref) == bi.distance(pi) {
								counter++

								// If we find 12 points of overlap we can assume that they overlap
								if counter >= 12 {
									// Since the beacons overlap compared to their respective reference point,
									// we know that bref should be equal bi and pref should be equal to pi
									// So we can form two vectors, these should have the same magnitude and direction
									l1 := bref.subtract(pref)
									l2 := bi.subtract(pi)

									var rot Vec

									// In order to find the correct rotation we just apply all 24 rotations
									// and figure out which  result in l1 = l2
									rotate:
									for rx := 0; rx < 4; rx++ {
										for ry := 0; ry < 4; ry++ {
											for rz := 0; rz < 4; rz++ {
												rot = Vec{rx, ry, rz}
												if l1.equals(l2.rotate(rot)) {
													fmt.Printf("Rotation is %v\n", rot)
													break rotate
												}
											}
										}
									}

									// Update the postion of the scanner and all the beacons
									scanners[i].Position = bref.subtract(bi.rotate(rot))
									for k := range scanners[i].Beacons {
										scanners[i].Beacons[k] = scanners[i].Position.add(scanners[i].Beacons[k].rotate(rot))
									}

									known = append(known, &scanners[i])

									fmt.Printf("Scanner %d is positioned at %v\n", scanners[i].ID, scanners[i].Position)
									break matcher
								}
							}
						}
					}
				}
			}
		}
	}

	return scanners
}

func main() {
	challenge := aoc.New(2021, 19)

	challenge.Test(`--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14`, []int{79, 3621})

	challenge.Solution(1, func (input *bufio.Scanner) int {
		scanners := mapScanners(input)

		var beacons []Vec
		for _, scanner := range scanners {
			findUnique:
			for _, beacon := range scanner.Beacons {
				for _, b := range beacons {
					if beacon.equals(b) {
						continue findUnique
					}
				}

				beacons = append(beacons, beacon)
			}
		}

		return len(beacons)
	})

	challenge.Solution(2, func (input *bufio.Scanner) int {
		scanners := mapScanners(input)

		largest := 0
		for _, a := range scanners {
			for _, b := range scanners {
				if a.ID == b.ID {
					continue
				}

				distance := a.Position.distance(b.Position)

				if distance > largest {
					fmt.Printf("Scanner %d and %d are the farthest apart\n", a.ID, b.ID)
					largest = distance
				}
			}
		}

		return largest
	})
}
