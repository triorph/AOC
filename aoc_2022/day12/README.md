# Day 12

Find the shortest path to a destination.

part a: From a given starting point, what is the shortest path
part b: From a given set of starting points, what is the length of their
shortest paths.

Paths must follow the rules of only going up 1 elevation at most at a time.

This one is basically Dijkstra's algorithm the challenge. Originally I quickly
did it DFS and that did not work at all. I can never remember the exacts of
Djikstra's algorithm, but last year I came up with my own "spin" of it, which
essentially boils down to the same thing. Have a map of costs for all nodes on
the space, and do a breadth first search. At each point in a breadth first
search, compare to the existing cost, if less then set the new cost, otherwise
abort early. Do the usual BFS stuff at this point (work out the next path, add
it to the end of the queue with its current cost) and wait until the queue is
empty. By the end you should have the correct cost to get to your destination in
its cost-node spot.

I did get a little stuck on this, as I had the answer 470 and couldnt figure out
why it wasn't working. I went on reddit and borrowed someone else's code and
they also said I had 470 so whatever I was doing wrong was a common gotcha. The
next code I borrowed said 468. I printed out the full path taken next to the
heights, and measured through it and everything seemed hunky dory. It wasn't
until I was complaining to Kiara about how my covid brain wasn't feeling smart
enough to work out the problem that I realised I had set the height of the end
point to 26 rather than the 25 it should be. At 25 we didn't have to take 2
extra steps at the end to find our way onto an existing 25 first, so it all just
worked. Surprisingly there must be some input data where it doesn't care about
this, as was given by the python example on reddit not helping me out.

The day b was pretty trivial once day a was working.
