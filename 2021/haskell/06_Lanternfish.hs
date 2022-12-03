--- Day 6: Lanternfish ---

main :: IO ()
main = do
    n <- readFile "input/06_Lanternfish.input"
    let n' = read $ "[" ++ concat (lines n) ++ "]" :: [Int]
        f x xs = length $ filter (x==) xs
    print $ solve 80 (f 0 n', f 1 n', f 2 n', f 3 n', f 4 n', f 5 n', f 6 n', f 7 n', f 8 n')  -- Part One: 350149
    print $ solve 256 (f 0 n', f 1 n', f 2 n', f 3 n', f 4 n', f 5 n', f 6 n', f 7 n', f 8 n') -- Part Two: 1590327954513

solve :: Int -> (Int,Int,Int,Int,Int,Int,Int,Int,Int) -> Int
solve 0 (cero, uno, dos, tres, cuatro, cinco, seis, siete, ocho) = cero+uno+dos+tres+cuatro+cinco+seis+siete+ocho
solve n (cero, uno, dos, tres, cuatro, cinco, seis, siete, ocho) = solve (n-1) (uno, dos, tres, cuatro, cinco, seis, siete+cero, ocho, cero)
