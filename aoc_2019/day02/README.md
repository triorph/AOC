# Advent of Code 2019 Day 2

This is the dreaded INT code one I've heard so much about. Was wondering why people hated it soo much, because it didn't sound THAT complicated, but given that
its only day 2, I can see why it was disliked.

### Part a

Instructions are in position 0 + 4n, inputs are pointers located in 1+4n and 2+4n, and output is to be sent to the address pointed by 3+4n.
Possible instructions are: 1 - "add", 2 - "multiply" and 99 "halt".

Part a is: Set position 1 to 12 and position 2 to 2. Run until you halt. What is the value in the 0th register.

Part b:

Find which values to set to position 1 and position 2 that give a target result, then return a number that is 100 \* position1 + position2. It is known that position1 and position2
will both be between 0 and 99
