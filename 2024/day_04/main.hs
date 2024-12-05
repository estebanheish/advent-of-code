import Data.List (tails, transpose)
import Distribution.Utils.String (trim)

main = do
  input <- lines . trim <$> getContents
  print (part1 input)
  print (part2 input)

part1 :: [String] -> Int
part1 = sum . map (length . filter (`elem` ["XMAS", "SAMX"]) . transpose . take 4 . tails) . ps
  where
    ps s = concat [s, transpose s, diagonals s, diagonals (reverse s)]

diagonals = map concat . transpose . zipWith (\ns xs -> ns ++ map (: []) xs) (iterate ([] :) [])

part2 :: [String] -> Int
part2 ss = sum [1 | c <- [0 .. length (head ss) - 3], r <- [0 .. length ss - 3], all (`elem` ["MAS", "SAM"]) [d r c, d' r c]]
  where
    d r c = [ss !! r !! c, ss !! (r + 1) !! (c + 1), ss !! (r + 2) !! (c + 2)]
    d' r c = [ss !! r !! (c + 2), ss !! (r + 1) !! (c + 1), ss !! (r + 2) !! c]
