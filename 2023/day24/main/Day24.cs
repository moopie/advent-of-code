namespace main;

record Hailstone(long X, long Y, long Z, long Vx, long Vy, long Vz);

public static class Day24
{
    public static long SolvePart1(string input, long lowBound, long highBound)
    {
        var count = 0L;
        var hailstones = ParseInput(input);
        for (var i = 0; i < hailstones.Length - 1; i++)
        {
            for (var j = i + 1; j < hailstones.Length; j++)
            {
                if (IntersectsInTestArea(hailstones[i], hailstones[j], lowBound, highBound))
                {
                    count++;
                }
            }
        }
        return count;
    }

    private static Hailstone[] ParseInput(string input)
    {
        var options = StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries;

        return input
            .Split("\n", options)
            .Select(line =>
            {
                var parts = line.Split("@", options);
                var p1 = parts[0]
                    .Split(",", options)
                    .Select(long.Parse)
                    .ToArray();
                var p2 = parts[1]
                    .Split(",", options)
                    .Select(long.Parse)
                    .ToArray();

                return new Hailstone(p1[0], p1[1], p1[2], p2[0], p2[1], p2[2]);
            })
            .ToArray();
    }

    private static bool IntersectsInTestArea(Hailstone a, Hailstone b, double minBound, double maxBound)
    {
        // Solve:
        // a.X + a.Vx * ta = b.X + b.Vx * tb
        // a.Y + a.Vy * ta = b.Y + b.Vy * tb
        //
        // Rearranged into 2x2 system:
        // a.Vx * ta - b.Vx * tb = b.X - a.X
        // a.Vy * ta - b.Vy * tb = b.Y - a.Y

        double dx = b.X - a.X;
        double dy = b.Y - a.Y;

        double det = (double)a.Vx * (-b.Vy) - (double)a.Vy * (-b.Vx);

        // Parallel or same slope in 2D => no single intersection to count
        if (Math.Abs(det) < 1e-12)
        {
            return false;
        }

        double ta = (dx * (-b.Vy) - dy * (-b.Vx)) / det;
        double tb = ((double)a.Vx * dy - (double)a.Vy * dx) / det;

        // Must happen in the future for both hailstones
        if (ta < 0 || tb < 0)
        {
            return false;
        }

        double x = a.X + a.Vx * ta;
        double y = a.Y + a.Vy * ta;

        return x >= minBound
               && x <= maxBound
               && y >= minBound
               && y <= maxBound;
    }
}
