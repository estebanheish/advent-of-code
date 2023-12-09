HS = [list(map(int, l.split())) for l in open(0).readlines()]


def e(ns, d):
    if all(n == 0 for n in ns):
        return 0

    k = ns[0]
    l = []
    for n in ns[1:]:
        l.append(n - k)
        k = n

    return l[-1] + e(l, d) if not d else l[0] - e(l, d)


print("part 1 ->", sum(h[-1] + e(h, False) for h in HS))
print("part 2 ->", sum(h[0] - e(h, True) for h in HS))
