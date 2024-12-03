import Data.List (sort, transpose)
import Distribution.Utils.String (trim)

main = do
  [l, r] <- map sort . transpose . map (map read . words) . lines . trim <$> getContents
  print $ sum $ zipWith ((abs .) . (-)) l r
  print $ sum $ map (\k -> length (filter (== k) r) * k) l
