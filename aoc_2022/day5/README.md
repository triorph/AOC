# Day 5

Follow some stack moving instructions then print out the top value on each stack

Part a) Moving n items means move each item n at a time 1 by 1 (essentially
reversing them)
Part b) Moving n items means moving all n at once

The actual challenge on this not too hard, but parsing the data was a bit of a
chore. Notably I had to read horizontally and then transpose to get the data the
way that you want it for this. Then doing some stack pop / push stuff, where I
put in a temporary buffer the second time. I left the stack set as a hashmap and
made a trait for handling what I want to do. I think I actually quite like this
approach, although its not the best.
