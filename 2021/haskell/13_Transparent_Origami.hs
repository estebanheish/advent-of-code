--- Day 13: Transparent Origami ---

import Data.List
import Data.Char

main :: IO ()
main = do
  input <- lines <$> readFile "input/13_Transparent_Origami.input"
  let p = (\x -> read ("(" ++ x ++ ")")) <$> takeWhile (/="") input :: [(Int,Int)]
      folds = toFold <$> drop 1 (dropWhile (/="") input)
      m = toMatrix p (maximum (snd <$> p)) (maximum (fst <$> p))
  print $ solve1 folds m -- Part One: 743
  solve2 folds m         -- Part Two: RCPLAKHL

toMatrix p r c = [[if (c,r) `elem` p then 1 else 0 | c <- [0..c]] | r <- [0..r]]

toFold :: String -> (Char,Int)
toFold f = (e,read n)
    where e = if 'x' `elem` f then 'x' else 'y'
          n = filter isDigit f

foldPaper (c,n) m | c == 'y' = toMatrix (pyu ++ pyd) (ryu-1) (cyu-1)
                  | c == 'x' = toMatrix (pxu ++ pxd) (rxu-1) (cxu-1)
  where (yu,yd) = (take n m,drop (n+1) m)
        (xu,xd) = (take n <$> m,drop (n+1) <$> m)
        (pyu,ryu,cyu) = points yu
        (pyd,_,_) = points $ reverse yd
        (pxu,rxu,cxu) = points xu
        (pxd,_,_) = points $ reverse <$> xd

points m = (concat [[(ix,iy) | (x,ix) <- zip y [0..], x == 1] | (y,iy) <- zip m [0..]],length m,length (head m))

solve1 (f:_) m = sum $ concatMap (filter (==1)) $ foldPaper f m

solve2 [] m = prettyMatrix m
solve2 (f:fs) m = solve2 fs (foldPaper f m)

prettyMatrix = mapM_ (print . map (\ y -> if y == 1 then '#' else ' '))
