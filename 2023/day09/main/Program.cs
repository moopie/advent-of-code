Console.WriteLine("AOC 2023 day 09!");

var input = File.ReadAllText("input.txt");

Console.WriteLine($"Part 1: {Day09.SolvePart1(input)}");
Console.WriteLine($"Part 2: {Day09.SolvePart2(input)}");

public static class Day09
{
    public static long SolvePart1(string input)
    {
        var histories = ParseInput(input);

        return histories.Select(ExtrapolateNext).Sum();
    }

    public static long SolvePart2(string input)
    {
        var histories = ParseInput(input);

        return histories.Select(ExtrapolatePrevious).Sum();
    }

    private static List<List<long>> ParseInput(string input)
    {
        return input
            .Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(x => x
                    .Split(" ", StringSplitOptions.RemoveEmptyEntries)
                    .Select(long.Parse)
                    .ToList())
            .ToList();
    }

    private static List<List<long>> BuildDifferenceLevels(List<long> history)
    {
        var levels = new List<List<long>>();
        var current = new List<long>(history);

        while (true)
        {
            levels.Add(current);

            if (current.All(x => x == 0))
            {
                break;
            }

            var next = new List<long>(current.Count - 1);

            for (var i = 0; i < current.Count - 1; i++)
            {
                next.Add(current[i + 1] - current[i]);
            }

            current = next;
        }

        return levels;
    }

    private static long ExtrapolateNext(List<long> history)
    {
        var levels = BuildDifferenceLevels(history);

        long next = 0;

        for (var i = 0; i < levels.Count - 1; i++)
        {
            var level = levels[i];
            var last = level[^1];
            next = last + next;
        }

        return next;
    }

    private static long ExtrapolatePrevious(List<long> history)
    {
        var levels = BuildDifferenceLevels(history);

        long prev = 0;

        for (var i = levels.Count - 1; i >= 0; i--)
        {
            var level = levels[i];
            var first = level[0];
            prev = first - prev;
        }
        return prev;
    }
}
