package main

import (
	"fmt"
)

type Blueprint struct {
	ID                int
	OreRobotCost      Resources
	ClayRobotCost     Resources
	ObsidianRobotCost Resources
	GeodeRobotCost    Resources
}

func newBlueprint(id int, oreRobot Resources, clayRobot Resources, obsidianRobot Resources, geodeRobot Resources) Blueprint {
	return Blueprint{
		ID:                id,
		OreRobotCost:      oreRobot,
		ClayRobotCost:     clayRobot,
		ObsidianRobotCost: obsidianRobot,
		GeodeRobotCost:    geodeRobot,
	}
}

func (b *Blueprint) maxOreNeeded() int {
	return maxOfInts(b.OreRobotCost.Ore, b.ClayRobotCost.Ore, b.ObsidianRobotCost.Ore, b.GeodeRobotCost.Ore)
}

func (b *Blueprint) maxClayNeeded() int {
	return maxOfInts(b.OreRobotCost.Clay, b.ClayRobotCost.Clay, b.ObsidianRobotCost.Clay, b.GeodeRobotCost.Clay)
}

func (b *Blueprint) maxObsidianNeeded() int {
	return maxOfInts(b.OreRobotCost.Obsidian, b.ClayRobotCost.Obsidian, b.ObsidianRobotCost.Obsidian, b.GeodeRobotCost.Obsidian)
}

func parseLinesToBlueprints(inputLines []string) []Blueprint {
	format := "Blueprint %d: Each ore robot costs %d ore. Each clay robot costs %d ore. Each obsidian robot costs %d ore and %d clay. Each geode robot costs %d ore and %d obsidian."
	blueprints := []Blueprint{}	

	for _, line := range inputLines {
		var id, v1, v2, v3, v4, v5, v6 int
		fmt.Sscanf(line, format, &id, &v1, &v2, &v3, &v4, &v5, &v6)

		oreRobotCost := newResources(v1, 0, 0, 0)
		clayRobotCost := newResources(v2, 0, 0, 0)
		obsidianRobotCost := newResources(v3, v4, 0, 0)
		geodeRobotCost := newResources(v5, 0, v6, 0)

		blueprint := newBlueprint(id, oreRobotCost, clayRobotCost, obsidianRobotCost, geodeRobotCost)
		blueprints = append(blueprints, blueprint)
	}
	return blueprints
}