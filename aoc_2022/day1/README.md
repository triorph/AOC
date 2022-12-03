# Day 1

Pretty basic, you just need to sum a some numbers and then find the largest,
followed by finding the sum of the 3 largest (which requires sorting).

I started this the next day (because NZ time isn't super conducive to working on
all of this) so I spent some time at the start setting up a multi-lib project
where each day is its own library, and is able to inherit some shared code. One
aspect is that we have a shared Trait for doing the calculations, and some nice
stuff for reading from file.
