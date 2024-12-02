import Data.List (sort)
import Data.List.Split (splitOn)
import Debug.Trace (traceShow)
import Distribution.Utils.String (trim)

main = do
  input <- foldl parse ([], []) . lines . trim <$> getContents
  print $ part1 input
  print $ part2 input

parse :: ([Int], [Int]) -> String -> ([Int], [Int])
parse (l, r) s =
  let [a, b] = words s
   in (read a : l, read b : r)

part1 :: ([Int], [Int]) -> Int
part1 (l, r) = sum [abs (a - b) | (a, b) <- zip (sort l) (sort r)]

part2 :: ([Int], [Int]) -> Int
part2 (ls, rs) = sum [l * k | l <- ls, let k = (length . filter (== l)) rs]
