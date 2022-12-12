import Data.List (tails, nub, elemIndex)

main = readFile "input.txt" >>= print . elemIndex 4 . (map (length . nub . take 4) . init . tails)

