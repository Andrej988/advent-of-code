package main

const ENTRY_TYPE_SENSOR = "S"
const ENTRY_TYPE_BEACON = "B"
const ENTRY_TYPE_NO_BEACON = "#"

type BeaconMap struct {
	beaconMap map[int]string
}

func newBeaconMap() BeaconMap {
	return BeaconMap{
		beaconMap: make(map[int]string),
	}
}

func (b *BeaconMap) contains(x int) bool {
	_, ok := b.beaconMap[x]
	if ok {
		return true
	} else {
		return false
	}
}

func (b *BeaconMap) addSensor(x int) {
	b.beaconMap[x] = ENTRY_TYPE_SENSOR
}

func (b *BeaconMap) addBeacon(x int) {
	b.beaconMap[x] = ENTRY_TYPE_BEACON
}

func (b *BeaconMap) addNoBeacon(x int) {
	if !b.contains(x) {
		b.beaconMap[x] = ENTRY_TYPE_NO_BEACON
	}
}

func (b *BeaconMap) addNoBeaconRange(fromX int, toX int) {
	for i := fromX; i <= toX; i++ {
		b.addNoBeacon(i)
	}
}

func (b *BeaconMap) countNoBeaconEntries() int {
	count := 0
	for _, val := range b.beaconMap {
		if val == ENTRY_TYPE_NO_BEACON {
			count++
		}
	}
	return count
}