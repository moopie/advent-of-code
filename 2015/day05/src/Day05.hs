module Day05 (solvePart1) where

import Data.List (isInfixOf)

solvePart1 :: String -> Int
solvePart1 input = length (filter isNice (lines input)) 

isVowel c = c `elem` "aeiou"

hasThreeVowels s =
    length (filter isVowel s) >= 3

hasDouble s =
    or (zipWith (==) s (tail s))

hasNoForbidden s =
    not (any (`isInfixOf` s) ["ab","cd","pq","xy"])

isNice s =
    hasThreeVowels s &&
    hasDouble s &&
    hasNoForbidden s
