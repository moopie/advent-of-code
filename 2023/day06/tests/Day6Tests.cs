namespace tests;

public class Day6Tests
{
    private const string Example =
        """
        Time:      7  15   30
        Distance:  9  40  200
        """;

    [Fact]
    public void Part1_ShouldBe_288()
    {
        var result = Day6.SolvePart1(Example);

        Assert.Equal(288, result);
    }

    [Fact]
    public void Part2_ShouldBe_71503()
    {
        var result = Day6.SolvePart2(Example);

        Assert.Equal(71503, result);
    }
}