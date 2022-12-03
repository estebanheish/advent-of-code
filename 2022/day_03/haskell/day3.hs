import Data.List (intersect)
import Data.Char (isLower, ord)

main = do 
  input <- lines <$> readFile "../input.txt"
  print $ solve1 input 
  print $ solve2 0 input 

solve1 :: [String] -> Int
solve1 i = sum $ (rank . share . half) <$> i

half :: String -> (String, String)
half s = splitAt (length s `div` 2) s

share :: (String, String) -> Char
share (a, b) = head $ intersect a b 

rank :: Char -> Int
rank c = ord c + (if isLower c then - ord 'a' + 1 else - ord 'A' + 27)

solve2 :: Int -> [String] -> Int
solve2 n [] = n
solve2 n (x:y:z:ss) = solve2 (n + rank (share3 x y z)) ss

share3 :: String -> String -> String -> Char
share3 a b c = head $ intersect c $ intersect a b 
