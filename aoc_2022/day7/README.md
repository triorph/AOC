# Day 7

Look at the pseudo directory/file structure of a disk
a) Work out the size of each folder, and get the sum of all whose size is less
than 100k
b) Work out the smallest folder we can delete to free up the required amount of
space

This one took me a while. Struggled to start with just with parsing the data
into enum types, which wasn't super difficult but was a bit tiresome. Once I
finally got there, I was failing to work with the input_data vs the test_data
because I was assuming that the base_directory was the same as the full
directory. For a bit I had assumed that there were symlink type things where
this would work like a graph rather than a tree, but actually there was nothing
of the sort, and the gotcha was that there were subdirectories with the same
name, but a different full path.

One issue with this is that my code fully worked for the test_data in the
examples, but was wrecked by this gotcha. Eventually I read in the slack channel
that someone had spent an hour stuck on this, (after I was stuck 2 hours on it).
