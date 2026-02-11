namespace tests;

public class Day09Tests
{
    const string Example =
        @"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        ";

    [Fact]
    public void Part1_ShouldBe_114()
    {
        Assert.Equal(114, Day09.SolvePart1(Example));
    }

    [Fact]
    public void Part2_ShouldBe_2()
    {
        Assert.Equal(2, Day09.SolvePart2(Example));
    }

}
