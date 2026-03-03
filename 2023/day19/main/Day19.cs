namespace main;

record Part(int X, int M, int A, int S);

record Rule(string? Attribute, char? Op, int Threshold, string Destination);

// For Part 2: a simple inclusive interval
record struct Interval(int Min, int Max);

// State of a "range of parts" at some workflow
record struct State(string Workflow, Interval X, Interval M, Interval A, Interval S);

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

    public static long SolvePart2(string input)
    {
        var (workflows, _) = ParseInput(input);

        // Start with all possible values 1..4000 in workflow "in"
        var stack = new Stack<State>();
        stack.Push(new State(
            "in",
            new Interval(1, 4000),
            new Interval(1, 4000),
            new Interval(1, 4000),
            new Interval(1, 4000)));

        long total = 0;

        while (stack.Count > 0)
        {
            var state = stack.Pop();

            if (state.Workflow == "A")
            {
                total += CountCombinations(state);
                continue;
            }

            if (state.Workflow == "R")
            {
                continue;
            }

            var rules = workflows[state.Workflow];

            var currentX = state.X;
            var currentM = state.M;
            var currentA = state.A;
            var currentS = state.S;

            foreach (var rule in rules)
            {
                // Default / unconditional rule (the "else" rule)
                if (rule.Attribute is null)
                {
                    if (!IsEmpty(currentX) && !IsEmpty(currentM) && !IsEmpty(currentA) && !IsEmpty(currentS))
                    {
                        stack.Push(new State(rule.Destination, currentX, currentM, currentA, currentS));
                    }

                    break;
                }

                // We split the current ranges into:
                // - true branch: satisfies this rule
                // - false branch: does not satisfy, continues to next rule
                var trueX = currentX;
                var trueM = currentM;
                var trueA = currentA;
                var trueS = currentS;

                var falseX = currentX;
                var falseM = currentM;
                var falseA = currentA;
                var falseS = currentS;

                switch (rule.Attribute)
                {
                    case "x":
                        {
                            SplitInterval(currentX, rule, out trueX, out falseX);
                            break;
                        }
                    case "m":
                        {
                            SplitInterval(currentM, rule, out trueM, out falseM);
                            break;
                        }
                    case "a":
                        {
                            SplitInterval(currentA, rule, out trueA, out falseA);
                            break;
                        }
                    case "s":
                        {
                            SplitInterval(currentS, rule, out trueS, out falseS);
                            break;
                        }
                    default:
                        {
                            throw new InvalidOperationException($"Unknown attribute {rule.Attribute}");
                        }
                }

                // True branch goes directly to the rule's destination
                if (!IsEmpty(trueX) && !IsEmpty(trueM) && !IsEmpty(trueA) && !IsEmpty(trueS))
                {
                    stack.Push(new State(rule.Destination, trueX, trueM, trueA, trueS));
                }

                // False branch stays in this workflow and continues with the next rule
                currentX = falseX;
                currentM = falseM;
                currentA = falseA;
                currentS = falseS;

                if (IsEmpty(currentX) || IsEmpty(currentM) || IsEmpty(currentA) || IsEmpty(currentS))
                {
                    // No values left that haven't taken a previous rule
                    break;
                }
            }
        }

        return total;
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
                if (rule.Attribute is null)
                {
                    // unconditional / default rule
                    destination = rule.Destination;
                    break;
                }

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

            current = destination!;
        }
    }

    private static bool IsEmpty(Interval interval)
    {
        return interval.Min > interval.Max;
    }

    private static long CountCombinations(State s)
    {
        long xCount = s.X.Max - s.X.Min + 1L;
        long mCount = s.M.Max - s.M.Min + 1L;
        long aCount = s.A.Max - s.A.Min + 1L;
        long sCount = s.S.Max - s.S.Min + 1L;

        return xCount * mCount * aCount * sCount;
    }

    // Split interval into values that satisfy rule (trueInterval)
    // and values that do not (falseInterval)
    private static void SplitInterval(Interval interval, Rule rule, out Interval trueInterval, out Interval falseInterval)
    {
        int min = interval.Min;
        int max = interval.Max;

        if (rule.Op == '<')
        {
            // x < T  -->  true: [min, T-1], false: [T, max]
            int tMax = Math.Min(max, rule.Threshold - 1);
            int fMin = Math.Max(min, rule.Threshold);

            trueInterval = new Interval(min, tMax);
            falseInterval = new Interval(fMin, max);
        }
        else if (rule.Op == '>')
        {
            // x > T  -->  true: [T+1, max], false: [min, T]
            int tMin = Math.Max(min, rule.Threshold + 1);
            int fMax = Math.Min(max, rule.Threshold);

            trueInterval = new Interval(tMin, max);
            falseInterval = new Interval(min, fMax);
        }
        else
        {
            throw new InvalidOperationException($"Unexpected operator {rule.Op}");
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
