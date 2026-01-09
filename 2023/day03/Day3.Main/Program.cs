namespace Day3.Main;

public class Program
{
    public static void Main()
    {
        Console.WriteLine("AOC 2023 day 3!");

        var input = File.ReadAllText("input.txt");

        var sum = Day03.SolvePart1(input);

        Console.WriteLine($"Part 1 solution: {sum}");

        sum = Day03.SolvePart2(input);

        Console.WriteLine("Part 2 solution: {0}", sum);
    }
}

public static class Day03
{
    private static readonly (int, int)[] Ranges =
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ];

    private static List<int> ParseSchematic(char[][] engine)
    {
        var stack = new Stack<(int, bool)>();
        for (var i = 0; i < engine.Length; i++)
        {
            var innerStack = new Stack<(int, bool)>();
            for (var j = 0; j < engine[i].Length; j++)
            {
                var ch = engine[i][j];

                if (!char.IsDigit(ch))
                {
                    AddNumber(innerStack, stack);
                    continue;
                }

                var hasSymbol = false;

                foreach (var (dx, dy) in Ranges)
                {
                    var di = i - dy;
                    var dj = j - dx;

                    if (di < 0 || dj < 0 || di >= engine.Length || dj >= engine[di].Length)
                        continue;

                    var dch = engine[di][dj];

                    if (char.IsDigit(dch) || dch == '.')
                        continue;

                    hasSymbol = true;
                }

                innerStack.Push((ch - '0', hasSymbol));
            }

            AddNumber(innerStack, stack);
        }

        var ret = new List<int>();
        foreach (var (n, s) in stack)
            if (s)
                ret.Add(n);

        return ret;
    }

    private static void AddNumber(Stack<(int, bool)> input, Stack<(int, bool)> output)
    {
        if (input.Count == 0)
            return;

        var num = 0;
        var hasSymbol = false;
        foreach (var (n, s) in input.Reverse())
        {
            num = num * 10 + n;
            if (s) hasSymbol = true;
        }

        output.Push((num, hasSymbol));
        input.Clear();
    }

    private static int GetNumber(char[][] engine, int i, int j)
    {
        var current = engine[i][j];

        while (j > 0 && char.IsDigit(current))
        {
            j--;
            current = engine[i][j];
        }

        if (!char.IsDigit(current)) j += 1;

        var buf = new Stack<int>();
        while (j < engine[i].Length && char.IsDigit(engine[i][j]))
        {
            var val = engine[i][j] - '0';
            buf.Push(val);
            j += 1;
        }

        var num = 0;
        foreach (var d in buf.Reverse()) num = num * 10 + d;

        return num;
    }

    private static int ParseGears(char[][] engine)
    {
        var nums = new List<int>();

        for (var i = 0; i < engine.Length; i++)
        for (var j = 0; j < engine[i].Length; j++)
        {
            var ch = engine[i][j];
            if (ch != '*')
                continue;

            var set = new HashSet<int>();
            foreach (var (x, y) in Ranges)
            {
                var di = i - x;
                var dj = j - y;

                if (di < 0 || dj < 0 || di >= engine.Length || dj >= engine[di].Length)
                    continue;

                var dch = engine[di][dj];

                if (!char.IsDigit(dch))
                    continue;

                var num = GetNumber(engine, di, dj);
                set.Add(num);
            }

            if (set.Count != 2) continue;

            var sum = 0;
            foreach (var num in set)
                if (sum == 0)
                    sum = num;
                else
                    sum *= num;
            nums.Add(sum);
        }

        return nums.Sum();
    }

    public static int SolvePart1(string input)
    {
        var lines = input.Trim().Split("\n");
        var map = lines.Select(line => line.Trim().ToCharArray()).ToArray();

        var results = ParseSchematic(map);

        return results.Sum();
    }

    public static int SolvePart2(string input)
    {
        var lines = input.Split("\n");
        var map = lines.Select(line => line.ToCharArray()).ToArray();

        var results = ParseGears(map);

        return results;
    }
}