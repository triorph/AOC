### Day 14

Polymer chain generation.

Given a starting polymer, and the ability to add a new element inbetween when 2 are found, create a new polymer.

Part a)
Perform 10 iterations, then count the histogram of polymer types, and subtract the max from the least

part b)
Do the same, but after 40 iterations.

Day b threw a wrench in my plans, because I did it the dumb way at first for part a. Then I started doing it the "right way" but all my unit tests were wrong, and
I had made some dumb assumptions about the rule for making maps (1. that we could figure out the element count at the end without having to keep track of it on
the way, and 2. that we don't have to subtract the pair count that we've just turned into 2 new pairs). The 2 iteration test quickly found there was an issue,
and after that all was fixed, but the overall code structure is an ugly mess. I especially don't like having to use "or_insert_with(::blank)" just to make the
hash_map work, but maybe I just have to.
