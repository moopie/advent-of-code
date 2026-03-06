namespace main;

class Brick
{
    public int Id;
    public int X1;
    public int Y1;
    public int Z1;
    public int X2;
    public int Y2;
    public int Z2;

    public HashSet<int> Supports = new();
    public HashSet<int> SupportedBy = new();
}

public static class Day22
{
    public static int SolvePart1(string input)
    {
        var bricks = ParseInput(input);

        DropBricks(bricks);

        BuildSupports(bricks);

        return CountSafe(bricks);
    }

    public static int SolvePart2(string input)
    {
        var bricks = ParseInput(input);

        DropBricks(bricks);

        BuildSupports(bricks);

        int sum = 0;

        foreach (var brick in bricks)
        {
            sum += CountChainReaction(bricks, brick.Id);
        }

        return sum;
    }

    private static Brick[] ParseInput(string input)
    {
        var options = StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries;

        return input
            .Split("\n", options)
            .Select((line, i) =>
            {
                var parts = line.Split("~", options);

                var p1 = parts[0].Split(",", options);
                var p2 = parts[1].Split(",", options);

                return new Brick
                {
                    Id = i,
                    X1 = int.Parse(p1[0]),
                    Y1 = int.Parse(p1[1]),
                    Z1 = int.Parse(p1[2]),
                    X2 = int.Parse(p2[0]),
                    Y2 = int.Parse(p2[1]),
                    Z2 = int.Parse(p2[2])
                };
            })
            .ToArray();
    }

    private static void DropBricks(Brick[] bricks)
    {
        Array.Sort(bricks, (a, b) => a.Z1.CompareTo(b.Z1));

        for (int i = 0; i < bricks.Length; i++)
        {
            var brick = bricks[i];

            int maxZ = 1;

            for (int j = 0; j < i; j++)
            {
                var other = bricks[j];

                if (OverlapXY(brick, other))
                {
                    maxZ = Math.Max(maxZ, other.Z2 + 1);
                }
            }

            int delta = brick.Z1 - maxZ;

            brick.Z1 -= delta;
            brick.Z2 -= delta;
        }
    }

    private static bool OverlapXY(Brick a, Brick b)
    {
        return !(a.X2 < b.X1 ||
                 a.X1 > b.X2 ||
                 a.Y2 < b.Y1 ||
                 a.Y1 > b.Y2);
    }

    private static void BuildSupports(Brick[] bricks)
    {
        for (int i = 0; i < bricks.Length; i++)
        {
            for (int j = 0; j < bricks.Length; j++)
            {
                if (i == j)
                {
                    continue;
                }

                var a = bricks[i];
                var b = bricks[j];

                if (a.Z2 + 1 == b.Z1 && OverlapXY(a, b))
                {
                    a.Supports.Add(b.Id);
                    b.SupportedBy.Add(a.Id);
                }
            }
        }
    }

    private static int CountSafe(Brick[] bricks)
    {
        int count = 0;

        foreach (var brick in bricks)
        {
            bool safe = true;

            foreach (var supportedId in brick.Supports)
            {
                var supported = bricks.First(b => b.Id == supportedId);

                if (supported.SupportedBy.Count == 1)
                {
                    safe = false;
                    break;
                }
            }

            if (safe)
            {
                count++;
            }
        }

        return count;
    }

    private static int CountChainReaction(Brick[] bricks, int startId)
    {
        var fallen = new HashSet<int>();
        var queue = new Queue<int>();

        fallen.Add(startId);
        queue.Enqueue(startId);

        while (queue.Count > 0)
        {
            var id = queue.Dequeue();

            var brick = bricks.First(b => b.Id == id);

            foreach (var supportedId in brick.Supports)
            {
                if (fallen.Contains(supportedId))
                {
                    continue;
                }

                var supported = bricks.First(b => b.Id == supportedId);

                bool stillSupported = false;

                foreach (var supporter in supported.SupportedBy)
                {
                    if (!fallen.Contains(supporter))
                    {
                        stillSupported = true;
                        break;
                    }
                }

                if (!stillSupported)
                {
                    fallen.Add(supportedId);
                    queue.Enqueue(supportedId);
                }
            }
        }

        return fallen.Count - 1;
    }
}
