# Day 17

Do a tetris-like simulation, where you need to see how these pieces fall when
moving left or right one by a given pattern.

Part a) 2022 pieces drop, how tall is the tower

Part b) 1_000_000_000_000 pieces drop. How tall is the tower?

Part a is relatively straight forward, although a little time consuming to code.

Part b cannot naively be done the normal way, however I think it should be
possible to work it out based on some other mechanism. The sample input is 10091
directions long, so what we need to do is work out is a point in time where we
have exactly started a new shape (as a bar) and exactly started the direction
array from scratch. At this point in time, hopefully the height of the tower
follows a deterministic pattern, so we can multiply by its current height, then
simulate the remainder.

I'm not sure on this though, some problems:
a) There's no guarantee that the bar that we drop is going to land perfectly on
top of the previous piece. This might be okay, so long as the next piece lands
on the bar, and not on the previous piece also.
