namespace main;

record Module(string Name, string? Operation, string[] Targets);

record Pulse(string From, string To, bool High);

public static class Day20
{
    public static long SolvePart1(string input)
    {
        var modules = ParseInput(input)!;

        // Flip-flop states
        var flipState = new Dictionary<string, bool>();

        // Conjunction memory: module -> (input -> last pulse)
        var conjMemory = new Dictionary<string, Dictionary<string, bool>>();

        // Initialize conjunction input tracking
        foreach (var (name, module) in modules)
        {
            if (module.Operation == "&")
            {
                conjMemory[name] = new Dictionary<string, bool>();
            }
        }

        foreach (var (name, module) in modules)
        {
            foreach (var target in module.Targets)
            {
                if (conjMemory.ContainsKey(target))
                {
                    conjMemory[target][name] = false;
                }
            }
        }

        long lowCount = 0;
        long highCount = 0;

        for (int press = 0; press < 1000; press++)
        {
            var queue = new Queue<Pulse>();

            queue.Enqueue(new Pulse("button", "broadcaster", false));

            while (queue.Count > 0)
            {
                var pulse = queue.Dequeue();

                if (pulse.High)
                {
                    highCount++;
                }
                else
                {
                    lowCount++;
                }

                if (!modules.TryGetValue(pulse.To, out var module))
                {
                    continue;
                }

                // Broadcaster
                if (module.Operation == null)
                {
                    foreach (var target in module.Targets)
                    {
                        queue.Enqueue(new Pulse(module.Name, target, pulse.High));
                    }
                }

                // Flip-flop
                else if (module.Operation == "%")
                {
                    if (pulse.High)
                    {
                        continue;
                    }

                    if (!flipState.ContainsKey(module.Name))
                    {
                        flipState[module.Name] = false;
                    }

                    flipState[module.Name] = !flipState[module.Name];

                    var outHigh = flipState[module.Name];

                    foreach (var target in module.Targets)
                    {
                        queue.Enqueue(new Pulse(module.Name, target, outHigh));
                    }
                }

                // Conjunction
                else if (module.Operation == "&")
                {
                    conjMemory[module.Name][pulse.From] = pulse.High;

                    bool allHigh = true;

                    foreach (var v in conjMemory[module.Name].Values)
                    {
                        if (!v)
                        {
                            allHigh = false;
                            break;
                        }
                    }

                    var outHigh = !allHigh;

                    foreach (var target in module.Targets)
                    {
                        queue.Enqueue(new Pulse(module.Name, target, outHigh));
                    }
                }
            }
        }

        return lowCount * highCount;
    }

    private static Dictionary<string, Module> ParseInput(string input)
    {
        var options = StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries;
        return input
            .Split("\n", options)
            .Select(line =>
            {
                var parts = line.Split("->", options);
                var fullName = parts[0];
                var targets = parts[1].Split(", ", options).ToArray();
                var mod = fullName.StartsWith('%') || fullName.StartsWith('&');
                var op = mod ? fullName[0].ToString() : null;
                var name = mod ? fullName[1..].ToString() : fullName;
                return new Module(name, op, targets);
            })
            .ToDictionary(x => x.Name);
    }
}
