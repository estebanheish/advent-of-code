from functools import reduce

with open("input.txt") as f:
    trees = [ list(map(int, l)) for l in f.read().splitlines()]

    l = len(trees)
    v = l*4-4

    s = [];
    for r in range(1,l-1):
        for c in range(1,l-1):
            d = lambda x, y, b: [ (trees[i][c] if b else trees[r][i]) < trees[r][c] for i in range(x,y)] # True is vertical 
            ntrees = [d(0,r, True)[::-1], d(r+1, l, True), d(c+1, l, False), d(0, c, False)[::-1]] # up, down, right, left

            if any(map(all, ntrees)):
                v += 1

            s.append(reduce(lambda x, y: x*y, [ g.index(False)+1 if False in g else len(g) for g in ntrees ]))

    print(v, max(s))
