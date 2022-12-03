--- Day 18: Snailfish ---

import Text.Read (readMaybe)

main = do
  input <- map parseSnail . lines <$> readFile "input/18_Snailfish.input"
  print (homework input) -- Part 1: 4243
  print $ maximum $ magnitude . reduce <$> (Node <$> input <*> input) -- Part 2: 4701
  -- print $ maximum [magnitude (reduce (Node i i')) | i <- input, i' <- input, i /= i']

data Tree = Node Tree Tree | Leaf Int
  deriving (Eq)

parseSnail :: String -> Tree
parseSnail s = Node s1'' s2''
  where
    s' = drop 1 (init s)
    ps =
      scanl
        ( \acc x -> case x of
            '[' -> acc + 1
            ']' -> acc - 1
            _ -> acc
        )
        0
        s'
    [(p, _, _)] = filter (\(i, b, c) -> b == 0 && c == ',') $ zip3 [0 ..] ps s'
    (s1, s2) = splitAt p s'
    s1' = s1
    s2' = drop 1 s2
    s1'' = case readMaybe s1' :: Maybe Int of
      Just x -> Leaf x
      Nothing -> parseSnail s1'
    s2'' = case readMaybe (drop 1 s2) :: Maybe Int of
      Just x -> Leaf x
      Nothing -> parseSnail s2'

homework :: [Tree] -> Int
homework (t : ts) = magnitude $ foldl (\acc x -> reduce (Node acc x)) t ts

magnitude :: Tree -> Int
magnitude (Leaf n) = n
magnitude (Node l r) = (magnitude l * 3) + (magnitude r * 2)

reduce :: Tree -> Tree
reduce t =
  case explode t of
    Right t' -> reduce t'
    Left t' -> case split t of
      Right t'' -> reduce t''
      Left t'' -> t''

split :: Tree -> Either Tree Tree
split = applyLeft split'

applyLeft :: (Int -> Either Tree Tree) -> Tree -> Either Tree Tree
applyLeft f (Leaf n) = f n
applyLeft f (Node l r) =
  case applyLeft f l of
    Right x -> Right $ Node x r
    Left x -> case applyLeft f r of
      Right x' -> Right $ Node x x'
      Left x' -> Left $ Node x x'

split' :: Int -> Either Tree Tree
split' n
  | n >= 10 = Right $ Node (Leaf (n `div` 2)) (Leaf ((1 + n) `div` 2))
  | otherwise = Left (Leaf n)

data Direction = L | R

type Path = [Direction]

explode :: Tree -> Either Tree Tree
explode t =
  case explode' t [] of
    Left x -> Left x
    Right (t, p, (l, r)) -> Right $ applyTree (+ r) (applyTree (+ l) t (extremo L t p)) (extremo R t p)

applyTree :: (Int -> Int) -> Tree -> Path -> Tree
applyTree f n@(Node _ _) [] = n
applyTree f (Leaf n) [] = Leaf (f n)
applyTree f (Node l r) (p : ps) =
  case p of
    L -> Node (applyTree f l ps) r
    R -> Node l (applyTree f r ps)

explode' :: Tree -> Path -> Either Tree (Tree, Path, (Int, Int))
explode' leaf@(Leaf _) path = Left leaf
explode' node@(Node (Leaf l) (Leaf r)) path
  | length path > 3 = Right (Leaf 0, path, (l, r))
  | otherwise = Left node
explode' (Node l r) path =
  case explode' l (path ++ [L]) of
    Right (t, path', (a, b)) -> Right (Node t r, path', (a, b))
    Left x -> case explode' r (path ++ [R]) of
      Right (t, path'', (a', b')) -> Right (Node l t, path'', (a', b'))
      Left x' -> Left $ Node x x'

extremo d t [] = []
extremo d t p =
  let p' = init p
      blocked = last p
   in case traza (gotoTree t p') d blocked of
        Nothing -> extremo d t p'
        Just path -> p' ++ path

traza :: Tree -> Direction -> Direction -> Maybe Path
traza _ L L = Nothing
traza _ R R = Nothing
traza (Node l _) L R = Just $ L : dumbo R l
traza (Node _ r) R L = Just $ R : dumbo L r

dumbo _ (Leaf _) = []
dumbo L (Node l r) = L : dumbo L l
dumbo R (Node l r) = R : dumbo R r

gotoTree :: Tree -> Path -> Tree
gotoTree t [] = t
gotoTree (Node l r) (p : ps) =
  case p of
    L -> gotoTree l ps
    R -> gotoTree r ps
