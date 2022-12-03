--- Day 9: Smoke Basin ---

import Data.Matrix as M (fromLists, Matrix, safeGet, nrows, ncols, (!))
import Data.Char (digitToInt)
import Data.Maybe (fromMaybe)
import Data.List (nub, sort)

main :: IO ()
main = do
  input <- M.fromLists . (map . map) digitToInt . lines <$> readFile "input/09_Smoke_Basin.input"
  print $ solve1 input -- Part One: 541
  print $ solve2 input -- Part Two: 847504

isLow :: Int -> Int -> Matrix Int -> Bool
isLow r c m = all (>n) [mget (r-1) c, mget (r+1) c, mget r (c+1), mget r (c-1)]
    where n = mget r c
          mget r c = fromMaybe 10 $ M.safeGet r c m

solve1 :: Matrix Int -> Int
solve1 m = sum [ (m ! (r,c)) + 1 | r <- [1..r], c <- [1..c], isLow r c m ]
    where { r = nrows m; c = ncols m }

solve2 :: Matrix Int -> Int
solve2 m = product . take 3 . reverse . sort $ length . nub <$> [ basin m (r,c) | r <- [1..r], c <- [1..c] ]
    where { r = nrows m; c = ncols m }

type Pos = (Int,Int)
basin :: Matrix Int -> Pos -> [Pos]
basin m p@(x,y) = [p] ++ vecinos ++ concatMap (basin m) vecinos
    where vecinos = filter (isValid p) [(x-1,y),(x+1,y),(x,y-1),(x,y+1)]
          isValid p1 p2 = mget p1 < mget p2 && mget p2 < 9
          mget (r,c) = fromMaybe 0 $ M.safeGet r c m
