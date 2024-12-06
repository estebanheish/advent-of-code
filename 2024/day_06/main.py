import copy

grid = open(0).read().strip().splitlines()

def starting(grid):
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if grid[i][j] in "^><v":
                return grid[i][j], i, j
    raise Exception("")

d, r, c = starting(grid)
DS,RS,CS = d,r,c

seen = set()
while ((len(grid)-1) >= r >= 0) and ((len(grid[0]) -1) >= c >= 0):
    seen.add((r,c))
    match d:
        case '^':
            r -= 1
            if r >= 0 and grid[r][c] == '#':
                r += 1
                d = '>'
        case 'v':
            r += 1
            if r <= len(grid) -1 and grid[r][c] == '#':
                r -= 1
                d = '<'
        case '>':
            c += 1
            if c <= len(grid[0]) -1 and grid[r][c] == '#':
                c -= 1
                d = 'v'
        case '<':
            c -= 1
            if c >= 0 and grid[r][c] == '#':
                c += 1
                d = '^'

print(len(seen))

G = list(map(list, grid))

def ciclo(i, j):
    grid = copy.deepcopy(G)
    d, r, c = DS, RS, CS
    grid[i][j] = '#'
    seen = set()
    while ((len(grid)-1) >= r >= 0) and ((len(grid[0]) -1) >= c >= 0):
        if ((r,c),d) in seen:
           return True
        seen.add(((r,c), d))
        match d:
            case '^':
                r -= 1
                if r >= 0 and grid[r][c] == '#':
                    r += 1
                    d = '>'
            case 'v':
                r += 1
                if r <= len(grid) -1 and grid[r][c] == '#':
                    r -= 1
                    d = '<'
            case '>':
                c += 1
                if c <= len(grid[0]) -1 and grid[r][c] == '#':
                    c -= 1
                    d = 'v'
            case '<':
                c -= 1
                if c >= 0 and grid[r][c] == '#':
                    c += 1
                    d = '^'
    return False


part2 = 0
for i in range(len(G)):
    for j in range(len(G[0])):
        if ciclo(i,j):
            part2 += 1

print(part2)
