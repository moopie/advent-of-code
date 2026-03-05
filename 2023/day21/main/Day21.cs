namespace main;

public static class Day21
{
    public static int SolvePart1(string input, int steps)
    {
        var (grid, startx, starty) = ParseInput(input);

        var q = new Queue<(int X, int Y, int Steps)>();
        var visited = new HashSet<(int X, int Y, int Steps)>();
        var reachable = new HashSet<(int X, int Y)>();

        q.Enqueue((startx, starty, steps));
        visited.Add((startx, starty, steps));

        while (q.Count > 0)
        {
            var (x, y, s) = q.Dequeue();

            if (s == 0)
            {
                reachable.Add((x, y));
                continue;
            }

            var dirs = new (int, int)[]
            {
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1)
            };

            foreach (var (dx, dy) in dirs)
            {
                if (dy < 0 || dy >= grid.Length || dx < 0 || dx >= grid[dy].Length)
                {
                    continue;
                }

                if (grid[dy][dx] == '#')
                {
                    continue;
                }

                var state = (dx, dy, s - 1);

                if (visited.Contains(state))
                {
                    continue;
                }

                visited.Add(state);
                q.Enqueue(state);
            }
        }

        return reachable.Count;
    }

    public static long SolvePart2(string input, int steps)
    {
        var (grid, startx, starty) = ParseInput(input);

        var n = grid.Length;
        if (n != grid[0].Length)
        {
            throw new InvalidOperationException("Expected square grid for Part 2.");
        }

        // Residue class (period N)
        var r = Mod(steps, n);

        // Small steps? Just do the exact simulation on the infinite tiling.
        // (Keeps behavior intuitive for tiny step counts.)
        if (steps <= r + 2 * n)
        {
            return CountReachableInfinite(grid, startx, starty, steps);
        }

        // Burn-in: sample at k=2,3,4 (steps = r + k*N)
        const long k0 = 2;
        const long k1 = 3;
        const long k2 = 4;

        var s0 = r + (int)(k0 * n);
        var s1 = r + (int)(k1 * n);
        var s2 = r + (int)(k2 * n);

        long y0 = CountReachableInfinite(grid, startx, starty, s0);
        long y1 = CountReachableInfinite(grid, startx, starty, s1);
        long y2 = CountReachableInfinite(grid, startx, starty, s2);

        var (a, b, c) = FitQuadraticFrom3Points(k0, y0, k1, y1, k2, y2);

        long kTarget = (steps - r) / n;
        return a * kTarget * kTarget + b * kTarget + c;
    }

    private static (long A, long B, long C) FitQuadraticFrom3Points(long x0, long y0, long x1, long y1, long x2, long y2)
    {
        // Solve:
        // a*x0^2 + b*x0 + c = y0
        // a*x1^2 + b*x1 + c = y1
        // a*x2^2 + b*x2 + c = y2
        //
        // Using elimination (all integer-safe for AoC inputs).

        long d01 = x0 - x1;
        long d02 = x0 - x2;
        long d12 = x1 - x2;

        long s01 = x0 + x1;
        long s02 = x0 + x2;
        long s12 = x1 + x2;

        // From subtracting equations:
        // a*(x0^2 - x1^2) + b*(x0 - x1) = y0 - y1
        // => a*(x0 - x1)*(x0 + x1) + b*(x0 - x1) = y0 - y1
        // => (x0 - x1) * (a*(x0 + x1) + b) = y0 - y1
        //
        // Let:
        // p01 = (y0 - y1) / (x0 - x1) = a*(x0 + x1) + b
        // p02 = (y0 - y2) / (x0 - x2) = a*(x0 + x2) + b

        long p01 = (y0 - y1) / d01;
        long p02 = (y0 - y2) / d02;

        // Subtract:
        // p01 - p02 = a*((x0 + x1) - (x0 + x2)) = a*(x1 - x2)
        long a = (p01 - p02) / d12;

        // b = p01 - a*(x0 + x1)
        long b = p01 - a * s01;

        // c = y0 - a*x0^2 - b*x0
        long c = y0 - a * x0 * x0 - b * x0;

        return (a, b, c);
    }

    private static long CountReachableInfinite(char[][] grid, int startx, int starty, int steps)
    {
        var h = grid.Length;
        var w = grid[0].Length;

        var current = new HashSet<(int X, int Y)>();
        current.Add((startx, starty));

        for (var s = 0; s < steps; s++)
        {
            var next = new HashSet<(int X, int Y)>();

            foreach (var (x, y) in current)
            {
                var dirs = new (int, int)[]
                {
                    (x - 1, y),
                    (x + 1, y),
                    (x, y - 1),
                    (x, y + 1)
                };

                foreach (var (nx, ny) in dirs)
                {
                    // Infinite tiling: check wall using wrapped coordinates
                    var gx = Mod(nx, w);
                    var gy = Mod(ny, h);

                    if (grid[gy][gx] == '#')
                    {
                        continue;
                    }

                    next.Add((nx, ny));
                }
            }

            current = next;
        }

        return current.Count;
    }

    private static int Mod(int a, int m)
    {
        var r = a % m;
        if (r < 0)
        {
            r += m;
        }
        return r;
    }

    private static (char[][] Grid, int StartX, int StartY) ParseInput(string input)
    {
        var rows = input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(row => row.ToCharArray())
            .ToArray();

        var startX = -1;
        var startY = -1;

        for (var y = 0; y < rows.Length; y++)
        {
            for (var x = 0; x < rows[y].Length; x++)
            {
                if (rows[y][x] != 'S')
                {
                    continue;
                }

                startX = x;
                startY = y;
            }
        }

        return (rows, startX, startY);
    }
}
