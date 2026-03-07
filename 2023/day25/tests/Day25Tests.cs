namespace tests;

using main;

public class Day25Tests
{
    const string Example =
        @"
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr
        ";

    [Fact]
    public void Part1_ShouldBe_54()
    {
        Assert.Equal(54, Day25.SolvePart1(Example));
    }
}
