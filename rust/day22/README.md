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
