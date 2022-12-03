--- Day 1: Sonar Sweep ---

main :: IO ()
main = do
    n <- readFile "input/01_Sonar_Sweep.input"
    let n' = read <$> lines n :: [Int]
    print $ solve1 n' --- Part One: 1665
    print $ solve2 n' --- Part Two: 1702

solve1 :: [Int] -> Int
solve1 [x]                    = 0
solve1 [x,xs]     | xs > x    = 1
                  | otherwise = 0
solve1 (x:xs:xss) | xs > x    = 1 + solve1 (xs:xss)
                  | otherwise = solve1 (xs:xss)

solve2 :: [Int] -> Int
solve2 [x,y,z,w]    | (x+y+z) < (y+z+w) = 1
                    | otherwise         = 0
solve2 (x:y:z:w:ss) | (x+y+z) < (y+z+w) = 1 + solve2 (y:z:w:ss)
                    | otherwise         = 0 + solve2 (y:z:w:ss)
