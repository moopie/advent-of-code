namespace main;

public static class Day23
{
    private static readonly (int dx, int dy)[] Directions =
    {
        (0, 1),   // south
        (0, -1),  // north
        (1, 0),   // east
        (-1, 0)   // west
    };

    public static int SolvePart1(string input)
    {
        var grid = ParseInput(input);

        var start = FindStart(grid);
        var end = FindEnd(grid);

        var visited = new HashSet<(int x, int y)>();

        return DFS(start.x, start.y, end, grid, visited);
    }

    private static int DFS(int x, int y, (int, int) end, char[][] grid, HashSet<(int, int)> visited)
    {
        if ((x, y) == end)
        {
            return 0;
        }

        visited.Add((x, y));

        int best = int.MinValue;

        foreach (var (nx, ny) in GetNeighbors(x, y, grid))
        {
            if (visited.Contains((nx, ny)))
            {
                continue;
            }

            int result = DFS(nx, ny, end, grid, visited);

            if (result != int.MinValue)
            {
                best = Math.Max(best, result + 1);
            }
        }

        visited.Remove((x, y));

        return best;
    }

    private static IEnumerable<(int x, int y)> GetNeighbors(int x, int y, char[][] grid)
    {
        char tile = grid[y][x];

        if (tile == '^')
        {
            yield return (x, y - 1);
            yield break;
        }

        if (tile == 'v')
        {
            yield return (x, y + 1);
            yield break;
        }

        if (tile == '<')
        {
            yield return (x - 1, y);
            yield break;
        }

        if (tile == '>')
        {
            yield return (x + 1, y);
            yield break;
        }

        foreach (var (dx, dy) in Directions)
        {
            int nx = x + dx;
            int ny = y + dy;

            if (ny < 0 || ny >= grid.Length)
            {
                continue;
            }

            if (nx < 0 || nx >= grid[0].Length)
            {
                continue;
            }

            if (grid[ny][nx] == '#')
            {
                continue;
            }

            yield return (nx, ny);
        }
    }

    private static (int x, int y) FindStart(char[][] grid)
    {
        for (int x = 0; x < grid[0].Length; x++)
        {
            if (grid[0][x] == '.')
            {
                return (x, 0);
            }
        }

        throw new Exception("Start not found");
    }

    private static (int x, int y) FindEnd(char[][] grid)
    {
        int last = grid.Length - 1;

        for (int x = 0; x < grid[0].Length; x++)
        {
            if (grid[last][x] == '.')
            {
                return (x, last);
            }
        }

        throw new Exception("End not found");
    }

    private static char[][] ParseInput(string input)
    {
        return input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(x => x.ToCharArray())
            .ToArray();
    }
}
