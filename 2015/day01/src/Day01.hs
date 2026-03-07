module Day01 (solvePart1, solvePart2) where

solvePart1 :: String -> Int
solvePart1 = sum . map f
    where
        f '(' = 1
        f ')' = -1
        f _ = 0

solvePart2 :: String -> Int
solvePart2 input = go 0 1 input
  where
    go _ _ [] = -1
    go floor pos (c:cs)
      | newFloor == -1 = pos
      | otherwise = go newFloor (pos + 1) cs
      where
        newFloor = floor + step c

    step '(' = 1
    step ')' = -1
    step _ = 0
