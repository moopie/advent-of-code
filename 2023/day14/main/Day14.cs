namespace main;

public static class Day14
{
    public static int SolvePart1(string input)
    {
        var grid = ParseInput(input);
        var h = grid.Length;
        var w = grid[0].Length;
        MoveRocks(grid, h, w);

        var sum = 0;
        for (var y = 0; y < h; y++)
        {
            for (var x = 0; x < w; x++)
            {
                if (grid[y][x] == 'O')
                {
                    sum += (h - y);
                }
            }
        }

        return sum;
    }

    private static char[][] ParseInput(string input)
    {
        return input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(line => line.ToCharArray())
            .ToArray();
    }

    private static void MoveRocks(char[][] grid, int height, int width)
    {
        for (int x = 0; x < width; x++)
        {
            int targetRow = 0;

            for (int y = 0; y < height; y++)
            {
                if (grid[y][x] == '#')
                {
                    targetRow = y + 1;
                }
                else if (grid[y][x] == 'O')
                {
                    if (y != targetRow)
                    {
                        grid[targetRow][x] = 'O';
                        grid[y][x] = '.';
                    }
                    targetRow++;
                }
            }
        }
    }
}
