# Day 15

Beacons and sensors. Beacons always select the closest sensor (or vice verca) so
we can be sure that if there's a distance of 15 between 2, then the circle
(diamond really in manhattan distance space) of the two does not have another
sensor.

Part a, find all points at y=2mil that cannot be beacons
Part b, find the only point between 0 and 4mil (x and/or y) that cannot be a
beacon.

For part b, I think we need to be a bit smarter about how we search the space.
I'm thinking that we only check all the points 1 distance further than the given
manhattan distance. (e.g. if the sensor is 0,0 and the beacon is 0.2, then check
0,3; 1,2; 2,1 etc..). Since we know there can only be 1 answer the real one
should be one of these points, and this significantly reduces the search space
we have to worry about.
