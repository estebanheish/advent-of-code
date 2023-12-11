G = [list(l.strip()) for l in open(0).readlines()[:-1]]

h, v = [], []
for i, l in enumerate(G):
    if all(c == "." for c in l):
        h.append(i)
    if all(G[j][i] == "." for j in range(len(G[0]))):
        v.append(i)

galaxies = [(i, j) for i, l in enumerate(G) for j, c in enumerate(l) if c == "#"]


def distance(a, b, s):
    ch = len([n for n in h if min(a[0], b[0]) < n < max(a[0], b[0])])
    cv = len([n for n in v if min(a[1], b[1]) < n < max(a[1], b[1])])
    return abs(a[0] - b[0]) + (s * cv) + abs(a[1] - b[1]) + (s * ch)


pairs = [
    (galaxies[i], galaxies[j])
    for i in range(len(galaxies))
    for j in range(i + 1, len(galaxies))
]

print("part 1 ->", sum([distance(a, b, 2 - 1) for a, b in pairs]))
print("part 2 ->", sum([distance(a, b, 1_000_000 - 1) for a, b in pairs]))
