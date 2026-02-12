Console.WriteLine("AOC 2023 day 10!");

var input = File.ReadAllText("input.txt");

Console.WriteLine($"Part 1: {Day10.SolvePart1(input)}");

public record Grid(char[][] Cells, int Width, int Height, (int X, int Y) Start);

public static class Day10
{
    public static int SolvePart1(string input)
    {
        var grid = ParseInput(input);
        var queue = new Queue<(int Cost, (int X, int Y))>();
        var visited = new HashSet<(int X, int Y)>();
        int maxCost = 0;

        queue.Enqueue((0, grid.Start));

        var dirs = new (int, int)[]
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

            if (!visited.Add((x, y)))
            {
                continue;
            }
            
            maxCost = Math.Max(maxCost, cost);

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
        return maxCost;
    }

    private static Grid ParseInput(string input)
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
