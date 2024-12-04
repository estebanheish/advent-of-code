import Distribution.Utils.String (trim)

main = do
  input <- map (map (read :: String -> Int) . words) . lines . trim <$> getContents
  print $ length $ filter id (safe . diffs <$> input)
  print $ length $ filter id $ map (any safe) (map diffs . withoutOne <$> input)

diffs l = zipWith (-) l (tail l)

safe l = all (\k -> k >= 1 && k <= 3) l || all (\k -> k <= -1 && k >= -3) l

withoutOne rs = [take i rs ++ drop (1 + i) rs | i <- [0 .. length rs - 1]]
