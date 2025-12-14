Console.WriteLine("AoC 2023 - Day 1 - Part 2");

var inputPath = "input.txt";

if (!File.Exists(inputPath))
{
    Console.WriteLine("input.txt not found");
    return;
}

var wordToDigit = new Dictionary<string, int>
{
    ["one"] = 1,
    ["two"] = 2,
    ["three"] = 3,
    ["four"] = 4,
    ["five"] = 5,
    ["six"] = 6,
    ["seven"] = 7,
    ["eight"] = 8,
    ["nine"] = 9,
};

int GetLineValue(string line)
{
    var digits = new List<int>();

    for (int i = 0; i < line.Length; i++)
    {
        // Case 1: numeric digit
        if (char.IsDigit(line[i]))
        {
            digits.Add(line[i] - '0');
            continue;
        }

        // Case 2: spelled-out digit
        foreach (var kv in wordToDigit)
        {
            if (line.AsSpan(i).StartsWith(kv.Key))
            {
                digits.Add(kv.Value);
                break; // only one match per position
            }
        }
    }

    return digits.First() * 10 + digits.Last();
}

var sum = 0;

foreach (var line in File.ReadLines(inputPath))
{
    if (string.IsNullOrWhiteSpace(line))
        continue;

    var value = GetLineValue(line);
    sum += value;

    Console.WriteLine($"{line} -> {value}");
}

Console.WriteLine($"Result: {sum}");