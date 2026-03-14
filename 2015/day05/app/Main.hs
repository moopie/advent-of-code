module Main (main) where

import Day05 (solvePart1)

main :: IO ()
main = do
  putStrLn "AOC 2015 day 5!"
  input <- readFile "input.txt"
  putStrLn  ("Part 1: " ++ show(solvePart1 input))
