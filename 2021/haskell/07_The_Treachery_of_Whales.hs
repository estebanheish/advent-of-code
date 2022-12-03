--- Day 7: The Treachery of Whales ---

import Data.List (minimum, maximum)

main :: IO ()
main = do
    n <- read . ("[" ++ ) . (++ "]") <$> readFile "input/07_The_Treachery_of_Whales.input"
    print $ minimum $ fuel n id                        -- Part One: 364898
    print $ minimum $ fuel n (\x -> x * (x+1) `div` 2) -- Part One: 104149091

fuel xs op = [ sum (map (op . abs . (-) h) xs) | h <- [minimum xs..maximum xs] ]
