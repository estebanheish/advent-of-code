input = open(0).read().strip().split("\n\n")
parsed = [
    [[int(n) for n in ns.split()] for ns in l.split(":")[1].strip().split("\n")]
    for l in input
]


def mp(ms, t) -> int:
    for m in parsed[1:]:
        for [d, s, l] in m:
            if s <= t <= s + l - 1:
                t = d + (t - s)
                break
    return t


seeds = parsed[0][0]
print("part 1 ->", min([mp(parsed[1:], s) for s in seeds]))


def mp2(ranges, maps):
    mapped_ranges = []
    while ranges:
        s, e = ranges.pop()
        for [dest, src, l] in maps:
            il, ir = max(s, src), min(e, src + l)
            if il < ir:
                mapped_ranges.append((il - src + dest, ir - src + dest))
                if e > ir:
                    ranges.append((ir, e))
                if il > s:
                    ranges.append((s, il))
                break
        else:
            mapped_ranges.append((s, e))
    return mapped_ranges


seeds2 = [(seeds[i], seeds[i] + seeds[i + 1] - 1) for i in range(0, len(seeds), 2)]
[seeds2 := mp2(seeds2, map) for map in parsed[1:]]
print("part 2 ->", min(seeds2)[0])
