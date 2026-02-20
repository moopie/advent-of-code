using main;

namespace tests;

public class Day17Tests
{
    private const string Example =
        @"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        ";

    [Fact]
    public void Part1_ShouldBe_102()
    {
        Assert.Equal(102, Day17.SolvePart1(Example));
    }

    [Fact]
    public void Part2_ShouldBe_94()
    {
        Assert.Equal(94, Day17.SolvePart2(Example));
    }
}
