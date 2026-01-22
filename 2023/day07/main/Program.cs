Console.WriteLine("AOC 2023 day 7!");

var input = File.ReadAllText("input.txt");

var part1 = Day7.SolvePart1(input);

Console.WriteLine($"Part 1 solution: {part1}");

internal record Hand(IList<int> Cards, int Bid);

public static class Day7
{
    private static readonly Dictionary<char, int> CardValues = new()
    {
        { 'A', 14 },
        { 'K', 13 },
        { 'Q', 12 },
        { 'J', 11 },
        { 'T', 10 },
    };

    public static int SolvePart1(string input)
    {
        var hands = ParseInput(input);
        hands.Sort(
            (a, b) =>
            {
                var typeA = GethandType(a.Cards);
                var typeB = GethandType(b.Cards);

                if (typeA != typeB)
                {
                    return typeA.CompareTo(typeB);
                }

                for (var i = 0; i < 5; i++)
                {
                    if (a.Cards[i] != b.Cards[i])
                    {
                        return a.Cards[i].CompareTo(b.Cards[i]);
                    }
                }
                return 0;
            }
        );

        var sum = 0;
        for (var i = 0; i < hands.Count; i++)
        {
            sum += hands[i].Bid * (i + 1);
        }
        return sum;
    }

    private static List<Hand> ParseInput(string input)
    {
        var result = new List<Hand>();
        foreach (
            var line in input.Split(
                "\n",
                StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries
            )
        )
        {
            var parts = line.Split(
                " ",
                StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries
            );

            var cards = parts[0];
            var bid = int.Parse(parts[1]);

            var current = ParseCards(cards);
            result.Add(new Hand(current, bid));
        }

        return result;
    }

    private static int GethandType(IList<int> cards)
    {
        var groups = cards
            .GroupBy(x => x)
            .Select(x => x.Count())
            .OrderByDescending(x => x)
            .ToList();

        return groups switch
        {
            [5] => 6, // 5 of a kind
            [4, 1] => 5, // 4 of a kind
            [3, 2] => 4, // full house
            [3, 1, 1] => 3, // 3 of a kind
            [2, 2, 1] => 2, // two pair
            [2, 1, 1, 1] => 1, // one pair
            _ => 0, // high card
        };
    }

    private static List<int> ParseCards(string cards)
    {
        var ret = cards
            .ToCharArray()
            .Select(card => char.IsDigit(card) ? int.Parse(card.ToString()) : CardValues[card])
            .ToList();

        return ret;
    }
}
