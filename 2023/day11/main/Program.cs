Console.WriteLine("AOC 2023 day 11!");

var input = File.ReadAllText("input.txt");

Console.WriteLine($"Part 1: {Day11.SolvePart1(input)}");
Console.WriteLine($"Part 2: {Day11.SolvePart2(input, 1000000)}");

public record Space(List<(int X, int Y)> Galaxies, int Height, int Width, HashSet<int> EmptyRows, HashSet<int> EmptyCols);

public static class Day11
{
    public static long SolvePart1(string input)
    {
        var space = ParseInput(input);

        return CalculateDistance(space);
    }

    public static long SolvePart2(string input, int scale)
    {
        var space = ParseInput(input);

        return CalculateDistance(space, scale);
    }

    private static Space ParseInput(string input)
    {
        var lines = input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries)
            .Select(l => l.Trim())
            .Where(l => !string.IsNullOrWhiteSpace(l))
            .ToArray();

        int h = lines.Length;
        int w = lines[0].Length;

        var galaxies = new List<(int X, int Y)>();

        for (int y = 0; y < h; y++)
            for (int x = 0; x < w; x++)
                if (lines[y][x] == '#')
                    galaxies.Add((x, y));

        var rowHasGalaxy = new bool[h];
        var colHasGalaxy = new bool[w];

        foreach (var (x, y) in galaxies)
        {
            rowHasGalaxy[y] = true;
            colHasGalaxy[x] = true;
        }

        var emptyRows = new HashSet<int>(
            Enumerable.Range(0, h).Where(r => !rowHasGalaxy[r])
        );

        var emptyCols = new HashSet<int>(
            Enumerable.Range(0, w).Where(c => !colHasGalaxy[c])
        );

        return new Space(galaxies, h, w, emptyRows, emptyCols);
    }

    private static long CalculateDistance(Space space, int scale = 2)
    {
        long sum = 0;

        for (int i = 0; i < space.Galaxies.Count; i++)
        {
            var (x1, y1) = space.Galaxies[i];

            for (int j = i + 1; j < space.Galaxies.Count; j++)
            {
                var (x2, y2) = space.Galaxies[j];

                int xMin = Math.Min(x1, x2);
                int xMax = Math.Max(x1, x2);
                int yMin = Math.Min(y1, y2);
                int yMax = Math.Max(y1, y2);

                long dist = (xMax - xMin) + (yMax - yMin);

                for (int x = xMin + 1; x < xMax; x++)
                    if (space.EmptyCols.Contains(x))
                        dist += scale - 1;

                for (int y = yMin + 1; y < yMax; y++)
                    if (space.EmptyRows.Contains(y))
                        dist += scale - 1;

                sum += dist;
            }
        }

        return sum;
    }
}
