Console.WriteLine("AOC 2023 day 7!");

var input = File.ReadAllText("input.txt");

var part1 = Day7.SolvePart1(input);

Console.WriteLine($"Part 1 solution: {part1}");

var part2 = Day7.SolvePart2(input);

Console.WriteLine($"Part 2 solution: {part2}");

internal record Hand(IList<int> Cards, int Bid);

public static class Day7
{
    private static readonly Dictionary<char, int> CardValuesPart1 = new()
    {
        { 'A', 14 },
        { 'K', 13 },
        { 'Q', 12 },
        { 'J', 11 },
        { 'T', 10 }
    };

    private static readonly Dictionary<char, int> CardValuesPart2 = new()
    {
        { 'A', 14 },
        { 'K', 13 },
        { 'Q', 12 },
        { 'T', 10 },
        { 'J', 1 }
    };

    public static int SolvePart1(string input)
    {
        var hands = ParseInput(input, CardValuesPart1);
        hands.Sort((a, b) =>
            {
                var typeA = GetHandType(a.Cards);
                var typeB = GetHandType(b.Cards);

                if (typeA != typeB) return typeA.CompareTo(typeB);

                for (var i = 0; i < 5; i++)
                    if (a.Cards[i] != b.Cards[i])
                        return a.Cards[i].CompareTo(b.Cards[i]);

                return 0;
            }
        );

        return hands.Select((t, i) => t.Bid * (i + 1)).Sum();
    }

    private static List<Hand> ParseInput(string input, Dictionary<char, int> cardValues)
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

            var current = ParseCards(cards, cardValues);
            result.Add(new Hand(current, bid));
        }

        return result;
    }

    private static int GetHandType(IList<int> cards, int? wildcard = null)
    {
        var wc = wildcard == null ? 0 : cards.Count(x => x == wildcard);

        var groups = cards
            .Where(x => x != wildcard)
            .GroupBy(x => x)
            .Select(x => x.Count())
            .OrderByDescending(x => x)
            .ToList();

        if (groups.Count == 0) return 6; // 5 of a Kind, all jokers

        groups[0] += wc;

        groups = groups.OrderByDescending(x => x).ToList();

        return groups switch
        {
            [5] => 6, // 5 of a kind
            [4, 1] => 5, // 4 of a kind
            [3, 2] => 4, // full house
            [3, 1, 1] => 3, // 3 of a kind
            [2, 2, 1] => 2, // two pair
            [2, 1, 1, 1] => 1, // one pair
            _ => 0 // high card
        };
    }

    private static List<int> ParseCards(string cards, Dictionary<char, int> cardValues)
    {
        var ret = cards
            .ToCharArray()
            .Select(card => char.IsDigit(card) ? int.Parse(card.ToString()) : cardValues[card])
            .ToList();

        return ret;
    }

    public static int SolvePart2(string input)
    {
        var hands = ParseInput(input, CardValuesPart2);
        hands.Sort((a, b) =>
            {
                var typeA = GetHandType(a.Cards, CardValuesPart2['J']);
                var typeB = GetHandType(b.Cards, CardValuesPart2['J']);

                if (typeA != typeB) return typeA.CompareTo(typeB);

                for (var i = 0; i < 5; i++)
                    if (a.Cards[i] != b.Cards[i])
                        return a.Cards[i].CompareTo(b.Cards[i]);

                return 0;
            }
        );

        return hands.Select((t, i) => t.Bid * (i + 1)).Sum();
    }
}