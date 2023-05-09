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

### Particularly Rustic Code

```rust
    // These are the different "origin" states we will be testing.
    let origin = if let Tetromino::I = self.tetromino {
        match (clockwise, new_rotation) {
            (true, State::Up) | (false, State::Right) => (row - 1, col),
            (true, State::Right) | (false, State::Down) => (row, col + 1),
            ...
        }
    } else {
        (row, col)
    };
    let mut origins = vec![origin];
    // "Kick data" refers to the possible offset values that can be used for the 4 kick states.
    // There are 8 total different offset value sets; 4 of which are inverted from the other 4.
    // Refers to the I Tetromino.
    let kick_data_i1 = vec![(-2, 0), (1, 0), (-2, -1), (1, -2)];
    let kick_data_i2 = vec![(-1, 0), (2, 0), (-1, -2), (2, 1)];
    // Refers to the other 5 (non-O) Tetrominos.
    let kick_data = vec![(-1, 0), (-1, -1), (0, 2), (-1, 2)];
    // We extend our possible tests with the 4 additional tests:
    origins.extend(
        match self.tetromino {
            Tetromino::O => return false, /* O Tetromino's have no rotational logic. */
            Tetromino::I => match (self.rotation, new_rotation) {
                // CW from Spawn State OR CCW to Inverted Spawn State
                (State::Up, State::Right) | (State::Left, State::Down) => kick_data_i1,
                // CCW to Spawn State OR CW from Inverted Spawn State
                (State::Right, State::Up) | (State::Down, State::Left) => {
                    kick_data_i1.into_iter().map(|(x, y)| (-x, -y)).collect()
                }
                ...
                _ => unreachable!(), /* THIS SHOULD NEVER HAPPEN. */
            },
            _ => match (self.rotation, new_rotation) {
                // CW from Spawn State OR CCW to Inverted Spawn State
                (State::Up, State::Right) | (State::Down, State::Right) => kick_data,
                // CCW to Spawn State OR CW from Inverted Spawn State
                (State::Right, State::Up) | (State::Right, State::Down) => {
                    kick_data.into_iter().map(|(x, y)| (-x, -y)).collect()
                }
                ...
                _ => unreachable!(), /* THIS SHOULD NEVER HAPPEN. */
            },
        }
        .into_iter()
        .map(|(x, y)| (row + y, col + x)),
    );
```

The above code snippet takes advantage of several Rust features to achieve maximum brevity (previous iterations of this function following more standard design patterns were quite a bit longer). It obviously makes heavy use of enums and pattern matching; with the enums allowing us to very easily describe and match rotations without having to directly play with coordinates, while stacked pattern matching allows us to cover the vast multitude of possible rotation cases in far fewer statements than you could with `if` checks. Working in unison, it also makes use of Rust's ability to stick code blocks anywhere by returning them from the matches, and having them evaluate to Rust's funcional style iterator mapping to breifly compute coordinate permutations. 

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
