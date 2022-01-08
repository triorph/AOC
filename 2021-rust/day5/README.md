## Day 5

Vents are given as 2D points from a to b. We are mapping all integer points between a to b, assuming a and b follow some simple rules.

Part a:
Considering only horizontal/vertical points (x1 = x2) or (y1 = y2), find how many points on the grid have a count greater than 1.

Part b:
Same as part a, but also consider diagonal points abs(y2 - y1) == abs(x2 - x1)

I used a hashmap to store the count of points, rather than worry about building up some faux grid. This acts as a "sparse" result.
