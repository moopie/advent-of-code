namespace main;

public static class Day15
{
    public static long SolvePart1(string input)
    {
        var hashes = ParseInput(input);

        long sum = 0;
        foreach (var hash in hashes)
        {
            var value = GetHash(hash);
            sum += value;
        }
        return sum;
    }

    private static string[] ParseInput(string input)
    {
        return input
            .Split('\n')
            .SelectMany(x =>
                    x
                    .Split(',',
                        StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            .ToArray();
    }

    private static long GetHash(string str)
    {
        long current = 0;

        foreach (var ch in str)
        {
            current += ch;        // ASCII value
            current *= 17;
            current %= 256;
        }

        return current;
    }
}
