with open("./input.txt") as f:

    knots = [(0,0) for _ in range(10)]
    paths = [set() for _ in range(10)]

    y = {'U': 1, 'D': -1, 'L': 0, 'R': 0}
    x = {'U': 0, 'D': 0, 'L': -1, 'R': 1}
    move = lambda p, m: (p[0]+x[m], p[1]+y[m])
    v = lambda t, d = 25: [print("".join(["#" if ((x,y) in t) else "." for x in range(-d, d)])) for y in range(d, -d, -1)]
    n = lambda n: n//2 if abs(n) > 1 else n

    for motion in f.read().splitlines():
        m, r = motion.split()
        for _ in range(int(r)):
            knots[0] = move(knots[0], m)
            for i in range(1, 10):
                xd, yd = knots[i-1][0]-knots[i][0], knots[i-1][1]-knots[i][1]
                if max(abs(xd), abs(yd)) > 1:
                    knots[i] = (knots[i][0]+n(xd),knots[i][1]+n(yd))
                paths[i].add(knots[i])
    
    print(len(paths[1]), len(paths[9]))
