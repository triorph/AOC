### Day 24

Run a interpretive language (with no jumps thankfully) on 6 registers.

Since we have to find the Z value after all the numbers have been added, we get into a problem that we have to
manually test 10^14 numbers, going down. The problem itself gives a hint, calling the algorithm MONAD, which
is a term I'd heard in casual reading but never properly looked into (a haskell thing i think?). Seems to be
a way of writing an algebraic expression (similar to what I did for the SnailNumbers).

What we need to do is, create variables W1 - W14, and then write each register as an algebraic expression
for each.
Small rules of thumb:
multiply by 0 changes the algebraic expression to 0.
Adding 0 just assigns the other algebraic expression
multiply by 1 just assigns the other side
anything by 2 literals just does the literal check
0 divide by something is 0
0 modulo something is 0
equality operation replaces the algebraic expression with 1 or 0 (if we can find out).

What remains of the problem:
Equality checks are growing massive as we move forward. What we want to do, is reduce the numbers that give a 1 for each equality, and the numbers
that give a 0, then "split the universe" along these lines and check which of them end up with z=0 for the final result.

Part a: Find the highest number that successfully passes the test.
