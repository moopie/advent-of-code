package main

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"time"
)

type Point struct{ x, y int }

type Interval struct{ a, b int } // inclusive [a,b]

func min(a, b int) int { if a < b { return a }; return b }
func max(a, b int) int { if a > b { return a }; return b }

func parseInput(input string) []Point {
	fmt.Println("Parsing input...")
	t0 := time.Now()

	lines := strings.Split(strings.TrimSpace(input), "\n")
	points := make([]Point, 0, len(lines))
	for _, line := range lines {
		parts := strings.Split(strings.TrimSpace(line), ",")
		x, _ := strconv.Atoi(parts[0])
		y, _ := strconv.Atoi(parts[1])
		points = append(points, Point{x, y})
	}

	fmt.Printf("Parsed %d red points in %v\n", len(points), time.Since(t0))
	return points
}

// Build orthogonal polygon edges from consecutive red points (wrap around)
// Returns:
// - verticalByX: x -> list of y-intervals for vertical boundary segments
// - horizontalByY: y -> list of x-intervals for horizontal boundary segments
// - sortedVXKeys, sortedHYKeys: sorted keys for range querying
func buildEdges(poly []Point) (map[int][]Interval, map[int][]Interval, []int, []int) {
	fmt.Println("Building polygon edges...")
	t0 := time.Now()

	verticalByX := make(map[int][]Interval)
	horizontalByY := make(map[int][]Interval)

	n := len(poly)
	for i := 0; i < n; i++ {
		a := poly[i]
		b := poly[(i+1)%n]

		if a.x == b.x {
			y1 := min(a.y, b.y)
			y2 := max(a.y, b.y)
			verticalByX[a.x] = append(verticalByX[a.x], Interval{y1, y2})
		} else if a.y == b.y {
			x1 := min(a.x, b.x)
			x2 := max(a.x, b.x)
			horizontalByY[a.y] = append(horizontalByY[a.y], Interval{x1, x2})
		} else {
			panic("non-orthogonal consecutive points found")
		}
	}

	// Sort + merge intervals per key to speed queries
	merge := func(list []Interval) []Interval {
		if len(list) == 0 {
			return list
		}
		sort.Slice(list, func(i, j int) bool {
			if list[i].a != list[j].a {
				return list[i].a < list[j].a
			}
			return list[i].b < list[j].b
		})
		out := make([]Interval, 0, len(list))
		cur := list[0]
		for i := 1; i < len(list); i++ {
			in := list[i]
			if in.a <= cur.b+1 {
				if in.b > cur.b {
					cur.b = in.b
				}
			} else {
				out = append(out, cur)
				cur = in
			}
		}
		out = append(out, cur)
		return out
	}

	vKeys := make([]int, 0, len(verticalByX))
	for x := range verticalByX {
		verticalByX[x] = merge(verticalByX[x])
		vKeys = append(vKeys, x)
	}
	sort.Ints(vKeys)

	hKeys := make([]int, 0, len(horizontalByY))
	for y := range horizontalByY {
		horizontalByY[y] = merge(horizontalByY[y])
		hKeys = append(hKeys, y)
	}
	sort.Ints(hKeys)

	fmt.Printf("Edges built in %v (V-keys=%d, H-keys=%d)\n", time.Since(t0), len(vKeys), len(hKeys))
	return verticalByX, horizontalByY, vKeys, hKeys
}

func intervalContains(list []Interval, v int) bool {
	// list is sorted & merged
	i := sort.Search(len(list), func(i int) bool { return list[i].b >= v })
	if i == len(list) {
		return false
	}
	return list[i].a <= v && v <= list[i].b
}

// True if any interval overlaps interior inclusive range [low, high]
// Assumes list sorted & merged
func intervalOverlaps(list []Interval, low, high int) bool {
	if low > high || len(list) == 0 {
		return false
	}
	// Find first interval whose b >= low
	i := sort.Search(len(list), func(i int) bool { return list[i].b >= low })
	if i == len(list) {
		return false
	}
	// Overlap exists if list[i].a <= high
	return list[i].a <= high
}

// Cache for intersections at specific y values: for point-in-polygon parity
type IntersectionsCache struct {
	verticalByX map[int][]Interval
	vKeys       []int
	cache       map[int][]int // y -> sorted x intersections
}

func NewIntersectionsCache(verticalByX map[int][]Interval, vKeys []int) *IntersectionsCache {
	return &IntersectionsCache{
		verticalByX: verticalByX,
		vKeys:       vKeys,
		cache:       make(map[int][]int, 1024),
	}
}

// Compute x-intersections of the polygon with a ray at integer y
// For orthogonal polygons, only vertical edges contribute
// We treat intervals as inclusive; for parity, we use a half-open rule by sampling at y,
// and counting vertical edges that span that y (y in [a, b), conceptually)
// To approximate with integer endpoints, we use y in [a, b) by excluding the top endpoint
func (ic *IntersectionsCache) intersectionsAtY(y int) []int {
	if xs, ok := ic.cache[y]; ok {
		return xs
	}
	xs := make([]int, 0, 64)
	for _, x := range ic.vKeys {
		ints := ic.verticalByX[x]
		// Check if any interval spans y in [a, b) -> i.e. a <= y < b
		// Our stored intervals are inclusive; convert to half-open by requiring y < b
		// If interval is [a,b], spanning y is a <= y && y < b
		// This prevents double-counting at vertices
		// Note: if b == a (degenerate), it contributes nothing to crossings
		i := sort.Search(len(ints), func(i int) bool { return ints[i].b > y }) // first with b > y
		if i < len(ints) {
			if ints[i].a <= y && y < ints[i].b {
				xs = append(xs, x)
			}
		}
	}
	sort.Ints(xs)
	ic.cache[y] = xs
	return xs
}

