Console.WriteLine("AOC 2023 day 1 part 1");

var lines = File.ReadAllLines("input.txt");

var first = 0;
var last = 0;
var acc = 0;

foreach (var line in lines)
{
    Console.WriteLine(line);
    if (line.Length == 0)
    {
        continue;
    }
    
    foreach (var t in line)
    {
        if (char.IsDigit(t))
        {
            first = int.Parse(t.ToString());
            break;
        }
    }

    for (var i = line.Length - 1; i >= 0; i--)
    {
        if (char.IsDigit(line[i]))
        {
            last = int.Parse(line[i].ToString());
            break;
        }
    }

    var num = first * 10 + last;
    Console.WriteLine($"num: {num}, acc: {acc}");
    acc += num;
}

Console.WriteLine($"Result: {acc}");