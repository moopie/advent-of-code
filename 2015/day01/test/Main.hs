module Main (main) where

import Day01 (solvePart1, solvePart2)
import Test.Hspec

main :: IO ()
main = hspec $ do
  describe "Day01 examples" $ do
    it "(()) -> 0" $
      solvePart1 "(())" `shouldBe` 0

    it "()() -> 0" $
      solvePart1 "()()" `shouldBe` 0

    it "(((" $
      solvePart1 "(((" `shouldBe` 3

  describe "Day01 part2 examples" $ do
    it ")" $
      solvePart2 ")" `shouldBe` 1

    it "()())" $
      solvePart2 "()())" `shouldBe` 5
