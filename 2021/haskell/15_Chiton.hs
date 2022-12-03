--- Day 15: Chiton ---

import           Data.Char     (digitToInt)
import qualified Data.Map      as M (Map, empty, fromList, lookup)
import qualified Data.Matrix   as X (Matrix, fromLists, ncols, nrows, (!))
import           Data.Maybe    (fromJust, isNothing)
import qualified Data.Set      as S (Set, null, singleton, fromList, minView)

main :: IO ()
main = do
    input <- map (digitToInt <$>) . lines <$> readFile "input/15_Chiton.input"
    let grid = X.fromLists input
        egrid = X.fromLists $ expand input
    print $ solve (X.nrows grid,X.ncols grid)    grid M.empty (S.singleton ((1,1),0)) -- Part 1: 707
    print $ solve (X.nrows egrid,X.ncols egrid) egrid M.empty (S.singleton ((1,1),1)) -- Part 2: 2942

type Pos = (Int,Int)
type N = (Pos, Int)
type Grid = M.Map Pos Int
type XGrid = X.Matrix Int

solve :: Pos -> XGrid -> Grid -> S.Set N -> Maybe Int
solve goal _     lowerRisk f | S.null f = M.lookup goal lowerRisk
solve goal risks lowerRisk f            = solve goal risks lr (f' <> S.fromList vecinos)
    where (((x,y),coste),f') = fromJust $ S.minView f
          vecinos = [ (v,coste+r)
            | v@(x',y') <- [(x+1,y),(x-1,y),(x,y+1),(x,y-1)],
              let r  = risks X.! v
                  r' = M.lookup v lowerRisk,
              x' <= fst goal && x' >= 1,
              y' <= snd goal && y' >= 1,
              isNothing r' || (coste + r) < fromJust r' ]
          lr = M.fromList vecinos <> lowerRisk

expand :: [[Int]] -> [[Int]]
expand g = g' ++ menb 1 ++ menb 2 ++ menb 3 ++ menb 4
  where enb i x = let x' = rem (x+i) 9 in if x' == 0 then 9 else x'
        menb i = map (enb i <$>) g'
        g' = [ x ++ [ enb 1 y | y <- x ]
                 ++ [ enb 2 y | y <- x ]
                 ++ [ enb 3 y | y <- x ]
                 ++ [ enb 4 y | y <- x ]
                  | x <- g ]
