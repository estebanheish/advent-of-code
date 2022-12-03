def rank(c):
    if c.islower():
        return ord(c) - ord("a") + 1
    else:
        return ord(c) - ord("A") + 27


f = open("../input.txt").readlines()

out = 0
for e in f:
    e = e.strip()
    l = len(e) // 2

    (s,) = set(e[:l]) & set(e[l:])

    out += rank(s[0])

print(out)

out = 0
for i in range(0, len(f), 3):
    x = f[i].strip()
    y = f[i + 1].strip()
    z = f[i + 2].strip()

    (s,) = set(x) & set(y) & set(z)

    out += rank(s)

print(out)
