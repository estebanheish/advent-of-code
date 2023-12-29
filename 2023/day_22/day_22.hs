import Control.Parallel.Strategies
import Data.Function.Memoize
import Data.List
import Data.Map qualified as Map
import Data.Maybe (mapMaybe)
import Data.Set qualified as Set

main = do
  blocks <- map parse . init . lines <$> getContents
  let sorted = sortBlocks blocks
      settled = fall [] sorted
      sups = supporters settled

  print $ canDisintegrate settled sups
  print $ sum $ parallelMap (subtract 1 . length . nub . manyFall sups settled [] . (: [])) settled

parallelMap :: (a -> b) -> [a] -> [b]
parallelMap f xs = map f xs `using` parList rseq

type Block = (Int, Int, Int, Int, Int, Int)

parse :: String -> Block
parse s = read $ "(" ++ s' ++ ")"
  where
    s' = map (\c -> if c == '~' then ',' else c) s

chocan :: Block -> Block -> Bool
chocan (ax, ay, _, ax', ay', _) (bx, by, _, bx', by', _) =
  max ax bx <= min ax' bx' && max ay by <= min ay' by'

sortBlocks :: [Block] -> [Block]
sortBlocks = sortOn (\(_, _, k, _, _, _) -> k)

fall :: [Block] -> [Block] -> [Block]
fall settled [] = settled
fall settled (b@(x, y, z, x', y', z') : rest) = fall ((x, y, nz, x', y', z' - (z - nz)) : settled) rest
  where
    nz = (maximum . (1 :) . map (\(_, _, _, _, _, k) -> k + 1) . filter (chocan b)) settled

type Sup = Map.Map Block (Set.Set Block)

supporters :: [Block] -> Sup
supporters blocks = Map.fromList $ map (f blocks) blocks
  where
    f blocks b@(_, _, _, _, _, z') = (b, Set.fromList [a | a@(_, _, z, _, _, _) <- blocks, z' == z - 1, chocan b a])

canDisintegrate :: [Block] -> Sup -> Int
canDisintegrate blocks sup = (length . filter f) blocks
  where
    f :: Block -> Bool
    f b = case Map.lookup b sup of
      Just s -> all (otherSupport b sup) (Set.toList s)
      Nothing -> True

otherSupport :: Block -> Sup -> Block -> Bool
otherSupport b1 map b2 = any (\(b, bs) -> b /= b1 && Set.member b2 bs) (Map.toList map)

-- otherSupport' :: Set.Set Block -> Sup -> Block -> Bool
-- otherSupport' b1s map b2 = any (\(b, bs) -> Set.notMember b b1s && Set.member b2 bs) (Map.toList map)

-- manyFall :: Sup -> [Block] -> [Block] -> [Block] -> [Block]
-- manyFall sup bs fl [] = fl
-- manyFall sup bs fl checking =
--   let csup = (concatMap Set.toList . mapMaybe (`Map.lookup` sup)) checking
--       csup' = filter (not . otherSupport' (Set.fromList (fl ++ csup ++ checking)) sup) csup
--       fl' = nub $ fl ++ csup' ++ checking
--    in manyFall sup bs fl' csup'

otherSupport' :: Sup -> Set.Set Block -> Block -> Bool
otherSupport' map b1s b2 = Map.foldrWithKey (\b bs acc -> acc || (Set.notMember b b1s && Set.member b2 bs)) False map

manyFall :: Sup -> [Block] -> [Block] -> [Block] -> [Block]
manyFall sup bs = memoize2 go
  where
    go fl [] = fl
    go fl checking =
      let csup = (concatMap Set.toList . mapMaybe (`Map.lookup` sup)) checking
          csup' = filter (not . otherSupport' sup (Set.fromList (fl ++ csup ++ checking))) csup
          fl' = nub $ fl ++ csup' ++ checking
       in go fl' csup'
