from collections import defaultdict

grid = [list(l.strip()) for l in open(0).readlines()[:-1]]
LR = len(grid)
LC = len(grid[0])
START = (0, 1)
END = (LR - 1, LC - 2)
NS = lambda r, c: [
    (r, c)
    for (r, c) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
    if r >= 0 and c >= 0 and r < LR and c < LC and grid[r][c] != "#"
]
D = {
    ".": [(-1, 0), (1, 0), (0, -1), (0, 1)],
    ">": [(0, 1)],
    "<": [(0, -1)],
    "^": [(-1, 0)],
    "v": [(1, 0)],
}
NSD = lambda r, c: [
    (r + rr, c + cc)
    for (rr, cc) in D[grid[r][c]]
    if r + rr >= 0
    and c + cc >= 0
    and r + rr < LR
    and c + cc < LC
    and grid[r + rr][c + cc] != "#"
]

bifurcaciones = [START, END] + [
    (r, c)
    for r in range(LR)
    for c in range(LC)
    if len(NS(r, c)) >= 3 and grid[r][c] != "#"
]


def find_next_bif(l, p, d, seen):
    if p in bifurcaciones:
        return (p, d)

    np = [p for p in l(p[0], p[1]) if p not in seen]
    if np:
        return find_next_bif(l, np[0], d + 1, seen | {p})
    else:
        return None


def create_graph(l):
    graph = defaultdict(list)
    for r, c in bifurcaciones:
        for p in NS(r, c):
            t = find_next_bif(l, p, 1, set([(r, c)]))
            if t:
                graph[(r, c)].append((t[0], t[1]))
    return graph


graph1 = create_graph(NSD)
graph2 = create_graph(NS)


def bfs(g, p, d, seen):
    if p == END:
        yield d

    for p, nd in g[p]:
        if p in seen:
            continue
        yield from bfs(g, p, d + nd, seen | {p})


print(max(bfs(graph1, START, 0, set([START]))))
print(max(bfs(graph2, START, 0, set([START]))))
