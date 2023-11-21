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
  - [x] knight
  - [x] queen
  - [x] king
    - [x] cannot move into check
    - [x] castling

- [x] Networking
    - [x] Serialization / Deserialization protocol for moves and commands
    - [x] Server state
    - [x] Event sourcing to bring new clients to correct game state
    - [x] Host games
    - [x] Joining games

- [x] Debug Client
    - [x] Parse Input from STDIN
    - [x] Render board to STDOUT
    - [x] Send moves to the server
    - [x] Get moves from the server
    - [x] Tick game

## What isn't yet implemented but being worked on
- [ ] 3D Client
    - [ ] Create Meshes for pieces
        - [ ] Create Textures
    - [ ] Create a Board Mesh
        - [ ] Create a shader for the checkered texture
    - [ ] Learn 3D programming
    - [ ] Camera
    - [ ] Lighting
    - [ ] Input

## Roadmap

- _Deck building_ - Draw cards while you are playing to assemble a deck with special powers such as "Rewind a square", "duplicate", "transform".
