# Noughts and Pawns
**Noughts and Pawns** is my personal interpretation of *Chess-Tic-Tac-Toe*, a small experimental board game that mixes the goal of Tic-Tac-Toe with chess pieces and movement rules.
The game is played on a 4×4 grid by two players.  
Instead of placing Os and Xs (noughts and crosses), players place and move chess pieces (pawn, rook, bishop, knight) with the goal of creating four pieces in a row.

I started this project mainly to practice [Rust](https://rust-lang.org/) and [Bevy](https://bevy.org/) by building an entire game in Rust.

## Rules
The White player starts first. Each player starts with a Pawn, a Rook, a Bishop and a Knight in their Reserve.
On each turn a player can:
- place a piece on the board from their reserve

*or*

- move a piece already on the board.

Each piece moves and captures like in chess: 
- Pawn: moves forward by one square, captures diagonally. When it reaches the edge of the board, its movement direction is reversed;
- Rook: moves in straight lines;
- Bishop: moves diagonally;
- Knight: L-shaped moves (and can jump over other pieces).

A captured piece returns to its owner's reserve.
To win, a player must get **4 of their piece in a row**, horizontally, vertically, or diagonally.

## How to run
### Run the game (CLI)
```bash
cd ./noughts-and-pawns
cargo run