from collections import deque

G = [l.strip() for l in open(0).readlines()[:-1]]


def fpipes(r, c):
    out = []
    if r > 0:
        if G[r - 1][c] in "|F7":
            out.append((r - 1, c))
    if r < len(G) - 1:
        if G[r + 1][c] in "|LJ":
            out.append((r + 1, c))
    if c > 0:
        if G[r][c - 1] in "-LF":
            out.append((r, c - 1))
    if c < len(G) - 1:
        if G[r][c + 1] in "-J7":
            out.append((r, c + 1))
    return out


F = deque()
seen = set()
main_loop = set()
for row in range(len(G)):
    for col in range(len(G)):
        if G[row][col] == "S":
            seen.add((row, col))
            main_loop.add((row, col))

            [F.append((r, c, 1)) for r, c in fpipes(row, col)]
            break


def a(x, y, n):
    if x >= 0 and y >= 0 and x < len(G) and y < len(G[0]) and (x, y) not in seen:
        F.append((x, y, n))


distances = []
while F:
    r, c, d = F.popleft()

    if (r, c) in seen:
        continue

    match G[r][c]:
        case "|":
            a(r - 1, c, d + 1)
            a(r + 1, c, d + 1)
        case "-":
            a(r, c + 1, d + 1)
            a(r, c - 1, d + 1)
        case "L":
            a(r - 1, c, d + 1)
            a(r, c + 1, d + 1)
        case "J":
            a(r - 1, c, d + 1)
            a(r, c - 1, d + 1)
        case "7":
            a(r + 1, c, d + 1)
            a(r, c - 1, d + 1)
        case "F":
            a(r + 1, c, d + 1)
            a(r, c + 1, d + 1)

    main_loop.add((r, c))
    distances.append(d)
    seen.add((r, c))


print("part 1 ->", max(distances))


NONLOOP = set()
for row in range(len(G)):
    for col in range(len(G[0])):
        if (row, col) not in main_loop:
            NONLOOP.add((row, col))


def ns(r, c):
    out = []
    if r > 0:
        out.append((r - 1, c))
    if r < len(G) - 1:
        out.append((r + 1, c))
    if c > 0:
        out.append((r, c - 1))
    if c < len(G[0]) - 1:
        out.append((r, c + 1))
    return out


def is_outside(r, c):
    if r == 0 or c == 0 or r == len(G) - 1 or c == len(G[0]) - 1:
        return True
    pipes = 0
    c = c + 1
    while c < len(G[0]):
        p = G[r][c]
        if p == "|":
            pipes += 1

        if p in "LS":
            c += 1
            while G[r][c] == "-":
                c += 1

            if G[r][c] in "7S":
                pipes += 1

        if p in "FS":
            c += 1
            while G[r][c] == "-":
                c += 1

            if G[r][c] in "JS":
                pipes += 1
        c += 1

    return pipes % 2 == 0


G2 = []
for r in G:
    G2.append(list(r))
for r in range(len(G)):
    for c in range(len(G[0])):
        if (r, c) in main_loop:
            G2[r][c] = G[r][c]
        else:
            G2[r][c] = "."
G = G2

outside = set()
while NONLOOP:
    pos = NONLOOP.pop()

    SEEN = set()
    F = deque()
    F.append(pos)

    while F:
        r, c = F.popleft()

        if (r, c) in SEEN:
            continue

        adj = [a for a in ns(r, c) if a not in main_loop]

        for a in adj:
            F.append(a)

        SEEN.add((r, c))

    if any(is_outside(r, c) for r, c in SEEN):
        outside = outside.union(SEEN)
        NONLOOP -= SEEN


print("part 2 ->", (len(G) * len(G[0])) - (len(outside) + len(main_loop)))
