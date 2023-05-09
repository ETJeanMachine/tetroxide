# TETROXIDE

Team members:

- Jesse Pingitore
- Andrew Idak
- Eric Hamilton

## Summary Description

A command line based version of [Tetris](https://en.wikipedia.org/wiki/Tetris?oldformat=true), written in Rust. The GUI and real-time input / gameplay component
of the app was implemented using [crossterm](https://github.com/crossterm-rs/crossterm), a text-based interface library. The GUI & appearance of our implementation is based off of the very original version of terminal Tetris, while the gameplay implements modern standards, namely the Super Rotation System and the Guideline scoring system.
![Demo](demo.gif)

## Project Execution Summary

### Phase I

- Implementing basic functional components:
  - `Tetromino` - enum differentiating basic piece shapes and values for the game board.
  - `State` - enum representing rotation states of an abstract piece.
  - `Pos` - struct representing a position on the game board.
  - `ActivePiece` - struct representing some currently active piece, irrespective of the game board.
  - `Bag` - struct abstracting the random selection of next pieces for play.
  - `Tetris` - struct managing the other components during play, as well as representing the game board and all other variables we want to track.
- Notably, all of these components were designed with some abstraction in mind, to avoid relying entirely on direct calculations on the game board as ourentire source of truth or worrying about the exact piece in play.
- Implementing basic gameplay logic for the components:
  - Displaying tetrominos
  - Rotating a piece state in isolation
  - Moving a position and bounds-checking it.
  - Shifting, rotating (checking collisions after abstract rotation), and dropping some actual active piece.
  - Solidifying a dropped piece and clearing filled lines.
  - Drawing random pieces from the bag, tracking them in a queue, and spawning them at the top of the board.
- Implementing a non-realtime gameplay loop based on basic printing and text input.

### Phase II

- Basic real-time game loop (via un-optimal terminal printing)
- "Gravity"-based drop calculations
- Level increases, with an effect on gravity
- Basic line clear & manual drop scoring (with level multiplier)
- Basic upcoming piece & score displays

### Phase III

- Full & appropriately timed real-time game loop
- Piece solidification delay
- Full text UI, with colored pieces and optimal terminal operation
- Pause & restart functionality
- Leveling system
- T-spin bonuses
- Combo system

## Structure Summery
<!-- TODO -->
<!-- - Briefly describe the structure of the code (what are the main components, the
  module dependency structure). Why was the project modularized in this way? -->
We modularized into two seperate crates - `Tetroxide` and `Tetris`. The former holds the primary game logic, whilst the latter control the GUI and user input. Modularizing in this fashion allows us to more efficiently and cohesively execute our code. Additionally, keeping much, if not the majority, of the game logic private is simply just good coding practice.

### Abandoned & Difficult Approaches

<!-- TODO -->

<!-- - Were any parts of the code particularly difficult to express using Rust? What
  are the challenges in refining and/or refactoring this code to be a better
  example of idiomatic Rust?
- Describe any approaches attempted and then abandoned and the reasons why. What
  did you learn by undertaking this project? -->
Working with TUI-RS can be best described as disgusting. Not only was documentation so poor for it that we found us going to Chat-GPT for simple documentation help; the way in which it's structured leaves... much to be desired. Our level menu, for instance, feels quite hacky as simply formatted text, instead of an actual button interface. It works, just not well. However, it still is a significant step up from what we had with our debug mode in the previous phase; so it's by no means unsatisfactory. But this:

```rust
let all = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Length((size.width - 48) / 2),
            Constraint::Length(48),
            Constraint::Length((size.width - 48) / 2),
        ]
        .as_ref(),
    )
    .split(size);
let layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Length(12),
            Constraint::Length(24),
            Constraint::Length(12),
            Constraint::Percentage(100),
        ]
        .as_ref(),
    )
    .margin(1)
    .split(all[1]);
let stats_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Percentage(100),
    ])
    .split(layout[0]);
```

Does leave a lot to be desired in terms of... sheer readability and understanding.

In our abandoned department, we would've liked to have made this a multiplayer game. However, we soon realized that we were undertaking a lot more than we expected when it came to actually working in purely single-player. This is something we may have been able to accomplish given we had one or so more group members, as ultimately, it came down to a time constraint, and fulfilling the fullest scope of the project as we could.

<!-- Jesse - tbh i should put down all of unit testing, I had trouble getting Rust to play nice while also trying to infer what the other crates were doing. Rust isn't a good language for prototyping and often writing Unit Tests were lengthier processes than hand-testing with print statements. -->

### Particularly Rustic Code

... and on the plus side, we have the following example exemplifying some of Rust's strengths:

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
    ...
    let kick_data_i1 = vec![(-2, 0), (1, 0), (-2, -1), (1, -2)];
    ...
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
            ...
        }
        .into_iter()
        .map(|(x, y)| (row + y, col + x)),
    );
