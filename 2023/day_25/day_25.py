import networkx as nx
from math import prod

G = nx.Graph()
for l in open(0).readlines()[:-1]:
    s, dest = l.strip().split(": ")
    for d in dest.split(" "):
        G.add_edge(s, d)

G.remove_edges_from(nx.minimum_edge_cut(G))
print(prod([len(c) for c in nx.connected_components(G)]))
