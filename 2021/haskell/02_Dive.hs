--- Day 2: Dive! ---

import Data.Char (isAlpha, isDigit)

main :: IO ()
main = do
    n <- lines <$> readFile "input/02_Dive.input"
    print $ solve1 n (0,0)   --- Part One: 2039256
    print $ solve2 n (0,0,0) --- Part Two: 1856459736

solve1 :: [String] -> (Int,Int) -> Int
solve1 [] (h,d) = h*d
solve1 (x:xs) (h,d) = case filter isAlpha x of
    "forward" -> solve1 xs (h+x', d)
    "up"      -> solve1 xs (h, d-x')
    "down"    -> solve1 xs (h, d+x')
    where x' = read (filter isDigit x) :: Int

solve2 :: [String] -> (Int,Int,Int) -> Int
solve2 [] (h,d,a) = h*d
solve2 (x:xs) (h,d,a) = case filter isAlpha x of
    "forward" -> solve2 xs (h+x', d+(x'*a), a)
    "up"      -> solve2 xs (h, d, a-x')
    "down"    -> solve2 xs (h, d, a+x')
    where x' = read (filter isDigit x) :: Int
