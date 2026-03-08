module Day02 (solvePart1, solvePart2) where
import Data.List

solvePart1 :: String -> Int
solvePart1 =
    sum . map solve . lines
  where
    solve line =
        let [l, w, h] = map read (split 'x' line)
            sides = [l*w, w*h, h*l]
        in 2 * sum sides + minimum sides

solvePart2 :: String -> Int
solvePart2 =
    sum . map solve . lines
  where
    solve line =
        let [a,b,c] = sort (map read (split 'x' line))
        in 2*(a+b) + a*b*c

split :: Char -> String -> [String]
split _ "" = []
split c s =
    let (w, r) = break (== c) s
    in w : case r of
        [] -> []
        (_:xs) -> split c xs
