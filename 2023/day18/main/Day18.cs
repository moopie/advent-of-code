namespace main;

record GridMvmnt(string Direction, int Value, string Color);

public static class Day18
{
    public static long SolvePart1(string input)
    {
        var moves = ParseInput(input);

        // Walk the path, collect vertices, and track perimeter length
        var points = new List<(long X, long Y)>();
        long x = 0;
        long y = 0;
        long perimeter = 0;

        points.Add((x, y));

        foreach (var move in moves)
        {
            (int dx, int dy) = move.Direction switch
            {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, -1),
                "D" => (0, 1),
                _ => throw new ArgumentOutOfRangeException(nameof(move.Direction), move.Direction, "Unknown direction")
            };

            x += dx * move.Value;
            y += dy * move.Value;

            points.Add((x, y));
            perimeter += move.Value;
        }

        // Shoelace formula to compute 2 * area
        long area2 = 0;
        for (int i = 0; i < points.Count; i++)
        {
            var (x1, y1) = points[i];
            var (x2, y2) = points[(i + 1) % points.Count];

            area2 += x1 * y2 - x2 * y1;
        }

        area2 = Math.Abs(area2);

        // Using Pick's theorem in the rearranged form:
        // answer = (area2 + perimeter) / 2 + 1
        long answer = (area2 + perimeter) / 2 + 1;

        return answer;
    }

    private static GridMvmnt[] ParseInput(string input)
    {
        return input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Where(l => !string.IsNullOrWhiteSpace(l))
            .Select(l =>
            {
                var parts = l.Split(" ", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);

                // parts[2] is like "(#70c710)" – we keep it as-is for now (used in part 2)
                return new GridMvmnt(parts[0], int.Parse(parts[1]), parts[2]);
            })
            .ToArray();
    }
}
