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

        var visited = Walk(0, 0, Direction.East, grid);

        return visited
            .Select(v => (v.X, v.Y))
            .Distinct()
            .Count();
    }

    public static int SolvePart2(string input)
    {
        var grid = ParseInput(input);

        var max = 0;

        for (var y = 0; y < grid.Length; y++)
        {
            var first = Walk(0, y, Direction.East, grid);
            var second = Walk(grid[0].Length - 1, y, Direction.West, grid);

            var fistCost = first
                .Select(x => (x.X, x.Y))
                .Distinct()
                .Count();

            var secondCost = second
                .Select(x => (x.X, x.Y))
                .Distinct()
                .Count();

            var maxCost = Math.Max(fistCost, secondCost);

            if (maxCost > max)
            {
                max = maxCost;
            }
        }

        for (var x = 0; x < grid[0].Length; x++)
        {
            var first = Walk(x, 0, Direction.South, grid);
            var second = Walk(x, grid.Length - 1, Direction.North, grid);

            var fistCost = first
                .Select(x => (x.X, x.Y))
                .Distinct()
                .Count();

            var secondCost = second
                .Select(x => (x.X, x.Y))
                .Distinct()
                .Count();

            var maxCost = Math.Max(fistCost, secondCost);

            if (maxCost > max)
            {
                max = maxCost;
            }
        }

        return max;
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

    private static HashSet<(int X, int Y, Direction Dir)> Walk(int x, int y, Direction direction, char[][] grid)
    {
        var queue = new Queue<(int X, int Y, Direction Dir)>();
        var visited = new HashSet<(int X, int Y, Direction Dir)>();

        queue.Enqueue((x, y, direction));

        while (queue.Count > 0)
        {
            var (newX, newY, dir) = queue.Dequeue();

            if (newX < 0 || newY < 0 || newY >= grid.Length || newX >= grid[newY].Length)
            {
                continue;
            }

            if (!visited.Add((newX, newY, dir)))
            {
                continue;
            }

            var tile = grid[newY][newX];

            switch (tile)
            {
                case '.':
                    queue.Enqueue(Move(newX, newY, dir));
                    break;

                case '/':
                    queue.Enqueue(Move(newX, newY, BounceSlash(dir)));
                    break;

                case '\\':
                    queue.Enqueue(Move(newX, newY, BounceBackslash(dir)));
                    break;

                case '-':
                    if (dir == Direction.East || dir == Direction.West)
                        queue.Enqueue(Move(newX, newY, dir));
                    else
                    {
                        queue.Enqueue(Move(newX, newY, Direction.West));
                        queue.Enqueue(Move(newX, newY, Direction.East));
                    }
                    break;

                case '|':
                    if (dir == Direction.North || dir == Direction.South)
                        queue.Enqueue(Move(newX, newY, dir));
                    else
                    {
                        queue.Enqueue(Move(newX, newY, Direction.North));
                        queue.Enqueue(Move(newX, newY, Direction.South));
                    }
                    break;
            }
        }

        return visited;
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
