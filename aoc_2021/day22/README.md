### Day 22

#### Part a:

Find the final volume of a bunch of on and off cubes being applied within a 100x100x100 area.

The "easy" but slow way to do this is to just simulate 1 million pixels via a hashmap/index and then give them a yes/no, but almost certainly
part b is going to wreck us if we do that. Then again, maybe a quick and dirty solution is better just to see what part b is like.

For a "better" solution, I think we make contiguous volumes with a given size.
When creating new "ons", we subtract the existing from it.
When creating new "offs", we subtract from existing.
If any volumes end up at size 0, we remove them from the stack.

How should subtract work?
first split by x, into a, b and c elements (which may be empty, and can be early terminated)
Then for each x split, split by y, into ad, ae, af, bd, be, bf, cd, ce, cf elements (which may be empty, and can be early terminated)
Likewise for z, into adg, adh, .... cfi elements. For a total of 27 possible (if the "off" region is in the middle of an existing).
Of these 27, the "middle" one gets deleted.

In theory we could merge subsets into larger ones if they came along that way, but I think what we have is still fine, as we will still
always be much less than a million voxels to deal with.

#### Part b:

Thankfully this was just doing the same thing but without the -50..50 limit, so for everything. It all just worked.

#### Notes:

Got massively flubbed on many major points, even though the core idea of
what I was doing seemed fine. The final piece was that apparently I was failing to check overlaps
correctly. Once that was fixed it finally all just fell into place.

Another issue is I misread the problem to not care about data outside of -50..50, instead of ignoring all data that has points outside of that, even
if they have points within it too. This wasn't actually a problem for the test data.

One thing I did do was look at someone else's rust code to get some
ideas about what the day_a_larger_steps should actually be, so I could
write tests to see waht was going wrong. This didn't work super well,
as I was kind of stuck in the problem, and ended up replacing the
subtract on the new point, to the subtract on old points (thus
doing the same thing each time, then adding a new one after always).
Even worse, their code had a bug that made it fail on the example test_data from the problem.
I had to find and fix that bug to get the data at each step for the test_data, and run unit tests to
check the output count at each step.

The other change I did, was to always subtract from the existing data, then if there's a new cuboid, just add it as is.
Somehow this made everything fall into place, as it broke the basic unit tests in a way that doing it in reverse didn't, even though
they shared the same problem with overlaps not working.

One benefit of checking out this other person's code, is I finally got to see how properly modularised rust code looks, so
now I know how to move stuff into separate files and write tests just for those things.