// Boundary-aware inside test:
// - returns true if point is on boundary, or strictly inside by parity
func pointInsideOrOnBoundary(px, py int,
	verticalByX map[int][]Interval,
	horizontalByY map[int][]Interval,
	ic *IntersectionsCache,
) bool {
	// Boundary test first
	if vInts, ok := verticalByX[px]; ok {
		if intervalContains(vInts, py) {
			return true
		}
	}
	if hInts, ok := horizontalByY[py]; ok {
		if intervalContains(hInts, px) {
			return true
		}
	}

	// Parity test (ray to the right): count intersections with x > px
	xs := ic.intersectionsAtY(py)
	k := sort.Search(len(xs), func(i int) bool { return xs[i] > px })
	return ((len(xs) - k) % 2) == 1
}

// Check if polygon boundary passes through the *interior* of the rectangle
// Interior is open on the rectangle boundary; for tile coordinates, that corresponds
// to integer points strictly inside: x in (x1,x2), y in (y1,y2).
func boundaryCrossesRectangleInterior(
	x1, y1, x2, y2 int,
	verticalByX map[int][]Interval, horizontalByY map[int][]Interval,
	vKeys, hKeys []int,
) bool {
	// Interior ranges for integer samples
	ix1, ix2 := x1+1, x2-1
	iy1, iy2 := y1+1, y2-1
	if ix1 > ix2 || iy1 > iy2 {
		// rectangle is too thin to have interior points
		return false
	}

	// Any vertical boundary segment with x in (x1,x2) overlapping y interior?
	l := sort.SearchInts(vKeys, x1+1)
	r := sort.SearchInts(vKeys, x2) // first >= x2
	for idx := l; idx < r; idx++ {
		x := vKeys[idx]
		if intervalOverlaps(verticalByX[x], iy1, iy2) {
			return true
		}
	}

	// Any horizontal boundary segment with y in (y1,y2) overlapping x interior?
	l = sort.SearchInts(hKeys, y1+1)
	r = sort.SearchInts(hKeys, y2) // first >= y2
	for idx := l; idx < r; idx++ {
		y := hKeys[idx]
		if intervalOverlaps(horizontalByY[y], ix1, ix2) {
			return true
		}
	}

	return false
}

func FindSeating(input string) int64 {
	totalStart := time.Now()

	poly := parseInput(input)
	verticalByX, horizontalByY, vKeys, hKeys := buildEdges(poly)
	ic := NewIntersectionsCache(verticalByX, vKeys)

	fmt.Println("Searching for largest valid rectangle...")
	searchStart := time.Now()

	var best int64 = 0
	n := len(poly)

	for i := 0; i < n; i++ {
		if i%100 == 0 {
			fmt.Printf("Progress: i=%d/%d best=%d elapsed=%v cacheY=%d\n",
				i, n, best, time.Since(searchStart), len(ic.cache))
		}

		a := poly[i]
		for j := i + 1; j < n; j++ {
			b := poly[j]

			// Must form a rectangle (not a line)
			if a.x == b.x || a.y == b.y {
				continue
			}

			x1, x2 := min(a.x, b.x), max(a.x, b.x)
			y1, y2 := min(a.y, b.y), max(a.y, b.y)

			// Tile area (inclusive)
			area := int64(x2-x1+1) * int64(y2-y1+1)
			if area <= best {
				continue
			}

			// Necessary condition: all four corners must be inside-or-on-boundary
			if !pointInsideOrOnBoundary(x1, y1, verticalByX, horizontalByY, ic) {
				continue
			}
			if !pointInsideOrOnBoundary(x2, y1, verticalByX, horizontalByY, ic) {
				continue
			}
			if !pointInsideOrOnBoundary(x1, y2, verticalByX, horizontalByY, ic) {
				continue
			}
			if !pointInsideOrOnBoundary(x2, y2, verticalByX, horizontalByY, ic) {
				continue
			}

			// Sufficient condition (for simple polygons without holes):
			// boundary must not pass through rectangle interior
			if boundaryCrossesRectangleInterior(x1, y1, x2, y2, verticalByX, horizontalByY, vKeys, hKeys) {
				continue
			}

			best = area
		}
	}

	fmt.Println("Rectangle search finished in", time.Since(searchStart))
	fmt.Println("Total runtime:", time.Since(totalStart))
	fmt.Println("Max area:", best)
	return best
}

func main() {
	exe, err := os.Executable()
	if err != nil {
		panic(err)
	}
	input, err := os.ReadFile(filepath.Join(filepath.Dir(exe), "input.txt"))
	if err != nil {
		panic(err)
	}
	fmt.Println("Answer:", FindSeating(string(input)))
}