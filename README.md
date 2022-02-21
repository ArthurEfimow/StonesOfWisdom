# StonesOfWisdom
A simple Boardgame

This is a two player competitive turnbased boardgame written in rust. 

The rules are simple:
1. There are two players Green and Blue. Green starts.
2. The board is 8 x 8 tiles big. At the start there are four pieces on the board.(Picture 1)
3. The active player has to set a piece in their color on a free tile. Their opponent becomes the active player.
4. If the set piece encapsulates a line of the different color then the line becomes the same color of the et piece.
5. The game ends when there are no free tiles.
6. Winner is the one with more colored pieces.

Installation:

Download the sources.

Run "cargo build" in the StonesOfWisdom folder.

Starting the Game:

Run "cargo run --bin stones_of_wisdom <p1> <p2>"

<p1> and <p2> are numerical values, which set the ai for the palyers Green and Blue.

There are six AIs to choose from:

0: No AI. Player input.
1: Random
2: Greedy
3: MinMax
4: Greedy (weighted)
5: MinMax (weighted)
6: real AI

