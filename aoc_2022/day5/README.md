# Day 5

The actual challenge on this not too hard, but parsing the data was a bit of a
chore. Notably I had to read horizontally and then transpose to get the data the
way that you want it for this. Then doing some stack pop / push stuff, where I
put in a temporary buffer the second time. I left the stack set as a hashmap and
made a trait for handling what I want to do. I think I actually quite like this
approach, although its not the best.
