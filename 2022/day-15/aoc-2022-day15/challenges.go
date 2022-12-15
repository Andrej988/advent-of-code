package main

import (
	"fmt"
	"math"
)

func solveFirstChallenge(sensors *[]Sensor, yIdx int) {
	beaconMap := buildBeaconMap(*sensors, yIdx);
	fmt.Println("No beacon entries: ", beaconMap.countNoBeaconEntries())
}

func solveSecondChallenge(sensors *[]Sensor, rowLimit int) {
	for _, sensor := range *sensors {
		startX := int(math.Min(math.Max(float64(sensor.Position.X - sensor.ClosestBeaconDistance), 0),float64(rowLimit)))
		endX := int(math.Min(math.Max(float64(sensor.Position.X + sensor.ClosestBeaconDistance), 0),float64(rowLimit)))
		
		for x := startX; x <= endX; x++ {
			minLeft := int(math.Max(float64(sensor.minColumnCovered(x).Y) -1, 0))
			maxRight := int(math.Min(float64(sensor.maxColumnCovered(x).Y) + 1,float64(rowLimit)))

			if x > 0 {
				adjLeft := newCoordinates(x, minLeft)
				if isNotCovered(sensors, &adjLeft) {
					return
				}
			}

			if x < rowLimit {
				adjRight := newCoordinates(x, maxRight)
				if isNotCovered(sensors, &adjRight) {
					return
				}
			}	
		}
	}
}	

func isNotCovered(sensors *[]Sensor, coords *Coordinates) bool {
	positionCovered := false
	for _, s := range *sensors {
		positionCovered = s.sensorCovers(*coords)
		if positionCovered {
			break
		}
	}	

	if !positionCovered {
		result := (coords.X * 4000000) + coords.Y
		fmt.Printf("Found X: %v; Y: %v, RESULT=%v \n", coords.X, coords.Y, result)
		return true
	}
	return false
}

func buildBeaconMap(sensors []Sensor, yIdx int) BeaconMap {
	beaconMap := newBeaconMap()

	for _, sensor := range sensors {
		if sensor.Position.Y == yIdx {
			beaconMap.addSensor(sensor.Position.X)
		}

		if sensor.Beacon.Y == yIdx {
			beaconMap.addBeacon(sensor.Beacon.X)
		}
			
		newRange := rangeOnX(&sensor, yIdx)

		if newRange >= 0 {
			beaconMap.addNoBeaconRange(sensor.Position.X - newRange, sensor.Position.X + newRange)
		}
	}
	return beaconMap;
}

func rangeOnX(sensor *Sensor, yIdx int) int {
	return sensor.ClosestBeaconDistance - int(math.Abs(float64(yIdx) - float64(sensor.Position.Y)))
}
