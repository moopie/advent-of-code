using main;

namespace tests;

public class Day15Tests
{
    const string Example = @"
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        ";

    [Fact]
    public void Part1_ShouldBe_()
    {
        Assert.Equal(1320, Day15.SolvePart1(Example));
    }
}
