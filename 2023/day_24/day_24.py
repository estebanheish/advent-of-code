import sympy


class Line:
    def __init__(self, l):
        a, b = l.split(" @ ")
        self.x, self.y, self.z = [int(n) for n in a.split(",")]
        self.vx, self.vy, self.vz = [int(n) for n in b.split(",")]

        self.slope = self.vy / self.vx
        self.b = self.y - self.slope * self.x

    def intersection(self, other):
        if self.slope == other.slope:
            return None

        x = (other.b - self.b) / (self.slope - other.slope)
        y = self.slope * x + self.b

        if (self.vx > 0 and x < self.x) or (self.vx < 0 and x > self.x):
            return None

        if (self.vy > 0 and y < self.y) or (self.vy < 0 and y > self.y):
            return None

        if (other.vx > 0 and x < other.x) or (other.vx < 0 and x > other.x):
            return None

        if (other.vy > 0 and y < other.y) or (other.vy < 0 and y > other.y):
            return None

        return (x, y)


lines = [Line(l.strip()) for l in open(0).readlines()[:-1]]

L = 200000000000000
H = 400000000000000
p1 = 0
for i, la in enumerate(lines):
    for lb in lines[i:]:
        inter = la.intersection(lb)
        if inter:
            x, y = inter
            if L <= x <= H and L <= y <= H:
                p1 += 1

print("part 1 -> ", p1)

x, y, z, vx, vy, vz = sympy.symbols("x,y,z,vx,vy,vz")
eq = []
for i, l in enumerate(lines[:3]):
    t = sympy.symbols(f"t{i}")
    eq.append(sympy.Eq(x + vx * t, l.x + l.vx * t))
    eq.append(sympy.Eq(y + vy * t, l.y + l.vy * t))
    eq.append(sympy.Eq(z + vz * t, l.z + l.vz * t))

rock = sympy.solve(eq)[0]
print(rock[x] + rock[y] + rock[z])
