import Control.Monad (guard)
import Data.List (splitAt)
import Data.List.Split (splitOn)
import qualified Data.Set as S

--- Day 19: Beacon Scanner ---

main = do
  input <- parse <$> readFile "input/19_Beacon_Scanner.input"
  let (a, b) = solve (head input) [] input []
  print a -- Part 1: 472
  print $ maximum [abs (x - x') + abs (y - y') + abs (z - z') | m@(x, y, z) <- b, m'@(x', y', z') <- b, m /= m'] -- Part 2: 12092

parse i = ((\x -> read ("(" ++ x ++ ")")) <$>) . drop 1 . lines <$> splitOn "\n\n" i

rel n ss = ((0, 0, 0) : map (\(x', y', z') -> (-x + x', -y + y', -z + z')) (a ++ b), d)
  where
    (a, d@(x, y, z) : b) = splitAt n ss

orientaciones s = f s ++ f blanca ++ f azul ++ f amarilla ++ f naranja ++ f roja
  where
    f i = take 4 $ iterate ((\(x, y, z) -> (-z, y, x)) <$>) i
    x (x, y, z) = (x, z, -y)
    blanca = x <$> s
    azul = x <$> blanca
    amarilla = x <$> azul
    naranja = (\(x, y, z) -> (-y, x, z)) <$> s
    roja = (\(x, y, z) -> (y, -x, z)) <$> s

alinea a b = do
  r1 <- [0 .. length a - 1]
  let (a', (ax, ay, az)) = rel r1 a
  r2 <- [0 .. length b - 1]
  let (b', bd) = rel r2 b
  (o2, i) <- zip (orientaciones b') [0 ..]
  let s1 = S.fromList a'
      s2 = S.fromList o2
      both = S.toList $ s1 <> s2
      [(bx, by, bz)] = orientaciones [bd] !! i
  guard $ foldl (\acc x -> if S.member x s1 then acc + 1 else acc) 0 o2 > 3
  return ((\(x, y, z) -> (x + ax, y + ay, z + az)) <$> both, (-bx + ax, -by + ay, -bz + az))

solve m [] [] ds = (length m, ds)
solve m r [] ds = solve m [] r ds
solve m r (s : sc) ds =
  case alinea m s of
    ((m', d) : _) -> solve m' r sc (d : ds)
    [] -> solve m (s : r) sc ds
