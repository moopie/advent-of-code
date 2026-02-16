namespace tests;

using main;

public class Day13Tests
{
    private const string Example = @"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
        ";

    [Fact]
    public void Part1_ShouldBe_405()
    {
        Assert.Equal(405, Day13.SolvePart1(Example));
    }

    [Fact]
    public void Part2_ShouldBe_400()
    {
        Assert.Equal(400, Day13.SolvePart2(Example));
    }
}
