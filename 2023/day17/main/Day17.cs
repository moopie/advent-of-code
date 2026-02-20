namespace main;

public static class Day17
{
    // Right, Down, Left, Up
    private static readonly (int dx, int dy)[] Directions =
    {
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1)
    };

    public static int SolvePart1(string input)
    {
        var grid = ParseInput(input);
        return FindPath(grid, minStraight: 1, maxStraight: 3);
    }

    public static int SolvePart2(string input)
    {
        var grid = ParseInput(input);
        return FindPath(grid, minStraight: 4, maxStraight: 10);
    }

    private static int FindPath(int[][] grid, int minStraight, int maxStraight)
    {
        int height = grid.Length;
        int width = grid[0].Length;

        var pq = new PriorityQueue<State, int>();
        var best = new Dictionary<(int x, int y, int dir, int straight), int>();

        // Start from (0,0) going right and down
        pq.Enqueue(new State(0, 0, 0, 0, 0), 0);
        pq.Enqueue(new State(0, 0, 1, 0, 0), 0);

        while (pq.Count > 0)
        {
            var state = pq.Dequeue();

            if (state.X == width - 1 && state.Y == height - 1)
            {
                if (state.Straight >= minStraight)
                {
                    return state.Cost;
                }
            }

            var key = (state.X, state.Y, state.Dir, state.Straight);

            if (best.TryGetValue(key, out var existing))
            {
                if (existing <= state.Cost)
                {
                    continue;
                }
            }

            best[key] = state.Cost;

            // Continue straight
            if (state.Straight < maxStraight)
            {
                TryMove(state, state.Dir, state.Straight + 1);
            }

            // Turn left/right only if minimum straight satisfied
            if (state.Straight >= minStraight)
            {
                int left = (state.Dir + 3) % 4;
                int right = (state.Dir + 1) % 4;

                TryMove(state, left, 1);
                TryMove(state, right, 1);
            }
        }

        return -1;

        void TryMove(State current, int newDir, int newStraight)
        {
            var (dx, dy) = Directions[newDir];

            int nx = current.X + dx;
            int ny = current.Y + dy;

            if (nx < 0 || nx >= width || ny < 0 || ny >= height)
            {
                return;
            }

            int newCost = current.Cost + grid[ny][nx];

            pq.Enqueue(
                new State(nx, ny, newDir, newStraight, newCost),
                newCost
            );
        }
    }

    private static int[][] ParseInput(string input)
    {
        return input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(line =>
            {
                return line
                    .Select(c =>
                    {
                        return c - '0';
                    })
                    .ToArray();
            })
            .ToArray();
    }

    private readonly record struct State(
        int X,
        int Y,
        int Dir,
        int Straight,
        int Cost
    );
}
