# day 14

Simulate the fluid dynamic of sand falling into a chamber.

Part 1: No bottom, so fill up all the specified pieces, then error out when
something exceeds the bounds of the game
Part 2: An infinite bottom, so fill up all the pieces, then form a pyramid up to
the starting point and stop when nothing more can be added

Overall, solving this was relatively easy, and despite having covid I didn't hit
any real gotchas and was able to get the real answer within about an hour. I did
have to adjust the "infinite wall" aspect until the clause was "point.y < max_y

- 1" which I dont' quite understand based on the spec, but it works for all the
  data so I'll stick with it.

This one was by far the first to hurt my execution time. in release mode running
all the projects took 0.21 seconds, and then went up to 6.51 seconds. Someone on
the atlassian slack suggested that I should convert the walls into their
hashsets points. Someone on reddit pointed out that the walls have significant
duplicated entries, so the hashset points should solve that as well. Furthermore
someone on reddit suggested using a path searching algorithm, so that instead of
starting from 500,0 each time, you can start from the previous path's last good
entry. Combining these 3 (mostly 2) ideas I was able to get the cargo run
--release time of the full project back down to 0.22 seconds, making it probably
less of a problem than some of the other older ones like day12 or something
