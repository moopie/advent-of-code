using main;

Console.WriteLine("AOC 2023 day 21!");

var input = File.ReadAllText("input.txt");

Console.WriteLine($"Part 1: {Day21.SolvePart1(input, 64)}");
Console.WriteLine($"Part 2: {Day21.SolvePart2(input, 26501365)}");
