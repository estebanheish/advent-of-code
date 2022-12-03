--- Day 5: Hydrothermal Venture ---

import Data.List (sort, group)

main :: IO ()
main = do
    n <- readFile "input/05_Hydrothermal_Venture.input"
    let n1 = concatMap (build1 . parse . words) $ lines n
        n2 = concatMap (build2 . parse . words) $ lines n
    print $ length $ filter (>1) $ map length $ group $ sort n1 -- Part One: 5092
    print $ length $ filter (>1) $ map length $ group $ sort n2 -- Part Two: 20484

parse [xy1, _, xy2] = read $ "(" ++ xy1 ++ "," ++ xy2 ++ ")"

build1 :: (Int,Int,Int,Int) -> [(Int,Int)]
build1 (x1,y1,x2,y2) | x1 == x2 || y1 == y2 = take m $ zip xs yx
                     | otherwise = []
    where xs = [x1,(c x1 x2)..]
          yx = [y1,(c y1 y2)..]
          m = 1 + max (abs (x1-x2)) (abs (y1-y2))
          c x y = case compare x y of
                      EQ -> x
                      LT -> x+1
                      GT -> x-1

build2 :: (Int,Int,Int,Int) -> [(Int,Int)]
build2 (x1,y1,x2,y2) = take m $ zip xs yx
    where xs = [x1,(c x1 x2)..]
          yx = [y1,(c y1 y2)..]
          m = 1 + max (abs (x1-x2)) (abs (y1-y2))
          c x y = case compare x y of
                      EQ -> x
                      LT -> x+1
                      GT -> x-1
