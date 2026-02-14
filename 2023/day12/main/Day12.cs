namespace main;

public static class Day12
{
    public static long SolvePart1(string input)
    {
        long sum = 0;
        foreach (var (pattern, groups) in ParseInput(input))
            sum += CountArrangements(pattern, groups);
        return sum;
    }

    private static IEnumerable<(string Pattern, int[] Groups)> ParseInput(string input)
    {
        foreach (var line in input.Split('\n', StringSplitOptions.RemoveEmptyEntries))
        {
            var trimmed = line.Trim();
            if (trimmed.Length == 0) continue;

            var parts = trimmed.Split(' ', StringSplitOptions.RemoveEmptyEntries);
            var pattern = parts[0];
            var groups = parts[1].Split(',', StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray();
            yield return (pattern, groups);
        }
    }

    private static long CountArrangements(string s, int[] groups)
    {
        // Key: (pos, groupIndex, runLen)
        var memo = new Dictionary<(int pos, int gi, int run), long>(capacity: s.Length * 16);

        long Dp(int pos, int gi, int run)
        {
            var key = (pos, gi, run);
            if (memo.TryGetValue(key, out var cached)) return cached;

            // End of string: validate any active run and that we consumed all groups
            if (pos == s.Length)
            {
                if (run > 0)
                {
                    if (gi < groups.Length && run == groups[gi]) gi++;
                    else return memo[key] = 0;
                }
                return memo[key] = (gi == groups.Length) ? 1 : 0;
            }

            long ways = 0;
            char c = s[pos];

            // Try placing '.' (or treating '?' as '.')
            if (c == '.' || c == '?')
            {
                if (run == 0)
                {
                    ways += Dp(pos + 1, gi, 0);
                }
                else
                {
                    // Closing a run: must match current group exactly
                    if (gi < groups.Length && run == groups[gi])
                        ways += Dp(pos + 1, gi + 1, 0);
                }
            }

            // Try placing '#' (or treating '?' as '#')
            if (c == '#' || c == '?')
            {
                // Can't exceed current group length
                if (gi < groups.Length && run + 1 <= groups[gi])
                    ways += Dp(pos + 1, gi, run + 1);
            }

            return memo[key] = ways;
        }

        return Dp(0, 0, 0);
    }
}
