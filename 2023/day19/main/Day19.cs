namespace main;

record Part(int X, int M, int A, int S);

record Rule(string? Attribute, char? Op, int Threshold, string Destination);

public static class Day19
{
    public static int SolvePart1(string input)
    {
        var (workflows, parts) = ParseInput(input);

        int sum = 0;

        foreach (var part in parts)
        {
            if (IsAccepted(part, workflows))
            {
                sum += part.X + part.M + part.A + part.S;
            }
        }
        return sum;
    }

    private static bool IsAccepted(Part part, Dictionary<string, Rule[]> workflows)
    {
        string current = "in";

        while (true)
        {
            var rules = workflows[current];
            string? destination = null;

            foreach (var rule in rules)
            {
                // Unconditional rule (the final "else" rule)
                if (rule.Attribute is null)
                {
                    destination = rule.Destination;
                    break;
                }

                // Get the value of x/m/a/s for this part
                int value = rule.Attribute switch
                {
                    "x" => part.X,
                    "m" => part.M,
                    "a" => part.A,
                    "s" => part.S,
                    _ => throw new InvalidOperationException($"Unknown attribute {rule.Attribute}")
                };

                bool condition = rule.Op switch
                {
                    '<' => value < rule.Threshold,
                    '>' => value > rule.Threshold,
                    _ => throw new InvalidOperationException($"Unknown operator {rule.Op}")
                };

                if (condition)
                {
                    destination = rule.Destination;
                    break;
                }
            }

            if (destination == "A")
            {
                return true;
            }

            if (destination == "R")
            {
                return false;
            }

            // Otherwise, jump to the next workflow
            current = destination!;
        }
    }

    private static (Dictionary<string, Rule[]>, Part[]) ParseInput(string input)
    {
        var sections = input.Split("\n\n");
        var options = StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries;

        // workflows: name -> Rule[]
        var workflows = sections[0]
            .Split('\n', options)
            .Select(line =>
            {
                var ip = line.Split('{', options);
                var name = ip[0];
                var rest = ip[1].TrimEnd('}');

                var tokens = rest.Split(',', options);
                var defaultDest = tokens[^1];         // last token without '}'
                var condTokens = tokens[..^1];        // all but last

                var rules = condTokens
                    .Select(r =>
                    {
                        var pts = r.Split(':', options);
                        var cond = pts[0];
                        var dest = pts[1];

                        var register = cond[0].ToString();
                        var op = cond[1];
                        var num = int.Parse(cond[2..]);

                        return new Rule(register, op, num, dest);
                    })
                    // add the final "else" rule
                    .Append(new Rule(null, null, 0, defaultDest))
                    .ToArray();

                return (name, rules);
            })
            .ToDictionary(x => x.name, x => x.rules);

        // parts
        var workflowParts = sections[1]
            .Split('\n', options)
            .Select(line =>
            {
                line = line.Trim('{', '}');

                var dict = line
                    .Split(',', options)
                    .Select(x =>
                    {
                        var pp = x.Split('=', options);
                        var attr = pp[0].Trim();
                        var val = int.Parse(pp[1].Trim());
                        return (attr, val);
                    })
                    .ToDictionary(x => x.attr, x => x.val);

                return new Part(dict["x"], dict["m"], dict["a"], dict["s"]);
            })
            .ToArray();

        return (workflows, workflowParts);
    }
}
