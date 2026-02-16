using main;

Console.WriteLine("AOC 2023 day 14!");

var input = File.ReadAllText("input.txt");

Console.WriteLine($"Part 1: {Day14.SolvePart1(input)}");
Console.WriteLine($"Part 2: {Day14.SolvePart2(input, 1000000000)}");
