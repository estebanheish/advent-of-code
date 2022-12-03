--- Day 14: Extended Polymerization ---

import Data.List.Split (splitOn)
import Data.List (nub, nubBy)
import Data.Maybe (fromMaybe)

main :: IO ()
main = do
  input <- lines <$> readFile "input/14_Extended_Polymerization.input"
  let start = head $ takeWhile (/="") input
      rules = map ((\[x,y] -> (x,y)) . splitOn " -> ") . drop 1 $ dropWhile (/="") input
  print $ solve start rules 10 -- Part 1: 2447
  print $ solve start rules 40 -- Part 2: 3018019237563

solve start rules n = maximum d - minimum d
    where step r s = merge $ concatMap (nstep r) s
          c = count $ iterate (step rules) (prep start) !! n
          d = (`div` 2) . snd <$> [ if z == head start || z == last start then (z,n+1) else c'  | c'@(z, n) <- c ]

prep [s] = []
prep (s:ss:sss) = ([s,ss], 1) : prep (ss:sss)

nstep rules (p@[s1,s2],n) = [([s1,m],n), ([m,s2],n)]
    where [m] = fromMaybe "" $ lookup p rules

merge p = [ (p', many p') | p' <- nub (fst <$> p) ] 
    where many y =  sum . map snd $ filter (\x -> fst x == y) p

count p = [ (c, many c) | c <- nub (fst <$> p') ]
    where p' = concat $ (\([p,p'],n) -> [(p,n), (p',n)]) <$> p
          many y = sum $ snd <$> filter (\x -> fst x == y) p'
