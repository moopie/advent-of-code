namespace tests;

public class Day8Tests
{
    private const string Example =
        """
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        """;

    [Fact]
    public void Part1_ShouldBe_2()
    {
        var result = Day8.SolvePart1(Example);

        Assert.Equal(2, result);
    }
}