import Data.List (transpose)
import Distribution.Utils.String (trim)

main = do
  input <- lines . trim <$> getContents
  print (part1 input)
  print (part2 input)

part1 :: [String] -> Int
part1 m = sum $ map (countXmas 0) (posibilities m)

posibilities :: [String] -> [String]
posibilities s = concat [s, map reverse s, v, map reverse v, d, map reverse d, d', map reverse d']
  where
    v = transpose s
    d = diagonals s
    d' = diagonals (reverse s)

countXmas :: Int -> String -> Int
countXmas c r | length r < 4 = c
countXmas c r@('X' : 'M' : 'A' : 'S' : _) = countXmas (c + 1) (tail r)
countXmas c r = countXmas c (tail r)

diagonals :: [[a]] -> [[a]]
diagonals = tail . go []
  where
    go b es_ =
      [h | h : _ <- b] : case es_ of
        [] -> transpose ts
        e : es -> go (e : ts) es
      where
        ts = [t | _ : t <- b]

isBoxXmas :: [String] -> Bool
isBoxXmas ss = ((== 2) . length . filter (elem "MAS")) [d, d', map reverse d, map reverse d']
  where
    d = diagonals ss
    d' = diagonals (map reverse ss)

part2 :: [String] -> Int
part2 ss = (length . filter isBoxXmas) [map (take 3 . drop c) (take 3 (drop r ss)) | c <- [0 .. length (head ss) - 1], r <- [0 .. length ss - 1]]
