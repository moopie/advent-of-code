module Main (main) where

import qualified Day01 (solvePart1, solvePart2)

main :: IO ()
main = do
    putStrLn "AOC 2015 day 1!"
    input <- readFile "input.txt"
    putStrLn ("Part 1: " ++ show (Day01.solvePart1 input))
    putStrLn ("Part 2: " ++ show (Day01.solvePart2 input))
