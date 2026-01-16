Console.WriteLine("AOC 2023 day 5!");

var input = File.ReadAllLines("input.txt");

var part1Result = Day5.SolvePart1(input);

Console.WriteLine($"Solution for part 1: {part1Result}");


public record RangeMap(long Destination, long Source, long Length);

public class Almanac
{
    public Almanac(string[] lines)
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
                Seeds = line[6..]
                    .Split(' ', StringSplitOptions.RemoveEmptyEntries)
                    .Select(long.Parse)
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
                    Maps[id].Add(new RangeMap(
                        long.Parse(parts[0]),
                        long.Parse(parts[1]),
                        long.Parse(parts[2])
                    ));
                }
            }
        }
    }

    public List<long> Seeds { get; set; } = new();
    public List<List<RangeMap>> Maps { get; set; } = new();
}

public static class Day5
{
    public static int SolvePart1(string[] input)
    {
        var almanac = new Almanac(input);
        var result = new List<long>();

        foreach (var seed in almanac.Seeds)
        {
            var origin = seed;

            foreach (var ranges in almanac.Maps)
            {
                var map = ranges.FirstOrDefault(r => r.Source <= origin && r.Source + r.Length > origin);

                if (map is null) continue;

                origin = map.Destination + (origin - map.Source);
            }

            result.Add(origin);
        }

        return (int)result.Min();
    }
}