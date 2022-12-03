--- Day 10: Syntax Scoring ---

import Data.Maybe (mapMaybe)
import Data.List (sort)

main :: IO ()
main = do
  input <- lines <$> readFile "input/10_Syntax_Scoring.input"
  print $ solve1 input -- Part One: 271245
  print $ solve2 input -- Part Two: 1685293086

removePairs :: String -> String
removePairs [] = []
removePairs [x] = [x]
removePairs (f:s:xs) | isPair [f,s] = removePairs xs
                     | otherwise    = f : removePairs (s:xs)
    where isPair x = x `elem` ["()","[]","{}","<>"]

reduce x | x == r    = x
         | otherwise = reduce r
      where r = removePairs x

isOpen x = x `elem` ["(","[","{","<"]
isClose x = x `elem` [")","]","}",">"]

ws x | x `elem` ["(", ")"] = 1
     | x `elem` ["[", "]"] = 2
     | x `elem` ["{", "}"] = 3
     | x `elem` ["<", ">"] = 4

pf [x] = ([],False)
pf (f:s:xs) | isOpen [f] && isClose [s] && ws [f] /= ws [s] = ([s],True)
            | otherwise = pf (s:xs)

idk x =
  let r = reduce x in
  case pf r of
    (")",_) -> 3
    ("]",_) -> 57
    ("}",_) -> 1197
    (">",_) -> 25137
    ("",_) -> 0

solve1 s = sum $ map idk s

isCorrupted = snd . pf

score [] p = p
score (s:ss) p | s == '(' = score ss (p'+1)
               | s == '[' = score ss (p'+2)
               | s == '{' = score ss (p'+3)
               | s == '<' = score ss (p'+4)
    where p' = p*5

idk' s | isCorrupted r = Nothing
       | otherwise = Just (score (reverse r) 0)
    where r = reduce s

solve2 s = scores !! (length scores `div` 2)
    where scores = sort $ mapMaybe idk' s
