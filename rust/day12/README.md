### Day 12

part a:
Given a set of edges, find all paths from start -> end, following the rule that nodes with lower-case names can only be visited once.

part b:
Given the same set of edges, how many paths if 1 lower-case name node can be visited twice.

Mostly happy with my current solution, but it seems to be the first one where the time taken has dragged. I wonder what optimisations I can make, preferably
at a higher level, that fix this. Once rust uses --release it still goes down to 2seconds, so not too bad; but not great either.

I have made some naive attempts to fix the speed issue, which have had minor but not significant improvements. The debug version takes around 52 seconds now.

The main consideration I have left, is whether or not we can memoize remaining path extensions from given node points, but that would assume the
list of small caves visited is part of that memoization, which is complicated and seems unlikely to bear fruit.

On the plus side, I got to do my first real use of lifetimes to make something work, so still learning something new with Rust every day, and who knows
how slow this would've run in Python.
