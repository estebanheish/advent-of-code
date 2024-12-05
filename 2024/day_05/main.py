from collections import defaultdict
from functools import cmp_to_key

ordering_rules, updates = open(0).read().strip().split("\n\n")

rules = defaultdict(set)
for ordering_rule in ordering_rules.splitlines():
    a, b = ordering_rule.split("|")
    rules[a].add(b)

updates = list(map(lambda x: x.split(",") , updates.splitlines()))

part1 = 0
bad = []
for update in updates:
    before = set()
    for page in update:
        before.add(page)
        if len(rules[page].intersection(before)) > 0:
            bad.append(update)
            break
    else:
        part1 += int(update[int(len(update)/2)])

print(part1)

part2 = 0
for b in bad:
    s = sorted(b, key=cmp_to_key(lambda a, b: -1 if b in rules[a] else 1))
    part2 += int(s[int(len(s)/2)])

print(part2)
