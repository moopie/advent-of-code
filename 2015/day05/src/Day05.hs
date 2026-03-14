{-# OPTIONS_GHC -Wno-typed-holes #-}
module Day05 (solvePart1, solvePart2) where

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

    
solvePart2 :: String -> Int
solvePart2 input =
    length $ filter isNicePart2 $ lines input

isNicePart2 :: String -> Bool
isNicePart2 str =
    hasRepeatedPair str && hasRepeatWithGap str

hasRepeatedPair :: String -> Bool
hasRepeatedPair (a:b:rest) =
    [a,b] `isInfixOf` rest || hasRepeatedPair (b:rest)
hasRepeatedPair _ = False

hasRepeatWithGap :: String -> Bool
hasRepeatWithGap (a:b:c:rest)
    | a == c    = True
    | otherwise = hasRepeatWithGap (b:c:rest)
hasRepeatWithGap _ = False
