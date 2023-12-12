import Data.List
import Data.List.Split
import Data.MemoTrie

main = do
  input <- map parse . lines . init <$> getContents
  (print . sum . map arrgsMemo) input
  (print . sum . map (arrgsMemo . unfold)) input

unfold (a, b) = (a', b')
  where
    a' = (intercalate "?" . replicate 5) a
    b' = (concat . replicate 5) b

parse :: String -> (String, [Int])
parse s = (a, (map read . splitOn ",") b)
  where
    [a, b] = splitOn " " s

arrgs :: ((String, [Int]) -> Int) -> (String, [Int]) -> Int
arrgs f ([], []) = 1
arrgs f ([], _) = 0
arrgs f (s, []) | '#' `elem` s = 0
arrgs f ('.' : ss, b) = f (ss, b)
arrgs f ('?' : ss, ns) = f (ss, ns) + f ('#' : ss, ns)
arrgs f (a@('#' : ss), n : ns)
  | notElem '.' (take n a)
      && length a >= n
      && (length a == n || a !! n /= '#') =
      f (drop (n + 1) a, ns)
arrgs f _ = 0

arrgsMemo = memoFix arrgs
