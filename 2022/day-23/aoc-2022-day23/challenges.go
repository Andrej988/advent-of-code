/*
--- Day 23: Unstable Diffusion ---
You enter a large crater of gray dirt where the grove is supposed to be. All around you, plants you imagine were expected to be full of fruit are instead withered and broken. A large group of Elves has formed in the middle of the grove.

"...but this volcano has been dormant for months. Without ash, the fruit can't grow!"

You look up to see a massive, snow-capped mountain towering above you.

"It's not like there are other active volcanoes here; we've looked everywhere."

"But our scanners show active magma flows; clearly it's going somewhere."

They finally notice you at the edge of the grove, your pack almost overflowing from the random star fruit you've been collecting. Behind you, elephants and monkeys explore the grove, looking concerned. Then, the Elves recognize the ash cloud slowly spreading above your recent detour.

"Why do you--" "How is--" "Did you just--"

Before any of them can form a complete question, another Elf speaks up: "Okay, new plan. We have almost enough fruit already, and ash from the plume should spread here eventually. If we quickly plant new seedlings now, we can still make it to the extraction point. Spread out!"

The Elves each reach into their pack and pull out a tiny plant. The plants rely on important nutrients from the ash, so they can't be planted too close together.

There isn't enough time to let the Elves figure out where to plant the seedlings themselves; you quickly scan the grove (your puzzle input) and note their positions.

For example:

....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
The scan shows Elves # and empty ground .; outside your scan, more empty ground extends a long way in every direction. The scan is oriented so that north is up; orthogonal directions are written N (north), S (south), W (west), and E (east), while diagonal directions are written NE, NW, SE, SW.

The Elves follow a time-consuming process to figure out where they should each go; you can speed up this process considerably. The process consists of some number of rounds during which Elves alternate between considering where to move and actually moving.

During the first half of each round, each Elf considers the eight positions adjacent to themself. If no other Elves are in one of those eight positions, the Elf does not do anything during this round. Otherwise, the Elf looks in each of four directions in the following order and proposes moving one step in the first valid direction:

If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
After each Elf has had a chance to propose a move, the second half of the round can begin. Simultaneously, each Elf moves to their proposed destination tile if they were the only Elf to propose moving to that position. If two or more Elves propose moving to the same position, none of those Elves move.

Finally, at the end of the round, the first direction the Elves considered is moved to the end of the list of directions. For example, during the second round, the Elves would try proposing a move to the south first, then west, then east, then north. On the third round, the Elves would first consider west, then east, then north, then south.

As a smaller example, consider just these five Elves:

.....
..##.
..#..
.....
..##.
.....
The northernmost two Elves and southernmost two Elves all propose moving north, while the middle Elf cannot move north and proposes moving south. The middle Elf proposes the same destination as the southwest Elf, so neither of them move, but the other three do:

..##.
.....
..#..
...#.
..#..
.....
Next, the northernmost two Elves and the southernmost Elf all propose moving south. Of the remaining middle two Elves, the west one cannot move south and proposes moving west, while the east one cannot move south or west and proposes moving east. All five Elves succeed in moving to their proposed positions:

.....
..##.
.#...
....#
.....
..#..
Finally, the southernmost two Elves choose not to move at all. Of the remaining three Elves, the west one proposes moving west, the east one proposes moving east, and the middle one proposes moving north; all three succeed in moving:

..#..
....#
#....
....#
.....
..#..
At this point, no Elves need to move, and so the process ends.

The larger example above proceeds as follows:

== Initial State ==
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............

== End of Round 1 ==
..............
.......#......
.....#...#....
...#..#.#.....
.......#..#...
....#.#.##....
..#..#.#......
..#.#.#.##....
..............
....#..#......
..............
..............

== End of Round 2 ==
..............
.......#......
....#.....#...
...#..#.#.....
.......#...#..
...#..#.#.....
.#...#.#.#....
..............
..#.#.#.##....
....#..#......
..............
..............

== End of Round 3 ==
..............
.......#......
.....#....#...
..#..#...#....
.......#...#..
...#..#.#.....
.#..#.....#...
.......##.....
..##.#....#...
...#..........
.......#......
..............

== End of Round 4 ==
..............
.......#......
......#....#..
..#...##......
...#.....#.#..
.........#....
.#...###..#...
..#......#....
....##....#...
....#.........
.......#......
..............

== End of Round 5 ==
.......#......
..............
..#..#.....#..
.........#....
......##...#..
.#.#.####.....
...........#..
....##..#.....
..#...........
..........#...
....#..#......
..............
After a few more rounds...

== End of Round 10 ==
.......#......
...........#..
..#.#..#......
......#.......
...#.....#..#.
.#......##....
.....##.......
..#........#..
....#.#..#....
..............
....#..#..#...
..............
To make sure they're on the right track, the Elves like to check after round 10 that they're making good progress toward covering enough ground. To do this, count the number of empty ground tiles contained by the smallest rectangle that contains every Elf. (The edges of the rectangle should be aligned to the N/S/E/W directions; the Elves do not have the patience to calculate arbitrary rectangles.) In the above example, that rectangle is:

......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..
In this region, the number of empty ground tiles is 110.

Simulate the Elves' process and find the smallest rectangle that contains the Elves after 10 rounds. How many empty ground tiles does that rectangle contain?

--- Part Two ---
It seems you're on the right track. Finish simulating the process and figure out where the Elves need to go. How many rounds did you save them?

In the example above, the first round where no Elf moved was round 20:
*/
package main

