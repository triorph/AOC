# Day 8

Forest analyzer

Trees have different heights, so thus block the view of further trees.
Part a) How many trees can be seen by looking directly up/down/left/right from
the edges
Part b) If we give trees a beauty score based on how many trees they can see in
each direction, which tree has the highest beauty score

Thursday is the day I always have to do these the next day, which isn't great
for my score but eh.

Was mostly straight-forward, in that my initial strategy for how to achieve this
just worked, but I did play around with leaving the data in iterators vs not
doing that. I think I could still use iterators, but there was a brief period of
time where I was comparing to the length for the day_a score (when rewriting to
work for day_b) and that didn't make sense, but because the trees upon path gets
screwey near the ends I had to abandon that, so I might rewrite it back to all
stay as iterators (even if the typing is a bit annoying)
