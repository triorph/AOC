# Day 1

Elves are holding packages of calories
part a) Find the elf with the most calories
Part b) Find the total calories across the top 3 elves

Pretty basic, you just need to sum a some numbers and then find the largest,
followed by finding the sum of the 3 largest (which requires sorting).

I started this the next day (because NZ time isn't super conducive to working on
all of this) so I spent some time at the start setting up a multi-lib project
where each day is its own library, and is able to inherit some shared code. One
aspect is that we have a shared Trait for doing the calculations, and some nice
stuff for reading from file.

For part b, I rewrote the code to not do a sort, and instead do a manual binary
heap implementation. This allows it to go from O(nlogn) to O(n), which isn't
really that important but fun to play around with
