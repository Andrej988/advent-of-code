package main

import (
	"fmt"

	"github.com/edwingeng/deque/v2"
)

func solveFirstChallenge(blueprints []Blueprint) {
	total := 0

	// Could be run in parallel
	for _, blueprint := range blueprints {
		startingState := buildStartingState(blueprint, 24)
		result := dfs(startingState)
		total += result * blueprint.ID
	}
	fmt.Println("Quality level: ", total)
}

func solveSecondChallenge(blueprints []Blueprint) {
	total := 1

	// Could be run in parallel
	for _, blueprint := range blueprints {
		startingState := buildStartingState(blueprint, 32)
		result := dfs(startingState)
		total = total * result
	}
	fmt.Println("Quality level: ", total)
}

func dfs(startingState State) int {
	bestResult := -1
	queue := deque.NewDeque[State]()
	queue.PushFront(startingState)
	seenStates := make(map[string]bool)

	for {
		if queue.IsEmpty() {
			break
		}

		state := queue.PopFront()
		if state.TimeLeft == 0 {
			bestResult = maxInt(bestResult, state.Resources.Geodes)
			continue
		}

		//fmt.Printf("State: %v; best result: %v Seen count: %v \n", state, bestResult, len(seenStates))

		newPossibleStates := []State{}
		seenStates[state.toString()] = true

		copiedRobotState := buildRobots(state.Robots[OreRobot], state.Robots[ClayRobot], state.Robots[ObsidianRobot], state.Robots[GeodeRobot])
		skipAllRobotBuildingState := newState(state.Blueprint, copiedRobotState, state.TimeLeft -1, state.Resources)
		newPossibleStates = append(newPossibleStates, skipAllRobotBuildingState)

		// Build Geode Robot (if possible)
		if isPossibleToBuildRobot(state.Blueprint.GeodeRobotCost, state.Resources) && state.TimeLeft > 1 {
			newPossibleStates = append(newPossibleStates, buildRobotAndReturnNewPossibleState(GeodeRobot, state))
		}

		// Build Obsidian Robot (if possible)
		if isPossibleToBuildRobot(state.Blueprint.ObsidianRobotCost, state.Resources) && state.maxObsidianNeeded() > state.robotCount(ObsidianRobot) && state.TimeLeft > 2 {
			newPossibleStates = append(newPossibleStates, buildRobotAndReturnNewPossibleState(ObsidianRobot, state))
		}

		// Build Clay Robot (if possible)
		if isPossibleToBuildRobot(state.Blueprint.ClayRobotCost, state.Resources) && state.maxClayNeeded() > state.robotCount(ClayRobot) && state.TimeLeft > 2 {
			newPossibleStates = append(newPossibleStates, buildRobotAndReturnNewPossibleState(ClayRobot, state) )
		}

		// Build Ore Robot (if possible)
		if isPossibleToBuildRobot(state.Blueprint.OreRobotCost, state.Resources) && state.maxOreNeeded() > state.robotCount(OreRobot) && state.TimeLeft > 2 {
			newPossibleStates = append(newPossibleStates, buildRobotAndReturnNewPossibleState(OreRobot, state))
		}

		// Collect ores
		collectedResources := collectElements(state.Robots)

		for _, possibleState := range newPossibleStates {
			possibleState.addResourcesToExistingState(collectedResources)
			possibleState.NormalizeState()
			
			// State was already seen
			_, ok := seenStates[possibleState.toString()]
			if ok {
				continue
			}

			// Check if it is even possible to beat the best result so far
			if !isPossibleToGetNewBestResult(possibleState, bestResult) {
				continue
			}
			queue.PushFront(possibleState)
		}
	}

	return bestResult
}

func collectElements(robots map[string]int) Resources {
	var ore, clay, obsidian, geode int = 0, 0, 0, 0
	for k, v := range robots {
		switch k {
		case OreRobot:
			ore += v
		case ClayRobot:
			clay += v
		case ObsidianRobot:
			obsidian += v
		case GeodeRobot:
			geode += v
		}
	}
	return newResources(ore, clay, obsidian, geode)
}

func isPossibleToBuildRobot(robotCost Resources, resources Resources) bool {
	return robotCost.Ore <= resources.Ore && robotCost.Clay <= resources.Clay && robotCost.Obsidian <= resources.Obsidian
}

func buildRobotAndReturnNewPossibleState(robotType string, existingState State) State {
	newResources := copyToNewResources(existingState.Resources)
	newRobots := buildRobots(existingState.Robots[OreRobot], existingState.Robots[ClayRobot], existingState.Robots[ObsidianRobot], existingState.Robots[GeodeRobot])
	
	switch robotType {
	case OreRobot:
		newResources.removeUsedResources(existingState.Blueprint.OreRobotCost)
	case ClayRobot:
		newResources.removeUsedResources(existingState.Blueprint.ClayRobotCost)
	case ObsidianRobot:
		newResources.removeUsedResources(existingState.Blueprint.ObsidianRobotCost)
	case GeodeRobot:
		newResources.removeUsedResources(existingState.Blueprint.GeodeRobotCost)
	}
	
	newRobots[robotType] = newRobots[robotType] + 1
	return newState(existingState.Blueprint, newRobots, existingState.TimeLeft - 1, newResources)
}

// Very dumb check: Could be additionally improved!!!
func isPossibleToGetNewBestResult(possibleState State, bestResultSoFar int) bool {
	possibleBestOption := possibleState.Resources.Geodes
	numOfGeodeRobots := possibleState.robotCount(GeodeRobot)

	for i := possibleState.TimeLeft; i >= 0; i-- {
		possibleBestOption += numOfGeodeRobots
		numOfGeodeRobots++	
	}
	return possibleBestOption > bestResultSoFar
}

func buildStartingState(blueprint Blueprint, totalTime int) State {
	robots := buildRobots(1, 0, 0, 0)
	resources := newResources(0, 0, 0, 0)
	return newState(blueprint, robots, totalTime, resources)
}

func buildRobots(oreRobots int, clayRobots int, obsidianRobots int, geodeRobots int) map[string]int {
	robots := make(map[string]int)
	robots[OreRobot] = oreRobots
	robots[ClayRobot] = clayRobots
	robots[ObsidianRobot] = obsidianRobots
	robots[GeodeRobot] = geodeRobots
	return robots
}