--- Day 8: Seven Segment Search ---

import Data.List (sort, intersect)
import Data.Maybe (catMaybes, fromMaybe)

main :: IO ()
main = do
    n <- lines <$> readFile "input/08_Seven_Segment_Search.input"
    let n' = map ((\(x,y) -> (sort <$> words x, sort <$> words (drop 1 y))) . span (/='|')) n
    print $ solve1 n' -- Part One: 261
    print $ solve2 n' -- Part Two: 987553

solve1 [] = 0
solve1 ((_,x):xs) = sum [ 1 | o <- x, length o `elem` [2,4,3,7]] + solve1 xs

luz :: String -> Maybe (Int, String)
luz s | length s == 2 = Just (1, sort s)
      | length s == 3 = Just (7, sort s)
      | length s == 4 = Just (4, sort s)
      | length s == 7 = Just (8, sort s)
      | otherwise = Nothing

decode :: [(Int, String)] -> String -> Char
decode d s = let
  c s n = length $ intersect s (sort (fromMaybe "" (lookup n d)))
  common s d = (c s 1, c s 4, c s 7, c s 8)
  in case common (sort s) d of
       (2,3,3,6) -> '0'
       (2,2,2,2) -> '1'
       (1,2,2,5) -> '2'
       (2,3,3,5) -> '3'
       (2,4,2,4) -> '4'
       (1,3,2,5) -> '5'
       (1,3,2,6) -> '6'
       (2,2,3,3) -> '7'
       (2,4,3,7) -> '8'
       (2,4,3,6) -> '9'

solve2 [] = 0
solve2 ((x,y):xs) = read (decode (catMaybes (luz <$> x)) <$> y) + solve2 xs
