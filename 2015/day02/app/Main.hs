module Main (main) where

import Day02 (solvePart1)

main :: IO ()
main = do
  input <- readFile "input.txt"
  putStrLn "AOC 2015 day 2!"
  putStrLn ("Part 1: " ++ show(solvePart1 input))
