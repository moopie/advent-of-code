Console.WriteLine("AOC 2023 day 2!");

var allowed = new Dictionary<string, int>
{
    ["red"] = 12,
    ["green"] = 13,
    ["blue"] = 14,
};

var contents = File.ReadLines("input.txt");

var sum = 0;

foreach (var line in contents)
{
    var parts = line.Split(":");
    var id = parts[0][5..];
    var rest = parts[1];
    var moves = rest.Split(";");
    var valid = true;
    foreach (var move in moves)
    {
        var stones = move.Split(",");
        foreach (var stone in stones)
        {
            var kind = stone.Trim().Split(" ");
            var num = int.Parse(kind[0]);
            var name = kind[1];
            var req = allowed[name];
            if (num > req)
            {
                valid = false;
                break;
            }
        }

        if (!valid)
        {
            break;
        }
    }

    if (valid)
    {
        sum += int.Parse(id);
    }
}

Console.WriteLine($"Part 1 result: {sum}");
