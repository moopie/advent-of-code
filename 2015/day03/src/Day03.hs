module Day03 (solvePart1, solvePart2) where

import qualified Data.Set as S

data Dir = UpD | DownD | LeftD | RightD
    deriving (Show)

solvePart1 :: String -> Int
solvePart1 input =
    let dirs = strToDir input
        moves = S.fromList $ scanl dirToPos (0,0) dirs
    in length moves

solvePart2 :: String -> Int
solvePart2 input =
    let dirs = strToDir input
        santaDirs = everyOther dirs
        roboDirs  = everyOther (tail dirs)

        santaMoves = scanl dirToPos (0,0) santaDirs
        roboMoves  = scanl dirToPos (0,0) roboDirs

        visited = S.fromList (santaMoves ++ roboMoves)
    in S.size visited

everyOther :: [a] -> [a]
everyOther [] = []
everyOther (x:xs) = x : everyOther (drop 1 xs)

strToDir :: String -> [Dir]
strToDir = map charToDir

charToDir :: Char -> Dir
charToDir '^' = UpD
charToDir 'v' = DownD
charToDir '<' = LeftD
charToDir '>' = RightD
charToDir _   = error "invalid direction"

dirToPos :: (Int, Int) -> Dir -> (Int, Int)
dirToPos (x, y) UpD    = (x + 1, y)
dirToPos (x, y) DownD  = (x - 1, y)
dirToPos (x, y) LeftD  = (x, y - 1)
dirToPos (x, y) RightD = (x, y + 1)

