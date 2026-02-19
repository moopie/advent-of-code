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
}
