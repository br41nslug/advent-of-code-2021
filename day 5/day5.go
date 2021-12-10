package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Point struct {
	X, Y int
}

type Line struct {
	start, end Point
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func makePoint(X int, Y int) Point {
	return Point{X, Y}
}

func parsePoint(str string) Point {
	parts := strings.Split(str, ",")
	X, _ := strconv.Atoi(parts[0])
	Y, _ := strconv.Atoi(parts[1])
	return Point{X, Y}
}

func parseLine(line string) Line {
	parts := strings.Split(line, " -> ")
	start := parsePoint(parts[0])
	end := parsePoint(parts[1])
	return Line{start, end}
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	cells := map[Point]int{}
	for scanner.Scan() {
		line := parseLine(scanner.Text())
		if line.start.X == line.end.X {
			minY := min(line.start.Y, line.end.Y)
			maxY := max(line.start.Y, line.end.Y)
			for i := minY; i <= maxY; i++ {
				P := makePoint(line.start.X, i)
				cells[P] = cells[P] + 1
			}
		}
		if line.start.Y == line.end.Y {
			minX := min(line.start.X, line.end.X)
			maxX := max(line.start.X, line.end.X)
			for i := minX; i <= maxX; i++ {
				P := makePoint(i, line.start.Y)
				cells[P] = cells[P] + 1
			}
		}
	}
	// fmt.Println(cells)
	total := 0
	for _, count := range cells {
		if count >= 2 {
			total++
		}
	}
	fmt.Println("Overlapping cells: ", total)

	if err := scanner.Err(); err != nil {
		log.Println(err)
	}
}
