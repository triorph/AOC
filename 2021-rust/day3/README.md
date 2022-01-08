### Day 3

Part a)
Find the most common bits and least common bits in some binary data (represented as binary strings in the data)

Part b)
Find the most common bits and filter out everything that doesn't match, then refine. Same with least common bits.

Handled the "and keep going" part of the string really poorly for day3b, causing it to take a lot longer than I should.
Tests are okay, but I could do with improving some parts of it to be more "Rust"-like. Actually checking the input data
is a binary format, and pushing them into an array of enum types. I think my handling of passing array types around
could definitely be better here too.
