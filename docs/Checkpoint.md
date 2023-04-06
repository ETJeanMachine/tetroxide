# Tetroxide

Team members:

- Eric Jean
- Andrew Idak
- Jesse Pingitore

## Summary Description

A command line based version of [Tetris](https://en.wikipedia.org/wiki/Tetris?oldformat=true), written in Rust. We will be using [TUI](https://github.com/fdehau/tui-rs) to handle the GUI component and real-time gameplay of the final app, although this checkpoint version will use the standard console for frame-by-frame gameplay debugging purposes.

## Checkpoint Progress Summary

- Implemented `tetris` crate
  - `Tetromino` enum to represent the different pieces, with `impl` for getting relative coordinates of piece blocks
  - `State` enum to represent abstract piece rotations states, with `impl` for progressing rotation
  - `Bag` struct to represent Tetris' clever system for generating random new pieces while ensuring no sequential repetition greater than 2, with `impl`s for creating new `Bag`, filling with 7 new random pieces, and drawing piece from `Bag`
  - `Pos` struct, with `impl`s to encapsulate piece position checking and attempting to move
  - `ActivePiece` struct to represent abstract active piece, both for control and on board, with `impl`s for getting exact block coords and rotation/movement.
  - `Tetris` struct to represent the overall single-frame gamestate (board and all supporting components), with `impl`s for initialization, piece rotation/movement, holding, line clearing, and displaying the actual game in the console.
- Partially implemented `tetroxide` crate
  - main() to handle interfacing between the `Tetris` struct and the game loop, taking input, and formatting for console printing.
  - started lib implementation for next phase with `TUI` based graphics.

## Additional Details

- External crates:
  - tui
  - tui-input
  - spin_sleep
  - crossterm
- Code Structure:
  - `tetris` crate
    - `lib` represents single frame game state, as well as all actions it can take
  - `tetroxide` crate
    - `main` runs core game loop, takes input, and calls everything else
    - `lib` will eventually contain `TUI` graphics and input handling
