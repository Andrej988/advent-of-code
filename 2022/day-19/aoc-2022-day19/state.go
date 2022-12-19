package main

import (
	"fmt"
	"strconv"
)

type Resources struct {
	Ore      int
	Clay     int
	Obsidian int
	Geodes   int
}

func newResources(ore int, clay int, obsidian int, geodes int) Resources {
	return Resources{
		Ore:      ore,
		Clay:     clay,
		Obsidian: obsidian,
		Geodes:   geodes,
	}
}

func (r *Resources) toString() string {
	return fmt.Sprintf("Res=[%v,%v,%v,%v]", r.Ore, r.Clay, r.Obsidian, r.Geodes)

}

func copyToNewResources(resources Resources) Resources {
	return newResources(resources.Ore, resources.Clay, resources.Obsidian, resources.Geodes)
}

func (r *Resources) add(resources Resources) {
	r.Ore += resources.Ore
	r.Clay += resources.Clay
	r.Obsidian += resources.Obsidian
	r.Geodes += resources.Geodes
}

func (r *Resources) removeUsedResources(usedResources Resources) {
	r.Ore -= usedResources.Ore
	r.Clay -= usedResources.Clay
	r.Obsidian -= usedResources.Obsidian
	r.Geodes -= usedResources.Geodes
}

func (r *Resources) normalizeResources(maxOre int, maxClay int, maxObsidian int) {
	r.Ore = minInt(r.Ore, maxOre)
	r.Clay = minInt(r.Clay, maxClay)
	r.Obsidian = minInt(r.Obsidian, maxObsidian)
}

type State struct {
	Blueprint Blueprint
	Robots    map[string]int
	TimeLeft  int
	Resources Resources
}

func newState(blueprint Blueprint, robots map[string]int, timeLeft int, resources Resources) State {
	return State{
		Blueprint: blueprint,
		Robots:    robots,
		TimeLeft:  timeLeft,
		Resources: resources,
	}
}

func (s *State) addResourcesToExistingState(resources Resources) {
	s.Resources.add(resources)
}

func (s *State) maxOreNeeded() int {
	return s.Blueprint.maxOreNeeded()
}

func (s *State) maxClayNeeded() int {
	return s.Blueprint.maxClayNeeded()
}

func (s *State) maxObsidianNeeded() int {
	return s.Blueprint.maxObsidianNeeded()
}

func (s *State) robotCount(robotType string) int {
	return s.Robots[robotType]
}

func (s *State) toString() string {
	robotsString := "("
	for k,v := range s.Robots {
		robotsString += fmt.Sprintf("%v=%v", k, strconv.Itoa(v))
	}
	robotsString += ")"
	result := fmt.Sprintf("BluId=%v; Time=%v; Robots=%v; %v", strconv.Itoa(s.Blueprint.ID), strconv.Itoa(s.TimeLeft), robotsString, s.Resources.toString())
	return result
}

func (s *State) NormalizeState() {
	maxOre := s.maxOreNeeded() * s.TimeLeft
	maxClay := s.maxClayNeeded() * s.TimeLeft
	maxObsidian := s.maxObsidianNeeded() * s.TimeLeft
	s.Resources.normalizeResources(maxOre, maxClay, maxObsidian)
}