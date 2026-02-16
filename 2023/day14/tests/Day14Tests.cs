using main;

namespace tests;

public class Day14Tests
{
    private const string Example = @"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
        ";

    [Fact]
    public void Part1_ShouldBe_136()
    {
        Assert.Equal(136, Day14.SolvePart1(Example));
    }

    [Fact]
    public void Part2_ShouldBe_64()
    {
        Assert.Equal(64, Day14.SolvePart2(Example, 1000000000));
    }
}
