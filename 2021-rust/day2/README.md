Day 2,
parse instructions to move a submarine forward.

Day 2a:
forward moves you forward, down moves you down, up moves you up.

Day 2b:
up/down adjust your tilt. Forward moves you forward, AND adjusts your depth by aim/tilt and how far forward you are going.

Solution info:
Used Parsing Expression Grammer (PEG) to handle the input file, creating an enum that holds the type of movement and how far it is.
This movement can be reused for both problems, with a "match" expression that figures out how to create a new Location for the day type.
