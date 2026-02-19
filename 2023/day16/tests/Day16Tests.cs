using main;

namespace tests;

public class Day16Tests
{
    private const string Example =
    @"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    ";

    [Fact]
    public void Part1_ShouldBe_46()
    {
        Assert.Equal(46, Day16.SolvePart1(Example));
    }

    [Fact]
    public void Part2_ShouldBe_51()
    {
        Assert.Equal(51, Day16.SolvePart2(Example));
    }
}
