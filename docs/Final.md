# Project Title

Team members:

- Jesse Pingitore
- Andrew Idak
- Eric Hamilton

## Summary Description

Reiterate the summary description of the overall goal of the project (updated as
necessary from the Proposal and/or Checkpoint documents).

A command line based version of [Tetris](https://en.wikipedia.org/wiki/Tetris?oldformat=true), written in Rust. We will be implementing with [TUI](https://github.com/fdehau/tui-rs) to handle the GUI component and real-time gameplay of the final app. 
TODO UPDATE THIS!!!!!!!!!!!!!!!!

## Project Execution Summary

Describe the work done for the project and lessons learned.

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
* serde = {version = "1.0", features = ["derive"] }
  - JSON serialization/deserialization
* serde_json = "1.0"
  - Serde helper
* chrono = { version = "0.4", features = ["serde"] }
  - More complete superset of the `Time` library, for  TODO WHAT DOES IT DO
* rand = { version = "0.7.3", default-features = false, features = ["std"] }
  - Randomness in piece order generation
* thiserror = "1.0"
  - Easy macro for `std::error::Error` 

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
