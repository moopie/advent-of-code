Console.WriteLine("AOC 2023 day 5!");

var input = File.ReadAllLines("input.txt");

var part1Result = Day5.SolvePart1(input);

Console.WriteLine($"Solution for part 1: {part1Result}");

var part2Result = Day5.SolvePart2(input);

Console.WriteLine($"Solution for part 2: {part2Result}");

public record RangeMap(long Destination, long Source, long Length);

public record SeedMap(long Source, long Length);

public class Almanac
{
    public Almanac(string[] lines, bool seedRange = false)
    {
        // 7 mapping blocks
        for (var i = 0; i < 7; i++)
            Maps.Add(new List<RangeMap>());

        var ids = new Dictionary<string, int>
        {
            { "seed-to-soil", 0 },
            { "soil-to-fertilizer", 1 },
            { "fertilizer-to-water", 2 },
            { "water-to-light", 3 },
            { "light-to-temperature", 4 },
            { "temperature-to-humidity", 5 },
            { "humidity-to-location", 6 }
        };

        string? current = null;

        foreach (var line in lines)
        {
            if (line.StartsWith("seeds"))
            {
                if (!seedRange)
                    Seeds = line[6..]
                        .Split(' ', StringSplitOptions.RemoveEmptyEntries)
                        .Select(seed => new SeedMap(long.Parse(seed), 1))
                        .ToList();
                else
                    Seeds = line[6..]
                        .Split(' ', StringSplitOptions.RemoveEmptyEntries)
                        .Chunk(2)
                        .Select(nums => new SeedMap(long.Parse(nums[0]), long.Parse(nums[1])))
                        .ToList();
                continue;
            }

            if (line.EndsWith("map:"))
            {
                current = line.Replace(" map:", "");
                continue;
            }

            if (current != null)
            {
                var parts = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);
                if (parts.Length == 3)
                {
                    var id = ids[current];
                    Maps[id]
                        .Add(
                            new RangeMap(
                                long.Parse(parts[0]),
                                long.Parse(parts[1]),
                                long.Parse(parts[2])
                            )
                        );
                }
            }
        }
    }

    public List<SeedMap> Seeds { get; } = new();
    public List<List<RangeMap>> Maps { get; } = new();
}

public static class Day5
{
    public static int SolvePart1(string[] input)
    {
        var almanac = new Almanac(input);
        var result = new List<long>();

        foreach (var seed in almanac.Seeds)
        {
            var origin = seed.Source;

            foreach (var ranges in almanac.Maps)
            {
                var map = ranges.FirstOrDefault(r =>
                    r.Source <= origin && r.Source + r.Length > origin
                );

                if (map is null)
                    continue;

                origin = map.Destination + (origin - map.Source);
            }

            result.Add(origin);
        }

        return (int)result.Min();
    }

    public static long SolvePart2(string[] input)
    {
        var almanac = new Almanac(input, true);

        // initial seed ranges
        var current = almanac
            .Seeds.Select(s => (start: s.Source, end: s.Source + s.Length))
            .ToList();

        foreach (var block in almanac.Maps)
        {
            var next = new List<(long start, long end)>();

            foreach (var (start, end) in current)
            {
                var s = start;
                var e = end;

                var remaining = new List<(long s, long e)> { (s, e) };

                foreach (var m in block)
                {
                    var ms = m.Source;
                    var me = m.Source + m.Length;

                    var updated = new List<(long, long)>();

                    foreach (var (rs, re) in remaining)
                    {
                        var overlapStart = Math.Max(rs, ms);
                        var overlapEnd = Math.Min(re, me);

                        if (overlapStart < overlapEnd)
                        {
                            // overlapping piece -> map it
                            var mappedStart = m.Destination + (overlapStart - ms);
                            var mappedEnd = mappedStart + (overlapEnd - overlapStart);
                            next.Add((mappedStart, mappedEnd));

                            // left remainder
                            if (rs < overlapStart)
                                updated.Add((rs, overlapStart));

                            // right remainder
                            if (overlapEnd < re)
                                updated.Add((overlapEnd, re));
                        }
                        else
                        {
                            updated.Add((rs, re)); // no overlap, keep as-is
                        }
                    }

                    remaining = updated;
                }

                // any unmapped pieces pass through unchanged
                next.AddRange(remaining);
            }

            current = next;
        }

        return current.Min(r => r.start);
    }
}