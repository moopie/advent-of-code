using main;

namespace tests;

public class Day20Tests
{
    private const string Example1 =
        @"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        ";

    private const string Example2 =
        @"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        ";

    [Fact]
    public void Part1_ShouldBe_32000000()
    {
        Assert.Equal(32000000, Day20.SolvePart1(Example1));
    }

    [Fact]
    public void Part1_ShouldBe_11687500()
    {
        Assert.Equal(11687500, Day20.SolvePart1(Example2));
    }
}
