grid = [l.strip() for l in open(0).readlines()[:-1]]


def tilt(grid):
    out = []
    for l in list(zip(*grid)):
        trd = "#".join(
            [a.count("O") * "O" + a.count(".") * "." for a in "".join(l).split("#")]
        )
        out.append(trd)

    return list(zip(*out))


print(
    "part 1 ->",
    sum([l.count("O") * (len(grid) - i) for i, l in enumerate(tilt(grid))]),
)


def r90(g):
    return [list(r) for r in zip(*g[::-1])]


def hashg(g):
    return "\n".join(["".join(l) for l in g])


def cycle(g):
    g = tilt(g)
    g = tilt(r90(g))
    g = tilt(r90(g))
    g = tilt(r90(g))

    return r90(g)


r = {}
i = 0
t = 1_000_000_000
while i < t:
    grid = cycle(grid)
    h = hashg(grid)
    if h in r:
        i += (i - r[h]) * ((t - i) // (i - r[h]))
    r[h] = i
    i += 1

print(
    "part 2 ->",
    sum([l.count("O") * (len(grid) - i) for i, l in enumerate(grid)]),
)
