# rust_lawn_mower

## Mower problem

### Description

Build a system for managing automatic lawn mowers designed to mow rectangular surfaces.

A mower has position defined by (X,Y) coordinates and a letter defining the direction it's facing, sides being N-north, E-east, W-west, S-south.
The lawn is a rectangle with a fixed size, and is divided into a grid, comprised of 1x1 cells exactly like a chess board,
the anchor, or coordinate system origin is the bottom left corner.
In example: 0 0 N means the lawn mower is located at the bottom left corner, facing North.
The mower rotates 90° in place and moves only 1 cell at a time
The commands for controlling the mower rotations are R for right and L for left, and F to make it move forward.
If the mower is instructed to go outside of the lawn, it ignores the command.
The input is provided via structured file format. The format starts with lawn size, followed by pairs of lines per mower, where line one of the pair is
position, line two is commands.
We can have multiple mowers in an input file, and each mower moves sequentially, so mower six starts after mower five and so on.
The separator is a space in all cases, except for commands where they're not separated.
Following, an example, with two mowers:

```
5 5 // lawn size, 5x5
1 2 N // first mower starting position, (1,2) facing north
LFLFLFLFF // commands for first mower
3 3 E // second mower starting position (3,3) facing east
FFRFFRFRRF // commands for second mower
```

the expected output is the final position and direction of the mower at the end of the execution of all of it's commands.

```
1 3 N // final position of the first mower
5 1 E // final position of the second mower
```

<b>Note</b>: in the samples, we use comments to provide insights, actual inputs and outputs will not have comments.
