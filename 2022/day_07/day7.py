with open("./input.txt") as f:

    filesize = {}
    current_path = []
    for l in f.read().splitlines():
        if l.startswith("$ cd"):
            dir = l.split(" ")[2]
            if dir == "..":
                current_path.pop()
            else:
                current_path.append(dir + "/")
        else:
            if l[0].isdigit():
                n = int(l.split(" ")[0])
                for i in range(1, len(current_path)+1):
                    key = "".join(current_path[:i])
                    try:
                        filesize[key] += n
                    except KeyError:
                        filesize[key] = n
    
    print(sum([ v for v in filesize.values() if v <= 100000]))
    print(min([ v for v in filesize.values() if filesize["//"] - v <= 40000000]))
