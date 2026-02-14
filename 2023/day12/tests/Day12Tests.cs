using main;

namespace tests;

public class Day12Tests
{
    private const string Example1 = @"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    ";

    private const string Example2 = @"
        ???.### 1,1,3
    ";

    private const string Example3 = @"
        .??..??...?##. 1,1,3
    ";

    private const string Example4 = @"
        ?#?#?#?#?#?#?#? 1,3,1,6
    ";

    private const string Example5 = @"
        ????.#...#... 4,1,1
    ";

    private const string Example6 = @"
        ????.######..#####. 1,6,5
    ";

    private const string Example7 = @"
        ?###???????? 3,2,1
    ";



    [Fact]
    public void Part1_ShouldBe_21()
    {
        Assert.Equal(21, Day12.SolvePart1(Example1));
    }

    [Fact]
    public void Part1_ShouldBe_1_0()
    {
        Assert.Equal(1, Day12.SolvePart1(Example2));
    }

    [Fact]
    public void Part1_ShouldBe_4_0()
    {
        Assert.Equal(4, Day12.SolvePart1(Example3));
    }

    [Fact]
    public void Part1_ShouldBe_1_1()
    {
        Assert.Equal(1, Day12.SolvePart1(Example4));
    }

    [Fact]
    public void Part1_ShouldBe_1_2()
    {
        Assert.Equal(1, Day12.SolvePart1(Example5));
    }

    [Fact]
    public void Part1_ShouldBe_4_1()
    {
        Assert.Equal(4, Day12.SolvePart1(Example6));
    }

    [Fact]
    public void Part1_ShouldBe_10()
    {
        Assert.Equal(10, Day12.SolvePart1(Example7));
    }

    [Fact]
    public void Part2_ShouldBe_525152()
    {
        Assert.Equal(525152, Day12.SolvePart2(Example1));
    }
}
