namespace main;

enum Direction
{
    North,
    South,
    West,
    East
}

public static class Day16
{
    public static int SolvePart1(string input)
    {
        var grid = ParseInput(input);

        var queue = new Queue<(int X, int Y, Direction Dir)>();
        var visited = new HashSet<(int X, int Y, Direction Dir)>();

        queue.Enqueue((0, 0, Direction.East));

        while (queue.Count > 0)
        {
            var (x, y, dir) = queue.Dequeue();

            if (x < 0 || y < 0 || y >= grid.Length || x >= grid[y].Length)
            {
                continue;
            }

            if (!visited.Add((x, y, dir)))
            {
                continue;
            }

            var tile = grid[y][x];

            switch (tile)
            {
                case '.':
                    queue.Enqueue(Move(x, y, dir));
                    break;

                case '/':
                    queue.Enqueue(Move(x, y, BounceSlash(dir)));
                    break;

                case '\\':
                    queue.Enqueue(Move(x, y, BounceBackslash(dir)));
                    break;

                case '-':
                    if (dir == Direction.East || dir == Direction.West)
                        queue.Enqueue(Move(x, y, dir));
                    else
                    {
                        queue.Enqueue(Move(x, y, Direction.West));
                        queue.Enqueue(Move(x, y, Direction.East));
                    }
                    break;

                case '|':
                    if (dir == Direction.North || dir == Direction.South)
                        queue.Enqueue(Move(x, y, dir));
                    else
                    {
                        queue.Enqueue(Move(x, y, Direction.North));
                        queue.Enqueue(Move(x, y, Direction.South));
                    }
                    break;
            }
        }

        return visited
            .Select(v => (v.X, v.Y))
            .Distinct()
            .Count();
    }

    private static Direction BounceSlash(Direction dir) => dir switch
    {
        Direction.North => Direction.East,
        Direction.South => Direction.West,
        Direction.West => Direction.South,
        Direction.East => Direction.North,
        _ => throw new Exception()
    };

    private static Direction BounceBackslash(Direction dir) => dir switch
    {
        Direction.North => Direction.West,
        Direction.South => Direction.East,
        Direction.West => Direction.North,
        Direction.East => Direction.South,
        _ => throw new Exception()
    };

    private static (int X, int Y, Direction Dir) Move(int x, int y, Direction dir)
    {
        var (nx, ny) = NextDirection(x, y, dir);
        return (nx, ny, dir);
    }

    private static (int X, int Y) NextDirection(int x, int y, Direction direction)
    {
        return direction switch
        {
            Direction.North => (x, y - 1),
            Direction.South => (x, y + 1),
            Direction.West => (x - 1, y),
            Direction.East => (x + 1, y),
            _ => throw new Exception(),
        };
    }

    private static char[][] ParseInput(string input)
    {
        return input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Where(l => !string.IsNullOrEmpty(l))
            .Select(l => l.ToCharArray())
            .ToArray();
    }
}
