# Project Title

Team members:

- Jesse Pingitore
- Andrew Idak
- Eric Hamilton

## Summary Description

A command line based version of [Tetris](https://en.wikipedia.org/wiki/Tetris?oldformat=true), written in Rust. The GUI and real-time input / gameplay component
of the app was implemented using [crossterm](https://github.com/crossterm-rs/crossterm), a text-based interface library. The GUI & appearance of our implementation is based off of the very original version of terminal Tetris, while the gameplay implements modern standards, namely the Super Rotation System and the Guideline scoring system.

## Project Execution Summary

* Phase I:
  * Implementing basic functional components:
    * `Tetromino` - enum differentiating basic piece shapes and values for the game board.
    * `State` - enum representing rotation states of an abstract piece.
    * `Pos` - struct representing a position on the game board.
    * `ActivePiece` - struct representing some currently active piece, irrespective of the game board.
    * `Bag` - struct abstracting the random selection of next pieces for play.
    * `Tetris` - struct managing the other components during play, as well as representing the game board and all other variables we want to track.
  * Notably, all of these components were designed with some abstraction in mind, to avoid relying entirely on direct calculations on the game board as our entire source of truth or worrying about the exact piece in play.
  * Implementing basic gameplay logic for the components:
    * Displaying tetrominos
    * Rotating a piece state in isolation
    * Moving a position and bounds-checking it.
    * Shifting, rotating (checking collisions after abstract rotation), and dropping some actual active piece.
    * Solidifying a dropped piece and clearing filled lines.
    * Drawing random pieces from the bag, tracking them in a queue, and spawning them at the top of the board.
  * Implementing a non-realtime gameplay loop based on basic printing and text input.
* Phase II:
  * Basic real-time game loop (via un-optimal terminal printing)
  * "Gravity"-based drop calculations
  * Level increases, with an effect on gravity
  * Basic line clear & manual drop scoring (with level multiplier)
  * Basic upcoming piece & score displays
* Phase III:
  * Full & appropriately timed real-time game loop
  * Piece solidification delay
  * Full text UI, with colored pieces and optimal terminal operation
  * Pause & restart functionality
  * T-spin bonuses
  * Combo system



## Additional Details


### Dependencies

For the `tetris` crate (business logic):
* rand="0.8.4" 
  * For randomness in generating piece order
* strum="0.24"
  - For easier enums
* strum_macros="0.24"
  - Strum extras

For the `tetroxide` crate (GUI and game handling):

* tetris = { path = "../tetris" } 
  * Path shortening
* spin_sleep = "1.1.1" 
  * More accurate thread sleeping
* clap = { version = "4.2.1", features = ["derive"] }
  - CLI argument parser
* crossterm = {version = "0.26.1", features = [ "serde" ]}
  - OS independent terminal interfaces
* futures = "0.3"
  - Asynchronous abstractions
* rand = { version = "0.7.3", default-features = false, features = ["std"] }
  - Randomness in piece order generation

[dependencies.async-std]
* version = "1.7.0"

[dependencies.tui]
* version = "0.19.0"
* default-features = false
* features = ["crossterm", 'serde']

[dependencies.tui-input]
* version = "0.7.0"*


- Briefly describe the structure of the code (what are the main components, the
  module dependency structure). Why was the project modularized in this way?
- Choose (at least) one code excerpt that is a particularly good example of Rust
  features, idioms, and/or style and describe what makes it “Rusty”.
- Were any parts of the code particularly difficult to expres using Rust? What
  are the challenges in refining and/or refactoring this code to be a better
  example of idiomatic Rust?
- Describe any approaches attempted and then abandoned and the reasons why. What
  did you learn by undertaking this project?
- Review the final project grading rubric and discuss any relevant aspects of
  the project.
