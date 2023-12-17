grid = [list(l.strip()) for l in open(0).readlines()[:-1]]
x = {"r": 1, "u": 0, "d": 0, "l": -1}
y = {"r": 0, "u": -1, "d": 1, "l": 0}
m = {
    "/": {"r": "u", "l": "d", "u": "r", "d": "l"},
    "\\": {"r": "d", "l": "u", "d": "r", "u": "l"},
    "|": {"d": "d", "u": "u", "r": "ud", "l": "ud"},
    "-": {"u": "lr", "d": "lr", "r": "r", "l": "l"},
}


def tiles_energized(start):
    energized = set()
    heads = [start]
    seen = set()
    while heads:
        r, c, d = heads.pop()

        energized.add((r, c))

        r += y[d]
        c += x[d]

        if r >= len(grid) or c >= len(grid[0]) or r < 0 or c < 0 or (r, c, d) in seen:
            continue

        p = grid[r][c]

        if p == ".":
            heads.append(((r, c, d)))
            continue

        for nd in m[p][d]:
            heads.append(((r, c, nd)))

        seen.add((r, c, d))

    return len(energized)


print(
    "part 1 ->",
    tiles_energized((0, 0, m[grid[0][0]]["r"] if grid[0][0] != "." else "r")),
)

t = [tiles_energized((0, i, "d")) for i in range(len(grid[0]))]
b = [tiles_energized((len(grid) - 1, i, "u")) for i in range(len(grid[0]))]
l = [tiles_energized((i, 0, "r")) for i in range(len(grid))]
r = [tiles_energized((i, len(grid[0]) - 1, "l")) for i in range(len(grid))]

print("part 2 ->", max(max(t), max(b), max(l), max(r)))
