# Tetroxide

Team members:

- Eric Hamilton
- Jesse Pingitore
- Andrew Idak

## Summary Description

A command line based version of [Tetris](https://en.wikipedia.org/wiki/Tetris?oldformat=true), written in Rust. We will be using either an [Ncurses](https://en.wikipedia.org/wiki/Ncurses?oldformat=true) wrapper crate (such as [curses-rs](https://github.com/jeaye/ncurses-rs) or [cursive](https://crates.io/crates/cursive)), or [TUI](https://github.com/fdehau/tui-rs) to handle the GUI component of the app.

## Additional Details

- The application should start up upon running `cargo run`. Once started, a single-player version of tetris would begin (depending on stretch goals, this would actually prompt the user for which gamemode they wish to start in, be it single or multiplayer).
- We need to implement the following:
  - A game model that handles the board state, the queue, the currently active block, and the block in hold.
    - Some of the data structures we'll have to use will include a definitely sized 2D array representing the board, a double-ended queue representing the upcoming pieces, and either a pointer or other data type such as a box to represent the piece currently in hold.
  - We need a module to display the actual game within either TUI or ncurses. This will also handle inputs.
  - We will need a logic controller to handle the core game logic, as well as handling the [game score when line clears occur](https://www.codewars.com/kata/5da9af1142d7910001815d32).
- We need to be able to test rotation, if hold is working correctly, if the queue is generating values properly, and handling specific edge cases of block placement/rotation (such as T-spins). We should also test if scores are generating as expected.
- MVP:
  - A complete version of single-player tetris. This includes all the explained functionality as described above.
- Stretch Goals:
  1. Levels (dropping pieces more quickly over time). This stretch goal should be attainable as MVP but we're including as an accessory just in case.
  2. Local multiplayer mode (race mode).
  3. Additional multiplayer gamemodes (attack).
      - Attack mode is where clearing lines sends pieces to another board.
      - Race mode is where its a timed score-based mode w/o any attacks.
  4. Online multiplayer.
  5. Puyo Puyo mode (additional game functionality).
- Game logic and basic rendering should be completed by the checkpoint. So, the UI may not be totally done, but the business logic should be complete.
