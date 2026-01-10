Console.WriteLine("AOC 2023 day 4!");

var input = File.ReadAllLines("input.txt");

var sum = 0;

foreach (var line in input)
{
    var parts = line.Split(' ');

    var id = 0;
    var state = "win";
    var win = new List<int>();
    var have = new List<int>();

    foreach (var part in parts)
    {
        if (part.Length == 0)
            continue;
        if (part == "Card")
            continue;
        if (part.EndsWith(":"))
        {
            id = int.Parse(part.Substring(0, part.Length - 1));
            continue;
        }
        if (part == "|")
        {
            state = "have";
            continue;
        }

        var num = int.Parse(part);

        switch (state)
        {
            case "win":
                win.Add(num);
                break;
            case "have":
                have.Add(num);
                break;
            default:
                throw new Exception();
        }
    }

    //Console.Write($"{id}: w: ");
    //foreach (var num in win)
    //{
    //    Console.Write($"{num} ");
    //}
    //Console.Write("h: ");
    //foreach (var num in have)
    //{
    //    Console.Write($"{num} ");
    //}
    //Console.WriteLine();

    var score = 0;

    foreach (var num in have)
    {
        if (win.Contains(num))
        {
            if (score == 0)
            {
                score++;
            }
            else
            {
                score *= 2;
            }
        }
    }

    sum += score;
}

Console.WriteLine($"Part 1 solution: {sum}");
