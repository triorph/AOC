### Day 15

Find the shortest path through a weighted node list

No time today to actually work on it, but my initial thoughts are that we do a BFS and at each node, assign a "current best distance to it", and cancel any paths
where we reach a node where the best distance is better than our current time.
