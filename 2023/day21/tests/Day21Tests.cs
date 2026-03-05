using main;

namespace tests;

public class Day21Tests
{
    private const string Example =
        @"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        ";

    [Fact]
    public void Part1_ShouldBe_16()
    {
        Assert.Equal(16, Day21.SolvePart1(Example, 6));
    }
}
