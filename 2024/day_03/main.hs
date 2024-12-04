import Data.Char (isDigit)
import Data.List.Split (splitOn)
import Distribution.Utils.String (trim)

main = do
  input <- trim <$> getContents
  let part1 = filter ((== 2) . length) . map (splitOn ",") . filter (\l -> all (\k -> isDigit k || k == ',') l && head l /= ',' && last l /= ',') . map (takeWhile (/= ')')) . splitOn "mul("
  let part2 = (\l -> [head l] : map (tail . splitOn "do()") (tail l)) $ splitOn "don't()" input
  print $ sum $ map (product . map read) (part1 input)
  print $ sum $ map (product . map read) (part1 (concat (concat part2)))
