Console.WriteLine("AOC 2023 day 11!");

var input = File.ReadAllText("input.txt");

Console.WriteLine($"Part 1: {Day11.SolvePart1(input)}");

public record Space(List<(int X, int Y)> Galaxies, int Height, int Width, HashSet<int> EmptyRows, HashSet<int> EmptyCols);

public static class Day11
{
    public static long SolvePart1(string input)
    {
        var space = ParseInput(input);

        long sum = 0;

        for (var i = 0; i < space.Galaxies.Count; i++)
        {
            var (x1, y1) = space.Galaxies[i];

            for (var j = i + 1; j < space.Galaxies.Count; j++)
            {
                var (x2, y2) = space.Galaxies[j];

                int rMin = Math.Min(x1, x2);
                int rMax = Math.Max(x1, x2);
                var cMin = Math.Min(y1, y2);
                var cMax = Math.Max(y1, y2);

                long dist = (rMax - rMin) + (cMax - cMin);

                for (int r = rMin + 1; r < rMax; r++)
                {
                    if (space.EmptyRows.Contains(r))
                    {
                        dist += 1;
                    }
                }

                for (int c = cMin + 1; c < cMax; c++)
                {
                    if (space.EmptyCols.Contains(c))
                    {
                        dist += 1;
                    }
                }

                sum += dist;
            }
        }

        return sum;
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
        {
            for (int x = 0; x < w; x++)
            {
                if (lines[y][x] == '#')
                {
                    galaxies.Add((x, y));
                }
            }
        }

        var rowHasGalaxy = new bool[w];
        var colHasGalaxy = new bool[h];

        foreach (var (x, y) in galaxies)
        {
            rowHasGalaxy[x] = true;
            colHasGalaxy[y] = true;
        }

        var emptyRows = new HashSet<int>(Enumerable.Range(0, w).Where(r => !rowHasGalaxy[r]));
        var emptyCols = new HashSet<int>(Enumerable.Range(0, h).Where(c => !colHasGalaxy[c]));

        return new Space(galaxies, h, w, emptyRows, emptyCols);
    }
}
