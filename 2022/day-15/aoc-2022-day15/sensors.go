package main

import (
	"math"
	"strconv"
	"strings"
)

type Sensor struct {
	Position              Coordinates
	Beacon                Coordinates
	ClosestBeaconDistance int
}

type Coordinates struct {
	X int
	Y int
}

func newCoordinates(x int, y int) Coordinates {
	return Coordinates{x, y}
}

func newSensor(sensor_x int, sensor_y int, beacon_x int, beacon_y int) Sensor {
	return Sensor{
		Position:              Coordinates{X: sensor_x, Y: sensor_y},
		Beacon: 			   Coordinates{X: beacon_x, Y: beacon_y},
		ClosestBeaconDistance: calculate_manhattan_distance(sensor_x, sensor_y, beacon_x, beacon_y),
	}
}

func calculate_manhattan_distance(x1 int, y1 int, x2 int, y2 int) int {
	return int(math.Abs(float64(x2) - float64(x1)) + math.Abs(float64(y2) - float64(y1)))
}

func calculate_manhattan_distance_coords(point1 *Coordinates, point2 *Coordinates) int {
	return calculate_manhattan_distance(point1.X, point1.Y, point2.X, point2.Y)
}

func parseSensors(lines []string) []Sensor {
	sensors := []Sensor{}
	for _, line := range lines {
		sensor_x, _ := strconv.ParseInt(strings.Trim(line[strings.Index(line, "x=") +2:strings.Index(line, ",")], " "), 10, 32)
		sensor_y, _ := strconv.ParseInt(strings.Trim(line[strings.Index(line, "y=") +2:strings.Index(line, ":")], " "), 10, 32)
		beacon_x, _ := strconv.ParseInt(strings.Trim(line[strings.LastIndex(line, "x=") +2:strings.LastIndex(line, ",")], " "), 10, 32)
		beacon_y, _ := strconv.ParseInt(strings.Trim(line[strings.LastIndex(line, "y=") +2:], " "), 10, 32)
		sensors = append(sensors, newSensor(int(sensor_x), int(sensor_y), int(beacon_x), int(beacon_y)))
	}
	return sensors
}

func (s *Sensor) sensorCovers(pos Coordinates) bool {
	return s.ClosestBeaconDistance >= calculate_manhattan_distance_coords(&s.Position, &pos)
}

func (s *Sensor) maxColumnCovered(row int) Coordinates {
	return newCoordinates(row, s.Position.Y + int(math.Abs(float64(s.ClosestBeaconDistance) - math.Abs(float64(row) - float64(s.Position.X)))))
}

func (s *Sensor) minColumnCovered(row int) Coordinates {
	return newCoordinates(row, s.Position.Y - int(math.Abs(float64(s.ClosestBeaconDistance) - math.Abs(float64(row) - float64(s.Position.X)))))
}