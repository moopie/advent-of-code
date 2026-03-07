namespace main;

public static class Day25
{
    public static int SolvePart1(string input)
    {
        var graph = ParseInput(input);
        var nodes = graph.Keys.ToArray();

        for (var i = 0; i < nodes.Length; i++)
        {
            for (var j = i + 1; j < nodes.Length; j++)
            {
                var s = nodes[i];
                var t = nodes[j];

                var (cutValue, reachableCount) = MinCut(graph, s, t);

                if (cutValue == 3)
                {
                    var otherSide = nodes.Length - reachableCount;
                    return reachableCount * otherSide;
                }
            }
        }

        throw new Exception();
    }

    private static (int, int) MinCut(
        Dictionary<string, HashSet<string>> graph,
        string source,
        string sink)
    {
        var capacity = BuildCapacity(graph);
        var flow = new Dictionary<string, Dictionary<string, int>>();

        foreach (var u in capacity.Keys)
        {
            flow[u] = new Dictionary<string, int>();

            foreach (var v in capacity[u].Keys)
            {
                flow[u][v] = 0;
            }
        }

        var maxFlow = 0;

        while (true)
        {
            var parent = FindAugmentingPath(capacity, flow, source, sink);

            if (!parent.ContainsKey(sink))
            {
                break;
            }

            var pathFlow = int.MaxValue;
            var cur = sink;

            while (cur != source)
            {
                var prev = parent[cur];
                var residual = capacity[prev][cur] - flow[prev][cur];

                pathFlow = Math.Min(pathFlow, residual);

                cur = prev;
            }

            cur = sink;

            while (cur != source)
            {
                var prev = parent[cur];

                flow[prev][cur] += pathFlow;
                flow[cur][prev] -= pathFlow;

                cur = prev;
            }

            maxFlow += pathFlow;
        }

        var reachable = GetReachableResidual(capacity, flow, source);

        return (maxFlow, reachable.Count);
    }

    private static Dictionary<string, Dictionary<string, int>> BuildCapacity(
        Dictionary<string, HashSet<string>> graph)
    {
        var capacity = new Dictionary<string, Dictionary<string, int>>();

        foreach (var u in graph.Keys)
        {
            if (!capacity.ContainsKey(u))
            {
                capacity[u] = new Dictionary<string, int>();
            }

            foreach (var v in graph[u])
            {
                if (!capacity.ContainsKey(v))
                {
                    capacity[v] = new Dictionary<string, int>();
                }

                if (!capacity[u].ContainsKey(v))
                {
                    capacity[u][v] = 0;
                }

                if (!capacity[v].ContainsKey(u))
                {
                    capacity[v][u] = 0;
                }

                capacity[u][v] = 1;
                capacity[v][u] = 1;
            }
        }

        return capacity;
    }

    private static Dictionary<string, string> FindAugmentingPath(
        Dictionary<string, Dictionary<string, int>> capacity,
        Dictionary<string, Dictionary<string, int>> flow,
        string source,
        string sink)
    {
        var queue = new Queue<string>();
        var parent = new Dictionary<string, string>();
        var visited = new HashSet<string>();

        queue.Enqueue(source);
        visited.Add(source);

        while (queue.Count > 0)
        {
            var u = queue.Dequeue();

            foreach (var v in capacity[u].Keys)
            {
                var residual = capacity[u][v] - flow[u][v];

                if (residual > 0 && !visited.Contains(v))
                {
                    visited.Add(v);
                    parent[v] = u;

                    if (v == sink)
                    {
                        return parent;
                    }

                    queue.Enqueue(v);
                }
            }
        }

        return parent;
    }

    private static HashSet<string> GetReachableResidual(
        Dictionary<string, Dictionary<string, int>> capacity,
        Dictionary<string, Dictionary<string, int>> flow,
        string source)
    {
        var visited = new HashSet<string>();
        var queue = new Queue<string>();

        visited.Add(source);
        queue.Enqueue(source);

        while (queue.Count > 0)
        {
            var u = queue.Dequeue();

            foreach (var v in capacity[u].Keys)
            {
                var residual = capacity[u][v] - flow[u][v];

                if (residual > 0 && visited.Add(v))
                {
                    queue.Enqueue(v);
                }
            }
        }

        return visited;
    }

    private static Dictionary<string, HashSet<string>> ParseInput(string input)
    {
        var options = StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries;

        var graph = new Dictionary<string, HashSet<string>>();

        var lines = input.Split("\n", options);

        foreach (var line in lines)
        {
            var parts = line.Split(":", options);
            var from = parts[0];
            var tos = parts[1].Split(" ", options);

            if (!graph.ContainsKey(from))
            {
                graph[from] = new HashSet<string>();
            }

            foreach (var to in tos)
            {
                if (!graph.ContainsKey(to))
                {
                    graph[to] = new HashSet<string>();
                }

                graph[from].Add(to);
                graph[to].Add(from);
            }
        }

        return graph;
    }
}
