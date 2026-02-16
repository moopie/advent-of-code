using System.Globalization;

namespace main;

public static class Day13
{
    public static int SolvePart1(string input)
    {
        int total = 0;

        foreach (char[][] grid in Parse(input))
        {
            var h = grid.Length;
            var w = grid[0].Length;

            var rows = RowsToBinary(grid, h, w).ToList();
            var columns = ColumnsToBinary(grid, h, w).ToList();

            int horizontal = FindReflection(rows);
            if (horizontal > 0)
            {
                total += horizontal * 100;
                continue;
            }

            int vertical = FindReflection(columns);
            total += vertical;
        }

        return total;
    }

    public static int SolvePart2(string input)
    {
        int total = 0;

        foreach (var grid in Parse(input))
        {
            var h = grid.Length;
            var w = grid[0].Length;

            var rows = RowsToBinary(grid, h, w).ToList();
            var columns = ColumnsToBinary(grid, h, w).ToList();

            int horizontal = FindReflectionWithSmudge(rows);
            if (horizontal > 0)
            {
                total += horizontal * 100;
                continue;
            }

            int vertical = FindReflectionWithSmudge(columns);
            total += vertical;
        }

        return total;
    }

    private static IEnumerable<int> ColumnsToBinary(char[][] grid, int height, int width)
    {
        for (var x = 0; x < width; x++)
        {
            var num = "";

            for (var y = 0; y < height; y++)
            {
                var ch = grid[y][x];

                switch (ch)
                {
                    case '#':
                        num += '1';
                        break;
                    case '.':
                        num += '0';
                        break;
                }
            }

            yield return int.Parse(num, NumberStyles.BinaryNumber);
        }
    }

    private static IEnumerable<char[][]> Parse(string input)
    {
        var grids = input.Split("\n\n");

        foreach (var grid in grids)
        {
            yield return grid
                .Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
                .Select(l => l.ToCharArray())
                .ToArray();
        }
    }

    private static IEnumerable<int> RowsToBinary(char[][] grid, int height, int width)
    {
        for (var y = 0; y < height; y++)
        {
            var num = "";

            for (var x = 0; x < width; x++)
            {
                var ch = grid[y][x];

                if (ch == '#')
                {
                    num += '1';
                }
                else if (ch == '.')
                {
                    num += '0';
                }
            }

            yield return int.Parse(num, NumberStyles.BinaryNumber);
        }
    }

    private static int FindReflection(IReadOnlyList<int> values)
    {
        for (int split = 1; split < values.Count; split++)
        {
            int left = split - 1;
            int right = split;

            bool valid = true;

            while (left >= 0 && right < values.Count)
            {
                if (values[left] != values[right])
                {
                    valid = false;
                    break;
                }

                left--;
                right++;
            }

            if (valid)
                return split; // number of rows/cols before mirror
        }

        return 0;
    }

    private static int FindReflectionWithSmudge(IReadOnlyList<int> values)
    {
        for (int split = 1; split < values.Count; split++)
        {
            int left = split - 1;
            int right = split;

            int totalDifferences = 0;

            while (left >= 0 && right < values.Count)
            {
                int diff = values[left] ^ values[right];

                if (diff != 0)
                {
                    // count how many bits differ
                    totalDifferences += BitCount(diff);

                    if (totalDifferences > 1)
                    {
                        break;
                    }
                }

                left--;
                right++;
            }

            if (totalDifferences == 1)
            {
                return split;
            }
        }

        return 0;
    }

    private static int BitCount(int n)
    {
        int count = 0;
        while (n != 0)
        {
            n &= (n - 1);
            count++;
        }
        return count;
    }
}
