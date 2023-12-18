main = do
  input <- map parseL . init . lines <$> getContents
  let p1 = fst <$> input
      n1 = sum (map snd p1) `div` 2 + 1
      p2 = snd <$> input
      n2 = sum (map snd p2) `div` 2 + 1
  print $ (+ n1) $ shoelace $ vertices p1
  print $ (+ n2) $ shoelace $ vertices p2

type Pos = (Int, Int)

type Ins = (Char, Int)

parseL :: String -> (Ins, Ins)
parseL s = ((d, read n), (nTod a, b))
  where
    [d : _, n, _ : _ : c] = words s
    c' = init c
    b = read $ "0x" ++ init c'
    a = read $ "0x" ++ [last c'] :: Int

nTod 0 = 'R'
nTod 1 = 'D'
nTod 2 = 'L'
nTod 3 = 'U'

nextPos :: Pos -> Ins -> Pos
nextPos (r, c) ('R', n) = (r, c + n)
nextPos (r, c) ('L', n) = (r, c - n)
nextPos (r, c) ('U', n) = (r - n, c)
nextPos (r, c) ('D', n) = (r + n, c)

vertices :: [Ins] -> [Pos]
vertices = scanl nextPos (0, 0)

shoelace :: [(Int, Int)] -> Int
shoelace points =
  abs $ sum terms `div` 2
  where
    n = length points
    terms = zipWith (\(x1, y1) (x2, y2) -> x1 * y2 - x2 * y1) points (rotate points)
    rotate xs = drop 1 xs ++ [head xs]
