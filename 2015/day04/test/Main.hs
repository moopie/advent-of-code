module Main (main) where

import Test.Hspec
import Day04 (solvePart1, solvePart2)

main :: IO ()
main = hspec spec

spec :: Spec
spec = do
  describe "Day04.solvePart1" $ do
    it "works for example 'abcdef'" $ do
      solvePart1 "abcdef\n" `shouldBe` 609043

    it "works for example 'pqrstuv'" $ do
      solvePart1 "pqrstuv\n" `shouldBe` 1048970

  describe "Day04.solvePart2" $ do
    it "works for example 'abcdef'" $ do
      solvePart2 "abcdef\n" `shouldBe` 6742839

    it "works for example 'pqrstuv'" $ do
      solvePart2 "pqrstuv\n" `shouldBe` 5714438
