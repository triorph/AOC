# Day 2

Work out some rock paper scissor games.

First is using a misunderstanding of X Y and Z as Rock Paper and Scissors in the
input, then working out if its a win, draw or loss and getting the sum

Secondly is using X Y and Z as Loss Draw and Win then working out what the hand
would be to force that to happen and calculating the score.

People were talking about the idea of some modulo functions that automatically
work this out, but I think that's actually making the code less clear than the
"smooth brained" approach of just writing a bunch of `match` statements, so I
did that. Notably, with the complex modulo case you'd want to test all the
possibilities instead which would be just as much work.
