Console.WriteLine("AOC 2023 day 8!");

var input = File.ReadAllText("input.txt");

var result = Day8.SolvePart1(input);

Console.WriteLine($"Part 1 solution: {result}");

public static class Day8
{
    private const string StartingPoint = "AAA";

    public static int SolvePart1(string input)
    {
        var (path, dict) = ParseInput(input);
        var queue = new Queue<char>(path);

        var moves = 0;
        var key = StartingPoint;
        while (key != "ZZZ")
        {
            var (left, right) = dict[key];
            moves++;

            var move = queue.Dequeue();

            switch (move)
            {
                case 'L':
                    key = left;
                    break;
                case 'R':
                    key = right;
                    break;
                default:
                    throw new Exception("Invalid input");
            }

            queue.Enqueue(move);
        }

        return moves;
    }

    private static (string, Dictionary<string, (string, string)>) ParseInput(string input)
    {
        var path = string.Empty;
        var dict = new Dictionary<string, (string, string)>();
        foreach (var line in input.Split("\n", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            if (!line.Contains('='))
            {
                path = line;
            }
            else
            {
                var eq = line.Split('=', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);

                var key = eq[0];
                var values = new List<string>();

                foreach (var value in eq[1].Split(','))
                {
                    var val = value.Trim('(', ')').Trim();
                    values.Add(val);
                }

                if (values.Count() != 2) throw new Exception("Invalid input");

                dict.Add(key, (values[0], values[1]));
            }

        return (path, dict);
    }
}