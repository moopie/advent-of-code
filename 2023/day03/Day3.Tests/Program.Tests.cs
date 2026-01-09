using Day3.Main;

namespace Day3.Tests;

public class Day03Tests
{
    private const string Example = """
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        """;

    [Fact]
    public void Example_Part1_ShouldBe_4361()
    {
        int result = Day03.SolvePart1(Example);
        Assert.Equal(4361, result);
    }

    [Fact]
    public void Example_Part2_ShouldBe_467835()
    {
        int result = Day03.SolvePart2(Example);
        Assert.Equal(467835, result);
    }
}
