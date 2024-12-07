import Data.Bifunctor (bimap)
import Data.List.Split (splitOn)
import Distribution.Utils.String (trim)

main = do
  input <- map (bimap read (map read . words) . (\[a, b] -> (a, b)) . splitOn ":") . lines . trim <$> getContents
  print $ (sum . map fst . filter check1) input
  print $ (sum . map fst . filter check2) input

check1 :: (Int, [Int]) -> Bool
check1 (t, [k]) = t == k
check1 (t, f : s : rest) = check1 (t, f * s : rest) || check1 (t, f + s : rest)

check2 :: (Int, [Int]) -> Bool
check2 (t, [k]) = t == k
check2 (t, f : s : rest) = check2 (t, f * s : rest) || check2 (t, f + s : rest) || check2 (t, read (show f ++ show s) : rest)
