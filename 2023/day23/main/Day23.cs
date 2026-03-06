namespace main;

public static class Day23
{
    private static readonly (int dx, int dy)[] Directions =
    [
        (1,0),
        (-1,0),
        (0,1),
        (0,-1)
    ];

    public static int SolvePart1(string input)
    {
        var grid = ParseInput(input);

        var start = FindStart(grid);
        var end = FindEnd(grid);

        var visited = new HashSet<(int, int)>();

        return DFSPart1(start.Item1, start.Item2, end, grid, visited);
    }

    public static int SolvePart2(string input)
    {
        var grid = ParseInput(input);

        var start = FindStart(grid);
        var end = FindEnd(grid);

        var nodes = FindNodes(grid, start, end);

        var graph = BuildGraph(grid, nodes);

        var visited = new HashSet<(int, int)>();

        return DFSGraph(start, end, graph, visited);
    }

    private static int DFSPart1(
        int x,
        int y,
        (int, int) end,
        char[][] grid,
        HashSet<(int, int)> visited)
    {
        if ((x, y) == end)
        {
            return 0;
        }

        visited.Add((x, y));

        int best = int.MinValue;

        foreach (var (nx, ny) in GetNeighborsPart1(x, y, grid))
        {
            if (visited.Contains((nx, ny)))
            {
                continue;
            }

            var result = DFSPart1(nx, ny, end, grid, visited);

            if (result != int.MinValue)
            {
                best = Math.Max(best, result + 1);
            }
        }

        visited.Remove((x, y));

        return best;
    }

    private static IEnumerable<(int, int)> GetNeighborsPart1(int x, int y, char[][] grid)
    {
        var tile = grid[y][x];

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
            var nx = x + dx;
            var ny = y + dy;

            if (!Walkable(grid, nx, ny))
            {
                continue;
            }

            yield return (nx, ny);
        }
    }

    private static int DFSGraph(
        (int, int) node,
        (int, int) end,
        Dictionary<(int, int), List<((int, int), int)>> graph,
        HashSet<(int, int)> visited)
    {
        if (node == end)
        {
            return 0;
        }

        visited.Add(node);

        int best = int.MinValue;

        foreach (var (next, dist) in graph[node])
        {
            if (visited.Contains(next))
            {
                continue;
            }

            var result = DFSGraph(next, end, graph, visited);

            if (result != int.MinValue)
            {
                best = Math.Max(best, result + dist);
            }
        }

        visited.Remove(node);

        return best;
    }

    private static Dictionary<(int, int), List<((int, int), int)>> BuildGraph(
        char[][] grid,
        HashSet<(int, int)> nodes)
    {
        var graph = new Dictionary<(int, int), List<((int, int), int)>>();

        foreach (var node in nodes)
        {
            graph[node] = new List<((int, int), int)>();

            foreach (var (dx, dy) in Directions)
            {
                var x = node.Item1 + dx;
                var y = node.Item2 + dy;

                if (!Walkable(grid, x, y))
                {
                    continue;
                }

                var px = node.Item1;
                var py = node.Item2;

                var dist = 1;

                while (!nodes.Contains((x, y)))
                {
                    foreach (var (ndx, ndy) in Directions)
                    {
                        var nx = x + ndx;
                        var ny = y + ndy;

                        if (!Walkable(grid, nx, ny))
                        {
                            continue;
                        }

                        if (nx == px && ny == py)
                        {
                            continue;
                        }

                        px = x;
                        py = y;
                        x = nx;
                        y = ny;
                        dist++;

                        break;
                    }
                }

                graph[node].Add(((x, y), dist));
            }
        }

        return graph;
    }

    private static HashSet<(int, int)> FindNodes(
        char[][] grid,
        (int, int) start,
        (int, int) end)
    {
        var nodes = new HashSet<(int, int)>();

        for (var y = 0; y < grid.Length; y++)
        {
            for (var x = 0; x < grid[0].Length; x++)
            {
                if (!Walkable(grid, x, y))
                {
                    continue;
                }

                var neighbors = 0;

                foreach (var (dx, dy) in Directions)
                {
                    if (Walkable(grid, x + dx, y + dy))
                    {
                        neighbors++;
                    }
                }

                if (neighbors != 2 || (x, y) == start || (x, y) == end)
                {
                    nodes.Add((x, y));
                }
            }
        }

        return nodes;
    }

    private static bool Walkable(char[][] grid, int x, int y)
    {
        if (y < 0 || y >= grid.Length)
        {
            return false;
        }

        if (x < 0 || x >= grid[0].Length)
        {
            return false;
        }

        return grid[y][x] != '#';
    }

    private static (int, int) FindStart(char[][] grid)
    {
        for (var x = 0; x < grid[0].Length; x++)
        {
            if (grid[0][x] != '#')
            {
                return (x, 0);
            }
        }

        throw new Exception();
    }

    private static (int, int) FindEnd(char[][] grid)
    {
        var y = grid.Length - 1;

        for (var x = 0; x < grid[0].Length; x++)
        {
            if (grid[y][x] != '#')
            {
                return (x, y);
            }
        }

        throw new Exception();
    }

    private static char[][] ParseInput(string input)
    {
        return input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(x => x.ToCharArray())
            .ToArray();
    }
}
