module Main (main) where

import Day04 (solvePart1, solvePart2)

main :: IO ()
main = do
  putStrLn "AOC 2015 day 4!"
  input <- readFile "input.txt"
  putStrLn ("Part 1: " ++ show(solvePart1 input))
  putStrLn ("Part 2: " ++ show(solvePart2 input))
