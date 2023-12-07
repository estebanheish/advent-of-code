import Data.List
import Data.Ord

main = do
  input <- map words . lines <$> getContents
  let rankedBids = (map (\[a, b] -> read b :: Int) . sortBy (\[hand, bid] [hand', bid'] -> cmpHand hand hand')) input
      rankedBids2 = (map (\[a, b] -> read b :: Int) . sortBy (\[hand, bid] [hand', bid'] -> cmpHand2 hand hand')) input
  putStrLn $ "part 1 -> " ++ show (sum [r * i | (r, i) <- zip rankedBids [1 ..]])
  putStrLn $ "part 2 -> " ++ show (sum [r * i | (r, i) <- zip rankedBids2 [1 ..]])

order = reverse ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']

rankType :: String -> Int
rankType hand =
  case (sort . map length . group . sort) hand of
    [5] -> 6
    [1, 4] -> 5
    [2, 3] -> 4
    [1, 1, 3] -> 3
    [1, 2, 2] -> 2
    [1, 1, 1, 2] -> 1
    [1, 1, 1, 1, 1] -> 0

cmpChar a b =
  let Just a' = elemIndex a order
      Just b' = elemIndex b order
   in compare a' b'

cmpOne :: String -> String -> Ordering
cmpOne [] [] = EQ
cmpOne (a : as) (b : bs) =
  let c = cmpChar a b
   in if c == EQ then cmpOne as bs else c

cmpHand :: String -> String -> Ordering
cmpHand a b =
  let c = compare (rankType a) (rankType b)
   in if c == EQ then cmpOne a b else c

order2 = reverse ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J']

rankType2 :: String -> Int
rankType2 hand =
  let ns = (sort . map length . group . sort) hand
      js = length (filter (== 'J') hand)
   in case ns of
        [5] -> 6
        [1, 4] -> if js > 0 then 6 else 5
        [2, 3] -> if js > 0 then 6 else 4
        [1, 1, 3] -> if js > 0 then 5 else 3
        [1, 2, 2] -> if js == 2 then 5 else if js == 1 then 4 else 2
        [1, 1, 1, 2] -> if js > 0 then 3 else 1
        [1, 1, 1, 1, 1] -> 0 + js

cmpHand2 :: String -> String -> Ordering
cmpHand2 a b =
  let c = compare (rankType2 a) (rankType2 b)
   in if c == EQ then cmpOne2 a b else c

cmpOne2 :: String -> String -> Ordering
cmpOne2 [] [] = EQ
cmpOne2 (a : as) (b : bs) =
  let c = cmpChar2 a b
   in if c == EQ then cmpOne2 as bs else c

cmpChar2 a b =
  let Just a' = elemIndex a order2
      Just b' = elemIndex b order2
   in compare a' b'
