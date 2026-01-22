namespace tests;

public class Day8Tests
{
    private const string ExamplePart1 =
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

    private const string ExamplePart2 =
        """
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        """;

    [Fact]
    public void Part1_ShouldBe_2()
    {
        var result = Day8.SolvePart1(ExamplePart1);

        Assert.Equal(2, result);
    }

    [Fact]
    public void Part2_ShouldBe_6()
    {
        var actual = Day8.SolvePart2(ExamplePart2);

        Assert.Equal(6, actual);
    }
}