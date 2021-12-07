## Day 7

Crabs need to align themselves horizontally to save you from a hole.

Part a:
They present you a list of horizontal positions, find the optimal location for least movement, and then say how much all the crabs move in total.

Part b:
Same deal, but the distance they move get quadratically worse with each movement, so find a different optimal.
Exact formula worse is n + (n-1) + .. + 2 + 1, which is equivalent to n * (n + 1) / 2

note: I had some problems assuming the mean would be the correct solution, but it was not, and I couldn't figure out a real formula. Brute force got the answer without any real issue though.
