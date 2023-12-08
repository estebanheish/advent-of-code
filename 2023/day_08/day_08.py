from math import lcm

INS, ms = open(0).read().strip().split("\n\n")
M = {}
for m in ms.splitlines():
    k, vs = m.split(" = ")
    vl, vr = vs.replace("(", "").replace(")", "").split(", ")
    M[k] = (vl, vr)


def f(s="AAA", f=lambda x: x != "ZZZ"):
    c = 0
    while f(s):
        l, r = M[s]

        if INS[c % len(INS)] == "L":
            s = l

        if INS[c % len(INS)] == "R":
            s = r

        c += 1
    return c


print("part 1 ->", f())
print(
    "part 2 ->",
    lcm(
        *[f(s=m, f=lambda x: not x.endswith("Z")) for m in M.keys() if m.endswith("A")]
    ),
)