import (
	"fmt"
	"math"
)

func solveFirstChallenge(lines []string) {
	elves, _ := solveChallenge(lines, 10)
	result := calculateResult(elves)
	fmt.Println("Result: ", result)
}

func solveSecondChallenge(lines []string) {
	_, numOfRounds := solveChallenge(lines, 9999999999)
	fmt.Println("Number of rounds:", numOfRounds)
}

func solveChallenge(lines []string, roundLimit int) (map[Coordinates]bool, int) {
	elves := parseLines(lines)

	moves := []string{}
	moves = append(moves, North)
	moves = append(moves, South)
	moves = append(moves, West)
	moves = append(moves, East)
	
	round := 0

	for {
		round++

		elvesAdjecant := false
		proposedMoves := make(map[Coordinates][]Coordinates)
		for coords := range elves {
			hasToMove := false
			for _, adjCoord := range coords.getAllAdjacentCoords() {
				if isFieldTaken(elves, adjCoord) {
					hasToMove = true
					elvesAdjecant = true
					break
				}
			}

			if hasToMove {
				for _, moveDirection := range moves {
					fieldsBlocked := false
					for _, adjCoord := range coords.getAdjacentCoords(moveDirection) {
						if isFieldTaken(elves, adjCoord) {
							fieldsBlocked = true
							break
						}
					}
					if !fieldsBlocked {
						destCoords := coords.peek(moveDirection)
						sourceCoords, ok := proposedMoves[destCoords]
						if !ok {
							sourceCoords = []Coordinates{}
						} 
						sourceCoords = append(sourceCoords, coords)
						proposedMoves[destCoords] = sourceCoords
						break;
					}
				}
			}
		}

		if !elvesAdjecant {
			break
		}

		for destinationCoords, sourceCoordsArr := range proposedMoves {
			if len(sourceCoordsArr) == 1 {
				sourceCoords := sourceCoordsArr[0]
				delete(elves, sourceCoords)
				elves[destinationCoords] = true
			}
		}

		moves = shuffleMoves(moves)

		if round == roundLimit {
			break
		}
	}

	return elves, round
}

func isFieldTaken(elves map[Coordinates]bool, coords Coordinates) bool {
	_, ok := elves[coords]
	return ok
}

func shuffleMoves(moves []string) []string {
	firstMove := moves[0]
	moves = moves[1:]
	moves = append(moves, firstMove)
	return moves
}

func calculateResult(elves map[Coordinates]bool) int {
	min_x := 1000000000
	max_x := -1000000000
	min_y := 1000000000
	max_y := -1000000000

	for coords := range elves {
		min_x = int(math.Min(float64(min_x), float64(coords.X)))
		max_x = int(math.Max(float64(max_x), float64(coords.X)))
		min_y = int(math.Min(float64(min_y), float64(coords.Y)))
		max_y = int(math.Max(float64(max_y), float64(coords.Y)))
	}
	
	return ((max_x - min_x) +1) * ((max_y - min_y) +1)  - len(elves)
}