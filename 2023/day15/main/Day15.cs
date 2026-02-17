namespace main;

record Lens(string Label, int FocalLength);

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

    public static long SolvePart2(string input)
    {
        var steps = ParseInput(input);

        var boxes = new List<Lens>[256];
        for (int i = 0; i < 256; i++)
            boxes[i] = new List<Lens>();

        foreach (var step in steps)
        {
            if (step.Contains('='))
            {
                var parts = step.Split('=');
                var label = parts[0];
                var focal = int.Parse(parts[1]);

                int boxIndex = (int)GetHash(label);
                var box = boxes[boxIndex];

                var existing = box.FindIndex(l => l.Label == label);
                if (existing >= 0)
                    box[existing] = new Lens(label, focal);
                else
                    box.Add(new Lens(label, focal));
            }
            else if (step.EndsWith('-'))
            {
                var label = step[..^1];

                int boxIndex = (int)GetHash(label);
                var box = boxes[boxIndex];

                box.RemoveAll(l => l.Label == label);
            }
        }

        return CalculatePower(boxes);
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

    private static long CalculatePower(List<Lens>[] boxes)
    {
        long total = 0;

        for (int boxIndex = 0; boxIndex < boxes.Length; boxIndex++)
        {
            var box = boxes[boxIndex];

            for (int slot = 0; slot < box.Count; slot++)
            {
                total += (boxIndex + 1) *
                         (slot + 1) *
                         box[slot].FocalLength;
            }
        }

        return total;
    }
}
