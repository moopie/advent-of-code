module Day04 (solvePart1, solvePart2) where

import Crypto.Hash
import qualified Data.ByteString.Char8 as BS
import Data.ByteArray (convert)
import qualified Data.ByteString as B
import Data.Word

solvePart1 :: String -> Int
solvePart1 input =
    findHash (trim input) 5

solvePart2 :: String -> Int
solvePart2 input =
    findHash (trim input) 6


findHash :: String -> Int -> Int
findHash key zeros =
    head [ n | n <- [0..], isCorrectHash key zeros n ]


isCorrectHash :: String -> Int -> Int -> Bool
isCorrectHash key zeros n =
    case zeros of
        5 -> b0 == 0 && b1 == 0 && b2 < 16
        6 -> b0 == 0 && b1 == 0 && b2 == 0
        _ -> False
  where
    digest = hash (BS.pack (key ++ show n)) :: Digest MD5
    bytes  = convert digest :: B.ByteString

    b0 = B.index bytes 0
    b1 = B.index bytes 1
    b2 = B.index bytes 2


trim :: String -> String
trim = takeWhile (/= '\n')
