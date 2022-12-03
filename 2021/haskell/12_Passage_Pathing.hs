--- Day 12: Passage Pathing ---

import Data.List
import Data.Char
import Data.Maybe

main :: IO ()
main = do
  input <- map toCon . lines <$> readFile "input/12_Passage_Pathing.input"
  print $ solve1 [["start"]] input -- 3410
  print $ solve2 [["start"]] input -- 98796

toCon s = (takeWhile (/= '-') s, drop 1 (dropWhile (/= '-') s))

options path@(p:_) = mapMaybe f
    where l = filter (all isLower) path
          repeated = any ((> 1) . length) (group $ sort $ filter (all isLower) path)
          f (x,y) | p == "end" = Nothing
                  | x == p && if repeated then y `notElem` l else y /= "start" = Just y
                  | y == p && if repeated then x `notElem` l else x /= "start" = Just x
                  | otherwise =  Nothing

add cons path = if null ops then [path] else add' path ops
  where ops = options path cons
        add' p s = map (:p) s

explore paths cons | next == paths = paths
                   | otherwise = explore next cons
    where next = concat $ add cons <$> paths

lowerRepeated path = all ((== 1) . length) (group . sort $ filter (\x -> all isLower x && x `notElem` ["start", "end"]) path)

solve1 startPath cons = length $ filter lowerRepeated $ filter (\(x:_) -> x == "end") $ explore startPath cons
solve2 startPath cons = length $ filter (\(x:_) -> x == "end") $ explore startPath cons
