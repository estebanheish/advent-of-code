--- Day 11: Dumbo Octopus ---

import Data.Char
import Data.Matrix
import Data.Maybe
import Data.Function

main :: IO ()
main = do
  input <- fromLists . (map . map) digitToInt . lines <$> readFile "input/11_Dumbo_Octopus.input"
  print $ solve1 input 100 -- Part One: 1594
  print $ solve2 input 0   -- Part Two: 437


vecinos (r,c) = [(r-1,c-1),(r-1,c),(r-1,c+1), (r,c-1),(r,c+1),(r+1,c-1),(r+1,c),(r+1,c+1)]

flash :: Matrix Int -> Matrix Int
flash m = mapPos (f m) m
    where f m (r,c) n | n > 9 || n == 0 = 0
                      | otherwise =
                      let nine = length $ filter (>9) $ catMaybes $ (\(r,c) -> safeGet r c m) <$> vecinos (r,c)
                      in n + nine

fixflash = fix (\f m -> if m == flash m then m else f (flash m))

cflashes m = sum [1 | r <- [1..nrows m], c <- [1..ncols m], m ! (r,c) == 0]

nextGen = fixflash . mapPos (\_ n -> n+1)

step m = (m' , cflashes m')
    where m' = nextGen m

solve1 m 0 = 0
solve1 m n = f + solve1 m' (n-1)
    where (m', f) = step m

solve2 m n | all (==0) (toList m) = n
           | otherwise             = solve2 m' (n+1)
    where m' = nextGen m
