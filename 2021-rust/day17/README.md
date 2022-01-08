### Day 17

We fire probes off, that follow some newtonian-ish physics. Drag reduces x launch speed by 1 every step, to a minimum of 0, and gravity reduces y launch speed by 1 every step, to a minimum of -infinity. We have a given launch area, and have to find initial trajectories that have at least 1 step be within that area.

#### Part a

Find the shot that meets the target trajectory area but also reaches the highest Y value. What y value is it that it reaches?

#### Part b

Find the total number of trajectories that make it past the given goal

My "which values to check" function ended up being a bit hacky and not working, so I just scaled multipliers up until the result stopped changing.

At some point I'll do the math and figure out why this didn't work. This entire module is a bit ugly so a strong candidate for a rewrite.
