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
}
