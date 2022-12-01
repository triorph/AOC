## Day 6

Monitoring lanternfish populations.

Lantern fish take 7 days to breed, where they immediately spawn a new one. Take 9 days if its a brand new lantern fish.

Can think of our population as a count of ages, from -2 to 7 (or in reverse, their time to breed of 8 to 0).

Iteration 1: [a, b, c, d, e, f, g, h]
Iteration 2: [b, c, d, e, f, g + a, h, a]

Day a) How many lantern fish are there after 80 days
Day b) How many lantern fish are there after 256 days

Apparently a lot of people had problems with this, but the simple model we used above makes both trivially easy to perform, with the only issue
being the size of the numbers growing beyond 32-bit / 64-bit.
