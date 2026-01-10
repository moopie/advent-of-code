Console.WriteLine("AOC 2023 day 4!");

var input = File.ReadAllLines("input.txt");

var part1Solution = Day4.SolvePart1(input);

Console.WriteLine("Part 1 solution: {0}", part1Solution);

var part2Solution = Day4.SolvePart2(input);

Console.WriteLine("Part 2 solution: {0}", part2Solution);

public static class Day4
{
    public static int SolvePart1(string[] lines)
    {
        var sum = 0;
        foreach (var line in lines)
        {
            var (id, win, have) = ParseLine(line);
            var score = 0;

            foreach (var num in have)
                if (win.Contains(num))
                {
                    if (score == 0)
                        score++;
                    else
                        score *= 2;
                }

            sum += score;
        }

        return sum;
    }

    public static int SolvePart2(string[] lines)
    {
        var n = lines.Length;

        var matches = new int[n];
        var copies = new int[n];

        for (var i = 0; i < n; i++)
            copies[i] = 1;

        for (var i = 0; i < n; i++)
        {
            var (_, win, have) = ParseLine(lines[i]);
            matches[i] = have.Count(win.Contains);
        }

        for (var i = 0; i < n; i++)
        for (var j = 1; j <= matches[i]; j++)
            if (i + j < n)
                copies[i + j] += copies[i];

        return copies.Sum();
    }

    private static (int, List<int>, List<int>) ParseLine(string line)
    {
        var parts = line.Split(' ');

        var id = 0;
        var state = "win";
        var win = new List<int>();
        var have = new List<int>();

        foreach (var part in parts)
        {
            if (part.Length == 0)
                continue;
            if (part == "Card")
                continue;
            if (part.EndsWith(":"))
            {
                id = int.Parse(part.Substring(0, part.Length - 1));
                continue;
            }

            if (part == "|")
            {
                state = "have";
                continue;
            }

            var num = int.Parse(part);

            switch (state)
            {
                case "win":
                    win.Add(num);
                    break;
                case "have":
                    have.Add(num);
                    break;
                default:
                    throw new Exception();
            }
        }

        return (id, win, have);
    }
}