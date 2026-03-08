module Main (main) where

import Day02 (solvePart1, solvePart2)

main :: IO ()
main = do
  input <- readFile "input.txt"
  putStrLn "AOC 2015 day 2!"
  putStrLn ("Part 1: " ++ show(solvePart1 input))
  putStrLn ("Part 2: " ++ show(solvePart2 input))
