module Main (main) where

import Test.Hspec
import Day05

main :: IO ()
main = hspec $ do
    describe "Day 5 examples" $ do
        it "'ugknbfddgicrmopn' is nice" $ solvePart1 "ugknbfddgicrmopn" `shouldBe` 1
        it "'jchzalrnumimnmhp' is naughty" $ solvePart1 "jchzalrnumimnmhp" `shouldBe` 0

        it "'qjhvhtzxzqqjkmpb' is nice" $ solvePart2 "qjhvhtzxzqqjkmpb" `shouldBe` 1
        it "'jchzalrnumimnmhp' is naughty" $ solvePart2 "uurcxstgmygtbstg" `shouldBe` 0
