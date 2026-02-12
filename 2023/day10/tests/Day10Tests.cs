namespace tests;

public class Day10Tests
{
    private const string Example1 = @"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
        ";

    private const string Example2 = @"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
        ";

    [Fact]
    public void Part1_ShouldBe_4()
    {
        Assert.Equal(4, Day10.SolvePart1(Example1));
    }

    [Fact]
    public void Part1_ShouldBe_8()
    {
        Assert.Equal(8, Day10.SolvePart1(Example2));
    }
}
