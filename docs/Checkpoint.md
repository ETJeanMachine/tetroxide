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
  - `ActivePiece` struct

## Additional Details

- List any external Rust crates required for the project (i.e., what
  `[dependencies]` have been added to `Cargo.toml` files).
- Briefly describe the structure of the code (what are the main components, the
  module dependency structure).
- Pose any questions that you may have about your project and/or request
  feedback on specific aspects of the project.
