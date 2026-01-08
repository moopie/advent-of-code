namespace Day03.Main;

public class Program
{
    public static void Main()
    {
        Console.WriteLine("AOC 2023 day 3!");

        var input = File.ReadAllText("input.txt");

        var sum = Day03.SolvePart1(input);

        Console.WriteLine($"Part 1 solution: {sum}");
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
        (1, 1),
    ];

    public static IEnumerable<int> ParseSchematic(char[][] engine)
    {
        var numbers = new Stack<(int, bool)>();
        for (var i = 0; i < engine.Length; i++)
        {
            var stack = new Stack<(int, bool)>();
            for (var j = 0; j < engine[i].Length; j++)
            {
                var ch = engine[i][j];

                if (!char.IsDigit(ch))
                {
                    AddNumber(stack, numbers);
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

                stack.Push((ch - '0', hasSymbol));
            }

            AddNumber(stack, numbers);
        }

        var ret = new List<int>();
        foreach (var (n, s) in numbers)
        {
            if (s)
            {
                ret.Add(n);
            }
        }

        return ret;
    }

    private static void AddNumber(Stack<(int, bool)> input, Stack<(int, bool)> output)
    {
        if (input.Count == 0)
            return;

        var num = 0;
        var symb = false;
        foreach (var (n, s) in input.Reverse())
        {
            num = num * 10 + n;
            if (s == true)
            {
                symb = true;
            }
        }

        output.Push((num, symb));
        input.Clear();
    }

    public static int SolvePart1(string input)
    {
        var lines = input.Split("\n");
        var map = lines.Select(line => line.ToCharArray()).ToArray();

        var results = ParseSchematic(map);

        var sum = 0;
        foreach (var num in results)
        {
            sum += num;
        }

        return sum;
    }
}
