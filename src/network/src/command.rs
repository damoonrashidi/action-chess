/**
A command is 4 bytes represented by a `[u8; 4]`.
This is all data needed to represent a move, and thus will be all that is sent over the wire during a game.

The first byte indicatates what kind of move it is.

0. Piece
1. Promotion
2. King Side Castle
3. Queen Side Castle
--
16. Join
32. Leave
48. Resign

### In the case of (0) Piece.
The second byte is the "from" `Coord` in the format u8 0..64 translated to (file, rank)
The third byte is the "to" `Coord` in the format u8 0..64 translated to (file, rank)
The fourth byte is an EMPTY buffer byte

### In the case of (1) Promotion:

The second byte is the "from" `Coord` sent as a u8 0..64 translated to (file, rank).

The third byte is the "to" `Coord` sent as a u8 0..64 translated to (file, rank).

The forth byte is a u8 representing a Piece where the 4 most significant bits are a piece representation.

```markdown
Pawn = 1
Knight = 2
Bishop = 3
Rook = 4
Queen = 5
King = 6
```
**Note: Pawn and King promotions are illegal moves.**

The four least significant bits are a color representation:
```markdown
0 = White
1 = Black
```
Thus `0b0101_0001` is a black queen.

### In the case of (2) King Side Castle

The second byte is color, signified by the least important bit:
```markdown
0 = White
1 = Black
```
The third byte is an EMPTY buffer byte.

The fourth byte is an EMPTY buffer byte.

### In the case of (2) Queen Side Castle

The second byte is color, signified by the least important bit:
```markdown
0 = White
1 = Black
```
The third byte is an EMPTY buffer byte.

The fourth byte is an EMPTY buffer byte.

*/
pub type Command = [u8; 4];
