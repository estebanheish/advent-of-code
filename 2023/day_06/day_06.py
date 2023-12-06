input = open(0).readlines()
times = input[0].split(":")[1].split()
distances = input[1].split(":")[1].split()


def w(t, d):
    w = 0
    for h in range(t):
        if (h * (t - h)) > d:
            w += 1
    return w


part1 = 1
for t, d in zip(list(map(int, times)), list(map(int, distances))):
    part1 *= w(t, d)

print("part 1 ->", part1)
print("part 2 ->", w(int("".join(times)), int("".join(distances))))
