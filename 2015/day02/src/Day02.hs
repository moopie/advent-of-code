module Day02 (solvePart1) where

solvePart1 :: String -> Int
solvePart1 =
    sum . map solve . lines
  where
    solve line =
        let [l, w, h] = map read (split 'x' line)
            sides = [l*w, w*h, h*l]
        in 2 * sum sides + minimum sides

split :: Char -> String -> [String]
split _ "" = []
split c s =
    let (w, r) = break (== c) s
    in w : case r of
        [] -> []
        (_:xs) -> split c xs
