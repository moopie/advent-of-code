Console.WriteLine("AOC 2023 day 10!");

var input = File.ReadAllText("input.txt");

Console.WriteLine($"Part 1: {Day10.SolvePart1(input)}");
Console.WriteLine($"Part 2: {Day10.SolvePart2(input)}");

public record Grid(char[][] Cells, int Width, int Height, (int X, int Y) Start);

public static class Day10
{
    public static int SolvePart1(string input)
    {
        var grid = ParseInput(input);
        var path = GetPath(grid);

        return path.Values.Max();
    }

    public static int SolvePart2(string input)
    {
        var grid = ParseInput(input);
        var path = GetPath(grid);
        var loop = path.Keys.ToHashSet();

        int w = grid.Width;
        int h = grid.Height;

        int ew = w * 3;
        int eh = h * 3;

        var expanded = new bool[eh, ew]; // true = wall

        // Draw pipes into expanded grid
        foreach (var (x, y) in loop)
        {
            char tile = grid.Cells[y][x];

            int cx = x * 3 + 1;
            int cy = y * 3 + 1;

            expanded[cy, cx] = true;

            void Mark(int dx, int dy)
            {
                expanded[cy + dy, cx + dx] = true;
            }

            switch (tile)
            {
                case '|':
                    Mark(0, -1);
                    Mark(0, 1);
                    break;
                case '-':
                    Mark(-1, 0);
                    Mark(1, 0);
                    break;
                case 'L': // up + right
                    Mark(0, -1);
                    Mark(1, 0);
                    break;
                case 'J': // up + left
                    Mark(0, -1);
                    Mark(-1, 0);
                    break;
                case '7': // down + left
                    Mark(0, 1);
                    Mark(-1, 0);
                    break;
                case 'F': // down + right
                    Mark(0, 1);
                    Mark(1, 0);
                    break;
                case 'S':
                    // infer shape from neighbors
                    foreach (var (dx, dy) in new[] { (-1, 0), (1, 0), (0, -1), (0, 1) })
                    {
                        int nx = x + dx;
                        int ny = y + dy;
                        if (loop.Contains((nx, ny)))
                            Mark(dx, dy);
                    }
                    break;
            }
        }

        // Flood fill expanded grid
        var visited = new bool[eh, ew];
        var queue = new Queue<(int X, int Y)>();

        queue.Enqueue((0, 0));
        visited[0, 0] = true;

        var dirs = new[]
        {
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1)
    };

        while (queue.Count > 0)
        {
            var (x, y) = queue.Dequeue();

            foreach (var (dx, dy) in dirs)
            {
                int nx = x + dx;
                int ny = y + dy;

                if (nx < 0 || ny < 0 || nx >= ew || ny >= eh)
                    continue;

                if (visited[ny, nx] || expanded[ny, nx])
                    continue;

                visited[ny, nx] = true;
                queue.Enqueue((nx, ny));
            }
        }

        // Count interior cells
        int inside = 0;

        for (int y = 0; y < h; y++)
        {
            for (int x = 0; x < w; x++)
            {
                if (loop.Contains((x, y)))
                    continue;

                int cx = x * 3 + 1;
                int cy = y * 3 + 1;

                if (!visited[cy, cx])
                    inside++;
            }
        }

        return inside;
    }

    private static Grid ParseInput(string input, bool ignoreStartingPoint = true)
    {
        var lines = input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Trim())
            .Where(line => !string.IsNullOrWhiteSpace(line))
            .ToArray();

        var height = lines.Length;
        var width = lines[0].Length;

        var grid = new char[height][];
        (int X, int Y) start = (-1, -1);

        for (var y = 0; y < height; y++)
        {
            grid[y] = lines[y].ToCharArray();

            for (var x = 0; x < width; x++)
            {
                if (grid[y][x] == 'S')
                {
                    start = (x, y);
                }
            }
        }

        return new Grid(grid, width, height, start);
    }

    private static Dictionary<(int X, int Y), int> GetPath(Grid grid)
    {
        var path = new Dictionary<(int X, int Y), int>();

        var queue = new Queue<(int Cost, (int X, int Y))>();

        queue.Enqueue((0, grid.Start));

        var dirs = new[]
        {
            (-1, 0),  // left
            (1, 0),   // right
            (0, -1),  // up
            (0, 1),   // down
        };

        while (queue.TryDequeue(out var item))
        {
            var (cost, pos) = item;
            var x = pos.X;
            var y = pos.Y;

            if (!path.TryAdd((x, y), cost))
            {
                continue;
            }

            var current = grid.Cells[y][x];

            foreach (var (dx, dy) in dirs)
            {
                var x1 = x + dx;
                var y1 = y + dy;

                if (x1 < 0 || y1 < 0 || x1 >= grid.Width || y1 >= grid.Height)
                {
                    continue;
                }

                var next = grid.Cells[y1][x1];

                if (!Connects(current, dx, dy) || !Connects(next, -dx, -dy))
                {
                    continue;
                }

                queue.Enqueue((cost + 1, (x1, y1)));
            }
        }

        return path;
    }

    private static bool Connects(char pipe, int dx, int dy)
    {
        return pipe switch
        {
            '|' => (dx, dy) is (0, -1) or (0, 1),
            '-' => (dx, dy) is (-1, 0) or (1, 0),
            'L' => (dx, dy) is (0, -1) or (1, 0),
            'J' => (dx, dy) is (0, -1) or (-1, 0),
            '7' => (dx, dy) is (0, 1) or (-1, 0),
            'F' => (dx, dy) is (0, 1) or (1, 0),
            'S' => true,
            _ => false,
        };
    }
}
