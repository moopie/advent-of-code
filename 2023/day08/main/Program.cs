Console.WriteLine("AOC 2023 day 8!");

var input = File.ReadAllText("input.txt");

var part1 = Day8.SolvePart1(input);

Console.WriteLine($"Part 1 solution: {part1}");

var part2 = Day8.SolvePart2(input);

Console.WriteLine($"Part 2 solution: {part2}");

public static class Day8
{
    public static int SolvePart1(string input)
    {
        var cond = new Func<string, bool>(val => val == "ZZZ");
        var (path, dict) = ParseInput(input);
        var queue = new Queue<char>(path);

        var moves = 0;
        var key = "AAA";
        while (!cond(key))
        {
            var (left, right) = dict[key];
            moves++;

            var move = queue.Dequeue();

            key = move switch
            {
                'L' => left,
                'R' => right,
                _ => throw new Exception("Invalid input")
            };

            queue.Enqueue(move);
        }

        return moves;
    }

    private static (
        string,
        Dictionary<string, (string, string)>
        ) ParseInput(string input)
    {
        var path = string.Empty;
        var dict = new Dictionary<string, (string, string)>();
        foreach (
            var line in input.Split(
                "\n",
                StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries
            )
        )
            if (!line.Contains('='))
            {
                path = line;
            }
            else
            {
                var eq = line.Split(
                    '=',
                    StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries
                );

                var key = eq[0];
                var values = new List<string>();

                foreach (var value in eq[1].Split(','))
                {
                    var val = value.Trim('(', ')').Trim();
                    values.Add(val);
                }

                if (values.Count != 2)
                    throw new Exception("Invalid input");

                dict.Add(key, (values[0], values[1]));
            }

        return (path, dict);
    }

    public static long SolvePart2(string input)
    {
        var (path, dict) = ParseInput(input);

        var starts = dict.Keys.Where(k => k.EndsWith('A')).ToList();
        var cycles = new List<long>();

        foreach (var start in starts)
        {
            var current = start;
            long steps = 0;
            var pathIndex = 0;

            while (!current.EndsWith('Z'))
            {
                var move = path[pathIndex];
                var (left, right) = dict[current];

                current = move == 'L' ? left : right;

                steps++;
                pathIndex = (pathIndex + 1) % path.Length;
            }

            cycles.Add(steps);
        }

        return LcmAll(cycles);
    }

    private static long Gcd(long a, long b)
    {
        while (b != 0)
            (a, b) = (b, a % b);
        return a;
    }

    private static long Lcm(long a, long b)
    {
        return a / Gcd(a, b) * b;
    }

    private static long LcmAll(IEnumerable<long> nums)
    {
        return nums.Aggregate(Lcm);
    }
}