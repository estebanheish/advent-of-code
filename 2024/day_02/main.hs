import Distribution.Utils.String (trim)

main = do
  input <- map (map (read :: String -> Int) . words) . lines . trim <$> getContents
  print $ length $ filter id (safe <$> input)
  print $ length $ filter id (any safe . withoutOne <$> input)

safe :: [Int] -> Bool
safe rs = let ds = diffs rs in sameSign ds && bounds ds

diffs :: [Int] -> [Int]
diffs (r : rs) = fst $ foldl f ([], r) rs
  where
    f (ds, l) k = ((l - k) : ds, k)

bounds :: [Int] -> Bool
bounds = all (\k -> abs k >= 1 && abs k <= 3)

sameSign :: [Int] -> Bool
sameSign ds = abs ((sum . map signum) ds) == length ds

withoutOne rs = [take i rs ++ drop (1 + i) rs | i <- [0 .. length rs - 1]]
