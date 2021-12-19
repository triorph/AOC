### Day 19

We need to find all beacons within some scanners. Beacons are given to us relative to scanners, as x,y,z coordinates.

A scanner has an x,y,z offset relative to 0,0,0, but also a rotation offset where there are 24 different rotation options.

Rotation options are: can rotate 90 degrees around any of the 3 axis. so 0, 90, 180 or 270 around x, plus around y, plus around z.
Some of these must be equivalent, because 4 _ 4 _ 4 = 64, which is a lot more. Haven't got it quite right in my head, but I think
4 _ 3 _ 2 = 24 so its probably something related to that.

if we have a point x=1, y=2, z=3, and we are rotating around z, then z stays the same for all, and x,y follow simple rules:

    rot:0, x = 1, y = 2;
    rot: 90, x = 2, y = -1;
    rot 180, x = -1, y = -2;
    rot 270, x = -2, y = 1,
    rot 360, x = 1, y = -2,

So each step of 90 requires us to do (x, y) = (y, -x);
A similar rule can be done for around y: (x, z) = (z, -x);, and for around x: (y, z) = (z, -y);

So how do we eliminate this down to 64?

Let's start by running all the possible rotations on a 1,2,3 point and find out which map to a duplicate. I ran a test that does all possible 4 rotations around
each axis, then removes duplicates and finds the correct mapping. I put these unique rotations in the source code as a constant.

#### Part a:

Find how many unique beacons there are

#### Part b:

Find the maximum manhattan distance between any scanners.

Part b should've been a breeze, but my answer was working for the test cases but not the real thing. I copied someone's solution from reddit to see if they
got a different answer, which they did, but their code was not great so took me a bit of time to dig into and see what was going on.
First thoughts were that the rotations were different, so took the example 1, 2, 3 input and sorted it, and it was the same as mine.
Then the points were clearly different, so I changed my algorithm to do things in the same order they did (which was wrong btw, first scanner is the reference not the last).
And then I ended up getting the same points too!

Finally I realised I was doing the manhattan distance wrong. sum of abs() of each points, not abs of sum of all points. I spent hours on this little fiddly thing
and the problem was so trivial. With a sleep cycle in between I went from Rank 1126 on part a to Rank 5647 on part b. (Both actually not that bad).

On the flip side, I think my approach to coding this was really good, and the final code without any real passes is pretty good, with only some minor changes needed to tidy it up. (Although a little worse after all the debugging)

The solution isn't the fastest, but runs faster than the person I copied off (not exactly sure why), so I'm happy with that.
