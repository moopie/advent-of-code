module Main (main) where

import Day02 (solvePart1)
import Test.Hspec

main :: IO ()
main = hspec $ do
    describe " Day 2 examples" $ do
        it "2x3x4 -> 58" $ solvePart1 "2x3x4" `shouldBe ` 58

        it "1x1x10 -> 43" $ solvePart1 "1x1x10" `shouldBe ` 43
