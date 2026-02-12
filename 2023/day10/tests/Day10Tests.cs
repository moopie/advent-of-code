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

    private const string Example3 = @"
    ...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........
    ";

    private const string Example4 = @"
    .F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...
    ";

    private const string Example5 = @"
    FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L
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

    [Fact]
    public void Part2_ShouldBe_4()
    {
        Assert.Equal(4, Day10.SolvePart2(Example3));
    }

    [Fact]
    public void Part2_ShouldBe_8()
    {
        Assert.Equal(8, Day10.SolvePart2(Example4));
    }

    [Fact]
    public void Part2_ShouldBe_10()
    {
        Assert.Equal(10, Day10.SolvePart2(Example5));
    }
}
