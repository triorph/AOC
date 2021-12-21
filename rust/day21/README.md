### Day 21

Dirac dice.

People play on a board of size 10, that loops back to 1, and roll 3 dice and add that to their position (modulo), then add that new position
to their final score. Repeat until someone has a score of 1000.

Part a:
Using a deterministic dice that starts at 1 and goes to 100, and repeats, find out the losing player's score, and multiply it by the number
of dice rolls.

Part b:
We have quantum dice, where each dice goes from 1 2 or 3 with each roll, but we have to check each possibility and determine the number
of wins per player, then return the number of wins of the highest win player.

Made a simple DFS algorithm that just backs out of the last change each time, by having a backup of the player at each step that it resets to.
I think at this point the requirements of the problem require it to go slow, although some kind of memoization might make it a bit faster again.
