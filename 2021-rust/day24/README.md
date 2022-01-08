### Day 24

Run a interpretive language (with no jumps thankfully) on 6 registers.

Since we have to find the Z value after all the numbers have been added, we get into a problem that we have to
manually test 10^14 numbers, going down. The problem itself gives a hint, calling the algorithm MONAD, which
is a term I'd heard in casual reading but never properly looked into (a haskell thing i think?). Seems to be
a way of writing an algebraic expression (similar to what I did for the SnailNumbers).

What we need to do is, create variables W1 - W14, and then write each register as an algebraic expression
for each.

Small rules of thumb:

- multiply by 0 changes the algebraic expression to 0.
- Adding 0 just assigns the other algebraic expression
- multiply by 1 just assigns the other side
- anything by 2 literals just does the literal check
- 0 divide by something is 0
- 0 modulo something is 0
- equality operation replaces the algebraic expression with 1 or 0 (if we can find out).

What remains of the problem:
Equality checks are growing massive as we move forward. What we want to do, is reduce the numbers that give a 1 for each equality, and the numbers
that give a 0, then "split the universe" along these lines and check which of them end up with z=0 for the final result.

The above was a good start, but after manually running the inputs I found several things.

1. the data keeps getting put into 26 \* left + right, and pulled through either div 26, or mod 26, which is equivalent to going
   (left, right), choose left, or choose right. This is all the div and mod are actually used for.
2. So with modulo, we can replace it with choose right (if possible).
3. With div, we can replace it with choose left (if possible), or even reduce to 0 if dividing by 26 and no multiplier available.
4. My input data was bung, some Vim copy/paste error turned a bunch of my 12s into 24s, so I probably could have just manually
   worked out the correct answer through the manual_input.txt exercise I did, if the numbers had been correct.
5. The "equality checks" don't actually require splitting. We always take the "yes" route if possible, although my code still handles the split.
6. Equality checks are always of the form Input(X) + B = Input(Y). So we just need to reduce the possibilities of the inputs for X and Y at
   each of these equality checks.

After solving the above, I was able to get the problem space to reduce down to a z of 0, and then finding the highest/lowest
was just a matter of getting the max/min of each of the possibilities.

Part a: Find the highest number that successfully passes the test.
Part b: Find the lowest number that successfully passes the test.
