package main

type MapOfRocks struct {
	mapOfRocks map[Coordinates]string
}

func newMapOfRocks() MapOfRocks {
	mapOfRocks := make(map[Coordinates]string)
	for i := 0; i < 7; i++ {
		mapOfRocks[newCoordinates(0, i)] = "F"
	}
	return MapOfRocks{mapOfRocks: mapOfRocks}
}

func (m *MapOfRocks) getHeight() int {
	max := 0
	for k := range m.mapOfRocks {
		if k.X > max {
			max = k.X
		}
	}
	return max
}

func (m *MapOfRocks) contains(coords Coordinates) bool {
	_, ok := m.mapOfRocks[coords]
	return ok
}

func (m *MapOfRocks) addRocks(shape Shape) {
	for _, coords := range shape.coords {
		m.mapOfRocks[coords] = "#"
	}
}

func (m *MapOfRocks) maxHeights() []int {
	max := []int{0, 0, 0, 0, 0, 0, 0}
	for k := range m.mapOfRocks {
		if k.X > max[k.Y] {
			max[k.Y] = k.X
		}
	}
	return max
}

func minOfMaxHeights(maxHeights []int) int {
	result := int(^uint(0) >> 1)
	for _, i := range maxHeights {
		if i < result {
			result = i
		}
	}

	return result
}

func (m *MapOfRocks) relativeStateOfHeights() []int {
	maxHeights := m.maxHeights()
	min := minOfMaxHeights(maxHeights)
	for i, v := range maxHeights {
		maxHeights[i] = v - min
	}
	return maxHeights
}
