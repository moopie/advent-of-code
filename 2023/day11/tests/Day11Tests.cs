namespace tests;

public class Day11Tests
{
    const string Example = @"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        ";

    [Fact]
    public void Part1_ShouldBe_374()
    {
        Assert.Equal(374, Day11.SolvePart1(Example));
    }

    [Fact]
    public void Part2_ShouldBe_1030()
    {
        Assert.Equal(1030, Day11.SolvePart2(Example, 10));
    }

    [Fact]
    public void Part2_ShouldBe_8410()
    {
        Assert.Equal(8410, Day11.SolvePart2(Example, 100));
    }
}