```

The above code snippet takes advantage of several Rust features to achieve maximum brevity (previous iterations of this function following more standard design patterns were quite a bit longer). It obviously makes heavy use of enums and pattern matching; with the enums allowing us to very easily describe and match rotations without having to directly play with coordinates, while stacked pattern matching allows us to cover the vast multitude of possible rotation cases in far fewer statements than you could with `if` checks. Working in unison, it also makes use of Rust's ability to stick code blocks anywhere by returning them from the matches, and having them demonstrate some of Rust's secondary strengths in the use of small closures and functional programming with the `into_iter` and `map` calls to replace what would otherwise be verbose manually loop rolling. Finally, the ability to use `unreachable!()` macros makes it much easier to succinctly deal with impossible edge cases which the compiler simply isn't able to understand, instead of bothering with extraneous error handling.

### Dependencies

For the `tetris` crate (business logic):

- `rand="0.8.4"`
  - For randomness in generating piece order
- `strum="0.24"`
  - For easier enums
- `strum_macros="0.24"`
  - Strum extras

For the `tetroxide` crate (GUI and game handling):

- `tetris = { path = "../tetris" }`
  - Path shortening
- `spin_sleep = "1.1.1"`
  - More accurate thread sleeping
- `clap = { version = "4.2.1", features = ["derive"] }`
  - CLI argument parser
- `crossterm = {version = "0.26.1", features = [ "serde" ]}`
  - OS independent terminal interfaces
- `futures = "0.3"`
  - Asynchronous abstractions, covered in class
- `rand = { version = "0.7.3", default-features = false, features = ["std"] }`
  - Randomness in piece order generation
- `[dependencies.async-std]`
  - Asynchronous functions for the main game loop.
- `[dependencies.tui]`
  - Additional TUI dependencies, such as the crossterm backend.
- `[dependencies.tui-input]`

### Rubric Discussion

- Completeness:
While our project accomplished our MVP; there was quite a bit more we would have liked to get done, such as multiplayer. This mostly did not take place due to our underestimation of how complex Tetris's rotational and scoring systems are. As it stands now, our current implementation is our MVP, and we completed that successfully.

- Style/Design:
  - Our overall design is solid. Modularizing into separate GUI/logical crates worked well, and our style, while messy with regards to how TUI-RS works; is consistent and works well.
  - We also consistently use various rust traits and elements throughout our project.
    - Our bag for our tetris module is an iterator
    - Our main tetris function has an extensive display function for that trait
    - We use many enums; with implemented traits; not limited to:
      - `Tetromino`
      - `SpinType`
      - `MenuState`
    - We also use typing very well; including limiting our board to be `u8` as we acknowledge that any more data would be wasteful.

- Effort/Accomplishment:
  - This was not a low-effort undertaking of a project. Tetris; while on the onset may seem simple, it is considerably more complex than that. As such, we believe that our end result was high-effort. Our graphical display we also believe to be of very good quality.
  - Our understanding of rust was enhanced through this project; and ways in which to use rust tools, like iterators, was very much so enhanced.
  - We had to do a lot of research and understanding into Tetris to fully understand a lot of the intricate rules and scoring systems in the game. This was an undertaking in and of itself; outside of the coding portion of the project.
  - We also had to undertake quite a deal of research in TUI-RS and how to effectively use it to display graphical output to the user.
