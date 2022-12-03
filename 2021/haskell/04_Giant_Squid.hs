--- Day 4: Giant Squid ---

import Data.List (transpose)
import Data.List.Split (splitOn)

main :: IO ()
main = do
    input <- readFile "input/04_Giant_Squid.input"
    let l = lines input
        n = read <$> splitOn "," (concat (takeWhile (/= "") l)) :: [Int]
        bs = [ [ read <$> words y :: [Int] | y <- x ] | x <- buildBoards $ dropWhile (/= "") l ]
    print $ solve1 n bs 5            -- Part One: 23177
    print $ solve2 n bs 1 ([[]],0,0) -- Part Two: 6804

buildBoards [] = []
buildBoards x = take 5 (drop 1 x) : buildBoards (drop 6 x)

won n b = or [ and [ l'' `elem` n | l'' <- l' ]  | l' <- l ]
    where l = b ++ transpose b

solve1 n bs t | null w = solve1 n bs (t+1)
              | otherwise = sum (filter (`notElem` n') ((concat . head) w)) * last n'
    where n' = take t n
          w = filter (won n') bs

solve2 ns bs s (w,n,s') | s == length ns = sum (filter (`notElem` take s' ns) (concat w)) * n
                        | otherwise      =
  let
    ns' = take s ns
    ws = filter (won ns') bs
  in case ws of
    [] -> solve2 ns bs (s+1) (w,n,s')
    [x] -> solve2 ns (filter (not . won ns') bs) (s+1) (x, ns !! (s-1), s)
    _ -> solve2 ns (filter (not . won ns') bs) (s+1) (last ws, ns !! (s-1), s)
