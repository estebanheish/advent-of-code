from functools import reduce

def follow(a, b):
    (ax,ay),(bx,by) = a,b
    out = []

    if max(abs(ax-bx), abs(ay-by)) <= 1:
        return out

    if by > ay:
        out.append('U')

    if by < ay:
        out.append('D')

    if bx > ax:
        out.append('R')

    if bx < ax:
        out.append('L')

    return out

with open("./input.txt") as f:

    trains = [ (0,0) for _ in range(10) ]
    paths = [ set() for _ in range(10) ]

    y = {'U': 1, 'D': -1, 'L': 0, 'R': 0 }
    x = {'U': 0, 'D': 0, 'L': -1, 'R': 1 }
    move = lambda p, m: (p[0]+x[m], p[1]+y[m])
    v = lambda t, d = 25: [print("".join(["#" if ((x,y) in t) else "." for x in range(-d, d)])) for y in range(d, -d, -1)]

    for motion in f.read().splitlines():
        m, r = motion.split()
        for _ in range(int(r)):
            trains[0] = move(trains[0], m)
            for i in range(1, 10):
                p = reduce(move, follow(trains[i], trains[i-1]), trains[i])
                trains[i] = p
                paths[i].add(p)

    print(len(paths[1]), len(paths[9]))
