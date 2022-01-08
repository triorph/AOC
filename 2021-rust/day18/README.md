### Day 18

Snailnumbers are a weird form of arithmetic with a bunch of horrible rules. We form layers of tuples, and go
through doing a reduce action over and over. A reduce action is trying an explosion, and if none happens, trying a split. If no split happens either, then the
reduction is complete.

An explosion is any literal-pair tuple (e.g. [a, b] for integers a and b) gets replaced with a 0, and a gets added to the next literal value to the left, and b gets
added to the next literal value to the right. If its the left-most value being exploded, then the left value is discarded. Likewise, right-most value being exploded then
right-most value is discarded.

A split is turning any number >= 10 into a left and a right tuple, such that left = number / 2 rounded down, and right = number / 2 rounded up.

We can add 2 snailnumbers a + b, by forming a pair [a, b] and then reducing.

a + b + c + d means we have to do reduce([reduce([reduce([a,b]),c]),d]), e.g. (((a + b) + c) + d), or just performing pair + reduce on each pair as they come in, from top to bottom.

Each snailnumber also has a magnitude, which is 1 _ literal, or for a tuple: 2 _ left + 3 \* right

Part a: Sum all snailnumbers in the input, and return the magnitude of the final result
Part b: For any 2 snailnumbers (and in either order), find the sum() whose magnitude is the largest

### Notes:
Have had a lot of trouble with this one; specifically on making the explosion work. I found Rust's mutability rules to be burdensome here, although
perhaps I just don't understand them well. I couldn't find a way to replace the contents of the Box inside the Enum with a new value; but that's what
I want to explore next. For now, I'm generating a new copy of the SnailNumber on the fly as we explode, either cloning what we already have if no
explosion is necessary, or replacing with the new values from the explosion. I have an ExplosionState variable that helps us keep track of what things
we need to be doing as we copy/explore the snail-number tree.

I'm going to commit what i have, then go back to trying to make this work as a mutable SnailNumber object.

Note: I went through some iterations with this, but I think DerefMut ended up being the key variable, which can be short-handed with "&mut *self" in the match
expression.
