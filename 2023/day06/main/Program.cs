Console.WriteLine("AOC 2023 day 6!");

var input = File.ReadAllText("input.txt");

var part1 = Day6.SolvePart1(input);

Console.WriteLine($"Part 1 solution: {part1}");

internal record Race(int Id, int Time, int Distance);

public static class Day6
{
    public static int SolvePart1(string input)
    {
        var races = ParseInput(input);
        var sum = 1;
        foreach (var race in races)
        {
            var rt = 0;
            for (var i = 1; i < race.Time; i++)
            {
                var t = i * (race.Time - i);

                if (t > race.Distance) rt++;
            }

            sum *= rt;
        }

        return sum;
    }

    private static List<Race> ParseInput(string input)
    {
        var dict = new Dictionary<string, IList<int>>();
        foreach (var line in input.Split("\n", StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries))
        {
            var raceName = string.Empty;
            var nums = new List<int>();
            foreach (
                var part in line.Split(
                    " ",
                    StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries
                )
            )
            {
                if (part.EndsWith(':')) raceName = part.TrimEnd(":").ToString().ToLower();

                if (int.TryParse(part, out var result)) nums.Add(result);
            }

            dict[raceName] = nums;
        }

        var n = dict.First().Value.Count();
        var res = new List<Race>();

        for (var i = 0; i < n; i++)
        {
            var buf = new List<int>();
            for (var j = 0; j < 2; j++)
            {
                var num = dict.ElementAt(j).Value.ElementAt(i);
                buf.Add(num);
            }

            res.Add(new Race(i, buf[0], buf[1]));
        }

        return res;
    }
}