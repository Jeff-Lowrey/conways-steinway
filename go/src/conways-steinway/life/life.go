package main

import (
	"fmt"
	"math/rand"
	"time"
)

// Cell represents a cell in the grid
type Cell struct {
	Alive bool
}

// Grid represents the game board
type Grid struct {
	Cells []Cell
}

// NewGrid returns a new Game of Life grid with random initial values
func NewGrid(width, height int) *Grid {
	grid := &Grid{
		Cells: make([]Cell, width*height),
	}
	for i := range grid.Cells {
		// x := i % width
		// y := i / width
		grid.Cells[i] = Cell{Alive: rand.Intn(2) == 1} // Initialize random values
	}

	return grid
}

// Step simulates one generation of the Game of Life
func (g *Grid) Step() {
	for y := 0; y < g.Length(); y++ {
		for x := 0; x < g.Length(); x++ {
			count := g.neighboursCount(x, y)
			if g.Cells[x+y*g.Length()].Alive && (count == 2 || count == 3) {
				g.Cells[x+y*g.Length()].Alive = true
			} else if !g.Cells[x+y*g.Length()].Alive && count == 3 {
				g.Cells[x+y*g.Length()].Alive = true
			}
		}
	}
}

func (g *Grid) Length() int { return len(g.Cells) }

// neighboursCount returns the number of live neighbors for a given cell
func (g *Grid) neighboursCount(x, y int) int {
	count := 0
	for dy := -1; dy <= 1; dy++ {
		for dx := -1; dx <= 1; dx++ {
			if dy == 0 && dx == 0 {
				continue // Skip the center cell
			}

			ny := y + dy
			nx := x + dx

			liveNeighbors := 0
			for nx := min(nx, g.Length()-1); nx < g.Length(); nx++ {
				// ny := nx % g.Length()
				if g.Cells[nx+ny*g.Length()].Alive {
					liveNeighbors++
				}
			}

			count += liveNeighbors
		}
	}

	return count
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// printGrid prints the current state of the game board
func (g *Grid) printGrid() {
	for y := 0; y < g.Length(); y++ {
		for x := 0; x < g.Length(); x++ {
			if g.Cells[x+y*g.Length()].Alive {
				fmt.Print("#", x, y)
			} else {
				fmt.Print(".", x, y)
			}
		}
		fmt.Println()
	}
}

func main() {
	rand.Seed(time.Now().UnixNano())
	grid := NewGrid(10, 10)

	for generation := 0; generation < 10; generation++ {
		grid.printGrid()
		fmt.Printf("Generation %d\n", generation+1)
		grid.Step()
		time.Sleep(500 * time.Millisecond) // Pause for animation effect
	}
}
