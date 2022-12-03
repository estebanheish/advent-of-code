--- Day 17: Trick Shot ---

import Control.Monad.State

main = mapM_ print $ sequence [higherShot, length] (tira ((217,240),(-126,-69)))
-- Part 1: 7875
-- Part 2: 2321
    
type Pos         = (Int, Int)
type Area        = (Pos, Pos)
type Velocidad   = Pos
type Trayectoria = [Pos]

data Tiro = Tiro Velocidad Trayectoria
    deriving (Show)

higherShot :: [Tiro] -> Int
higherShot = foldl f (-1000) 
    where f h (Tiro v trayectoria) = if nh > h then nh else h
            where nh = maximum (snd <$> trayectoria)

tira :: Area -> [Tiro]
tira a@(x,y) = [ 
  Tiro (vx,vy) trayectoria'
  | vx <- [-x'..x'], 
    vy <- [-y'..y'], 
  let trayectoria' = trayectoria a (vx,vy),
  any (converge a) trayectoria' ]
  where f (a,b) = maximum [abs a, abs b]
        x' = f x
        y' = f y

converge :: Area -> Pos -> Bool
converge ((ax,ax'),(ay,ay')) (x,y) =
  x >= minimum [ax,ax'] && 
  x <= maximum [ax,ax'] &&
  y >= minimum [ay,ay'] &&
  y <= maximum [ay,ay']

trayectoria :: Area -> Velocidad -> Trayectoria
trayectoria (x,y) v = takeWhile j $ evalState (sequence (repeat p)) (v,(0,0))
  where p = do ((vx, vy), (px, py)) <- get
               let p = (px+vx, py+vy)
                   v = (vx', vy-1)
                   vx' = case compare vx 0 of 
                           EQ -> vx
                           LT -> vx+1
                           GT -> vx-1
               put (v, p)
               return p
        j (px, py) = x' - abs px >= 0 && py >= (-y')
            where x' = maximum $ abs <$> [fst x, snd x]
                  y' = maximum [abs (fst y), abs (snd y)]
                
