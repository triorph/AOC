### Day 23

Find the shortest distance taken to move amphipods from stacks into corridors and to their correct stacks.

I've used a DFS search to find an answer, with early aborting of any movements that have taken more energy than the current path. Still runs
really slowly (4 minutes for the full problem, with --release)

Part a:
Find the shortest path for the given stacks

Part b:
Insert some not very helpful amphipods in the middle of the stacks, and then find the shortest path again.
