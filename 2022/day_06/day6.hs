import Data.List (tails, nub, elemIndex)

main = readFile "input.txt" >>= print . elemIndex 14 . (map (length . nub . take 14) . init . tails)

