# Day 3

The elves have some shared items in parcels, and we need to find what is common
between these parcels

Part a) Find the pairs and get their unique item's priority
Part b) Find groups of 3 and get their unique item's priority

Clearly a use case of hashsets + the intersection function, but I had a bit of
trouble with some of the basic collection handling in Rust, most notably quickly
building a hashset from a vector in one move, which I'm surprised didn't just
work. I did it the manually hard way and the rest of this wasn't too difficult
(although a bit ugly) and have spent a bit of time refactoring.

Some interesting refactors:

- Added a trait Priority that allows me to do the `get_priority` function for
  both `char` and even worse `Vec<Vec<HashSet<char>>>`, which means I could
  build up my group of group of parcels and just call `get_priority()` on them.
- Last minute, managed to get the vector to hashset working in 1 line.
