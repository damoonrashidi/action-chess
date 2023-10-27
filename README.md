# Action Chess!

Action chess is the same chess you know and love, with castling, en passant, promotions and all that, but _real time instead of turn based_. Each piece has a cooldown that limits how often it is allowed to move.

## Things that are done

- [x] Render a board in the terminal
- [x] Move pieces about
- [x] Find a list of legal moves
  - [x] pinned pieces
  - [x] pawn
    - [x] move once
    - [x] double move from starting position
    - [x] captures
    - [x] promotions
    - [x] promotion captures
  - [x] rook
    - [x] moves
    - [x] captures
    - [x] castling
  - [x] bishop
    - [x] moves
    - [x] captures
  - [x] knight
    - [x] moves
    - [x] captures
  - [x] queen
    - [x] moves
    - [x] captures
  - [x] king
    - [x] moves
    - [x] captures
    - [x] cannot move into check
    - [x] castling
  
 
## What isn't yet implemented but being worked on

- [ ] Networking
- [ ] Render to some ui

## Roadmap

- _Deck building_ - Draw cards while you are playing to assemble a deck with special powers such as "Rewind a square", "duplicate", "transform".
