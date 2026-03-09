module Main (main) where

import Day03
import Test.Hspec

main :: IO ()
main = hspec $ do
    describe "Day 3 examples" $ do
        it "> -> 2" $ solvePart1 ">" `shouldBe` 2
        it "^>v< -> 4" $ solvePart1 "^>v<" `shouldBe` 4
        it "^v^v^v^v^v -> 2" $ solvePart1 "^v^v^v^v^v" `shouldBe` 2

        it "^v -> 3" $ solvePart2 "^v" `shouldBe` 3
        it "^>v< -> 3" $ solvePart2 "^>v<" `shouldBe` 3
        it "^v^v^v^v^v -> 11" $ solvePart2 "^v^v^v^v^v" `shouldBe` 11
