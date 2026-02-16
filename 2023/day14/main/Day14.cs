namespace main;

enum Dir
{
    North,
    West,
    South,
    East
};

public static class Day14
{
    public static int SolvePart1(string input)
    {
        var grid = ParseInput(input);
        var h = grid.Length;
        var w = grid[0].Length;

        Tilt(grid, h, w, Dir.North);

        return CalculateLoad(grid, h, w);
    }

    public static int SolvePart2(string input, int totalCycles)
    {
        var grid = ParseInput(input);
        int h = grid.Length;
        int w = grid[0].Length;

        var seen = new Dictionary<string, int>();

        // Store initial state as step 0
        seen[Serialize(grid)] = 0;

        int step = 0;

        while (step < totalCycles)
        {
            step++;
            DoCycle(grid, h, w);

            string key = Serialize(grid);

            if (seen.TryGetValue(key, out int firstSeen))
            {
                int loopLength = step - firstSeen;

                int remaining = (totalCycles - step) % loopLength;

                for (int i = 0; i < remaining; i++)
                    DoCycle(grid, h, w);

                break;
            }

            seen[key] = step;
        }

        return CalculateLoad(grid, h, w);
    }

    private static char[][] ParseInput(string input)
    {
        return input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(line => line.ToCharArray())
            .ToArray();
    }

    private static void Tilt(char[][] grid, int h, int w, Dir dir)
    {
        switch (dir)
        {
            case Dir.North:
                for (int x = 0; x < w; x++)
                    CollapseColumn(grid, h, x, forward: true);
                break;

            case Dir.South:
                for (int x = 0; x < w; x++)
                    CollapseColumn(grid, h, x, forward: false);
                break;

            case Dir.West:
                for (int y = 0; y < h; y++)
                    CollapseRow(grid, w, y, forward: true);
                break;

            case Dir.East:
                for (int y = 0; y < h; y++)
                    CollapseRow(grid, w, y, forward: false);
                break;
        }
    }

    private static void CollapseColumn(char[][] grid, int h, int x, bool forward)
    {
        int start = forward ? 0 : h - 1;
        int end = forward ? h : -1;
        int step = forward ? 1 : -1;

        int write = start;

        for (int y = start; y != end; y += step)
        {
            if (grid[y][x] == '#')
            {
                write = y + step;
            }
            else if (grid[y][x] == 'O')
            {
                grid[y][x] = '.';
                grid[write][x] = 'O';
                write += step;
            }
        }
    }

    private static void CollapseRow(char[][] grid, int w, int y, bool forward)
    {
        int start = forward ? 0 : w - 1;
        int end = forward ? w : -1;
        int step = forward ? 1 : -1;

        int write = start;

        for (int x = start; x != end; x += step)
        {
            if (grid[y][x] == '#')
            {
                write = x + step;
            }
            else if (grid[y][x] == 'O')
            {
                grid[y][x] = '.';
                grid[y][write] = 'O';
                write += step;
            }
        }
    }

    private static void DoCycle(char[][] grid, int h, int w)
    {
        Tilt(grid, h, w, Dir.North);
        Tilt(grid, h, w, Dir.West);
        Tilt(grid, h, w, Dir.South);
        Tilt(grid, h, w, Dir.East);
    }

    private static int CalculateLoad(char[][] grid, int h, int w)
    {
        int sum = 0;

        for (int y = 0; y < h; y++)
        {
            for (int x = 0; x < w; x++)
            {
                if (grid[y][x] == 'O')
                    sum += (h - y);
            }
        }

        return sum;
    }

    private static string Serialize(char[][] grid)
    {
        int h = grid.Length;
        int w = grid[0].Length;

        var buffer = new char[h * w];
        int k = 0;

        for (int y = 0; y < h; y++)
        {
            Array.Copy(grid[y], 0, buffer, k, w);
            k += w;
        }

        return new string(buffer);
    }
}
