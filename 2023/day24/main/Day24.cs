namespace main;

record Hailstone(long X, long Y, long Z, long Vx, long Vy, long Vz);

public static class Day24
{
    public static long SolvePart1(string input, long lowBound, long highBound)
    {
        var count = 0L;
        var hailstones = ParseInput(input);

        for (var i = 0; i < hailstones.Length - 1; i++)
        {
            for (var j = i + 1; j < hailstones.Length; j++)
            {
                if (IntersectsInTestArea(hailstones[i], hailstones[j], lowBound, highBound))
                {
                    count++;
                }
            }
        }

        return count;
    }

    public static long SolvePart2(string input)
    {
        var hailstones = ParseInput(input);

        for (var i = 0; i < hailstones.Length - 2; i++)
        {
            for (var j = i + 1; j < hailstones.Length - 1; j++)
            {
                for (var k = j + 1; k < hailstones.Length; k++)
                {
                    var matrix = BuildMatrix(hailstones[i], hailstones[j], hailstones[k]);

                    if (!TrySolve(matrix, out var solution))
                    {
                        continue;
                    }

                    var x = (long)Math.Round(solution[0]);
                    var y = (long)Math.Round(solution[1]);
                    var z = (long)Math.Round(solution[2]);
                    var vx = solution[3];
                    var vy = solution[4];
                    var vz = solution[5];

                    if (MatchesAll(hailstones, x, y, z, vx, vy, vz))
                    {
                        return x + y + z;
                    }
                }
            }
        }

        throw new InvalidOperationException("No valid solution found.");
    }

    private static Hailstone[] ParseInput(string input)
    {
        var options = StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries;

        return input
            .Split("\n", options)
            .Select(line =>
            {
                var parts = line.Split("@", options);

                var p1 = parts[0]
                    .Split(",", options)
                    .Select(long.Parse)
                    .ToArray();

                var p2 = parts[1]
                    .Split(",", options)
                    .Select(long.Parse)
                    .ToArray();

                return new Hailstone(
                    p1[0], p1[1], p1[2],
                    p2[0], p2[1], p2[2]
                );
            })
            .ToArray();
    }

    private static bool IntersectsInTestArea(Hailstone a, Hailstone b, double minBound, double maxBound)
    {
        double dx = b.X - a.X;
        double dy = b.Y - a.Y;

        double det = (double)a.Vx * (-b.Vy) - (double)a.Vy * (-b.Vx);

        if (Math.Abs(det) < 1e-12)
        {
            return false;
        }

        double ta = (dx * (-b.Vy) - dy * (-b.Vx)) / det;
        double tb = ((double)a.Vx * dy - (double)a.Vy * dx) / det;

        if (ta < 0 || tb < 0)
        {
            return false;
        }

        double x = a.X + a.Vx * ta;
        double y = a.Y + a.Vy * ta;

        return x >= minBound
               && x <= maxBound
               && y >= minBound
               && y <= maxBound;
    }

    private static double[,] BuildMatrix(Hailstone a, Hailstone b, Hailstone c)
    {
        var matrix = new double[6, 7];

        FillPairRows(matrix, 0, a, b);
        FillPairRows(matrix, 3, a, c);

        return matrix;
    }

    private static void FillPairRows(double[,] matrix, int row, Hailstone h1, Hailstone h2)
    {
        var dux = h2.Vx - h1.Vx;
        var duy = h2.Vy - h1.Vy;
        var duz = h2.Vz - h1.Vz;

        var dpx = h2.X - h1.X;
        var dpy = h2.Y - h1.Y;
        var dpz = h2.Z - h1.Z;

        var c1 = Cross(h2.X, h2.Y, h2.Z, h2.Vx, h2.Vy, h2.Vz);
        var c2 = Cross(h1.X, h1.Y, h1.Z, h1.Vx, h1.Vy, h1.Vz);

        var rhsX = c1.x - c2.x;
        var rhsY = c1.y - c2.y;
        var rhsZ = c1.z - c2.z;

        matrix[row + 0, 0] = 0;
        matrix[row + 0, 1] = duz;
        matrix[row + 0, 2] = -duy;
        matrix[row + 0, 3] = 0;
        matrix[row + 0, 4] = -dpz;
        matrix[row + 0, 5] = dpy;
        matrix[row + 0, 6] = rhsX;

        matrix[row + 1, 0] = -duz;
        matrix[row + 1, 1] = 0;
        matrix[row + 1, 2] = dux;
        matrix[row + 1, 3] = dpz;
        matrix[row + 1, 4] = 0;
        matrix[row + 1, 5] = -dpx;
        matrix[row + 1, 6] = rhsY;

        matrix[row + 2, 0] = duy;
        matrix[row + 2, 1] = -dux;
        matrix[row + 2, 2] = 0;
        matrix[row + 2, 3] = -dpy;
        matrix[row + 2, 4] = dpx;
        matrix[row + 2, 5] = 0;
        matrix[row + 2, 6] = rhsZ;
    }

    private static (long x, long y, long z) Cross(long ax, long ay, long az, long bx, long by, long bz)
    {
        return
        (
            ay * bz - az * by,
            az * bx - ax * bz,
            ax * by - ay * bx
        );
    }

    private static bool TrySolve(double[,] matrix, out double[] solution)
    {
        var n = 6;
        var a = (double[,])matrix.Clone();
        solution = new double[n];

        for (var col = 0; col < n; col++)
        {
            var pivotRow = col;

            for (var row = col + 1; row < n; row++)
            {
                if (Math.Abs(a[row, col]) > Math.Abs(a[pivotRow, col]))
                {
                    pivotRow = row;
                }
            }

            if (Math.Abs(a[pivotRow, col]) < 1e-9)
            {
                return false;
            }

            if (pivotRow != col)
            {
                for (var k = col; k <= n; k++)
                {
                    (a[col, k], a[pivotRow, k]) = (a[pivotRow, k], a[col, k]);
                }
            }

            var pivot = a[col, col];

            for (var k = col; k <= n; k++)
            {
                a[col, k] /= pivot;
            }

            for (var row = 0; row < n; row++)
            {
                if (row == col)
                {
                    continue;
                }

                var factor = a[row, col];

                for (var k = col; k <= n; k++)
                {
                    a[row, k] -= factor * a[col, k];
                }
            }
        }

        for (var i = 0; i < n; i++)
        {
            solution[i] = a[i, n];
        }

        return true;
    }

    private static bool MatchesAll(Hailstone[] hailstones, long rx, long ry, long rz, double rvx, double rvy, double rvz)
    {
        const double eps = 1e-6;

        foreach (var h in hailstones)
        {
            double? t = null;

            var dx = rvx - h.Vx;
            var dy = rvy - h.Vy;
            var dz = rvz - h.Vz;

            if (Math.Abs(dx) > eps)
            {
                t = (h.X - rx) / dx;
            }
            else if (rx != h.X)
            {
                return false;
            }

            if (Math.Abs(dy) > eps)
            {
                var ty = (h.Y - ry) / dy;

                if (t == null)
                {
                    t = ty;
                }
                else if (Math.Abs(t.Value - ty) > eps)
                {
                    return false;
                }
            }
            else if (ry != h.Y)
            {
                return false;
            }

            if (Math.Abs(dz) > eps)
            {
                var tz = (h.Z - rz) / dz;

                if (t == null)
                {
                    t = tz;
                }
                else if (Math.Abs(t.Value - tz) > eps)
                {
                    return false;
                }
            }
            else if (rz != h.Z)
            {
                return false;
            }

            if (t is null or < -eps)
            {
                return false;
            }
        }

        return true;
    }
}
