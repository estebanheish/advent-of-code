--- Day 3: Binary Diagnostic ---

import Data.List (transpose, group, sort)
import Data.Char (digitToInt)

main :: IO ()
main = do
    n <- lines <$> readFile "input/03_Binary_Diagnostic.input"
    print $ solve1 n -- Part One: 1082324
    print $ solve2 n -- Part Two: 1353024

solve1 s = toDecimal gamma * toDecimal epsilon
    where gamma = concatMap (\x -> let u = sum (digitToInt <$> x)
                                      in if u > length x-u then "0" else "1") (transpose s)
          epsilon = [ if x == '1' then '0' else '1' |  x <- gamma ]
          toDecimal = foldl (\x y -> x * 2 + digitToInt y) 0

common :: Int -> [String] -> Int
common i s | z > u = 0
           | z < u = 1
           | otherwise = 1 -- head . head $ x'
    where x' = group $ sort $ map (\x -> read [x] :: Int) $ transpose s !! i
          z = length $ head x'
          u = length $ last x'

co2 :: [String] -> Int -> String
co2 [s] _ = s
co2 s n = co2 [ s' | s' <- s, digitToInt (s' !! n) == c ] (n+1)
    where c = if common n s == 1 then 0 else 1

oxygen :: [String] -> Int -> String
oxygen [s] _ = s
oxygen s n = oxygen [ s' | s' <- s, digitToInt (s' !! n) == c ] (n+1)
    where c = common n s

solve2 s = toDecimal c * toDecimal o
    where toDecimal = foldl (\x y -> x * 2 + digitToInt y) 0
          c = co2 s 0
          o = oxygen s 0
