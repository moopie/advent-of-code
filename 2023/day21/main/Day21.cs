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
