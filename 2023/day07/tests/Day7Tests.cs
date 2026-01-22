namespace tests;

public class Day7Tests
{
    private static readonly string Example =
        """
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        """;

    [Fact]
    public void Part1_ShouldBe_6440()
    {
        var result = Day7.SolvePart1(Example);

        Assert.Equal(6440, result);
    }
}