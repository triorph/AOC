### Day 12

part a:
Given a set of edges, find all paths from start -> end, following the rule that nodes with lower-case names can only be visited once.

part b:
Given the same set of edges, how many paths if 1 lower-case name node can be visited twice.

Mostly happy with my current solution, but it seems to be the first one where the time taken has dragged. I wonder what optimisations I can make, preferably
at a higher level, that fix this. Once rust uses --release it still goes down to 2seconds, so not too bad; but not great either.
