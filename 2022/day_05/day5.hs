import Data.List.Split (splitOn)
import Data.Char (isSpace)
import Data.List (transpose)

main = do 
  [input, ins] <- (lines <$>) <$> splitOn "\n\n" <$> readFile "input.txt"
  let input' = map (filter (\x -> not (isSpace x))) $ transpose $ (\x -> [ c | (i, c) <- zip [0..] x, (i-1) `mod` 4 == 0]) <$> init input
      ins' = ((\x -> [ read n :: Int | (i, n) <- zip [0..] x, odd i]) . words) <$> ins
  print $ head <$> foldl (\acc x -> crane acc x reverse) input' ins'
  print $ head <$> foldl (\acc x -> crane acc x id) input' ins'

crane crates [m,f,t] r = [ if i == t-1 then boxes ++ c else if i == f-1 then drop m c else c | (i, c) <- zip [0..] crates]
  where boxes = r $ take m $ crates !! (f-1)
