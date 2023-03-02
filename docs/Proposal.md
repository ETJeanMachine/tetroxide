# Tetroxide

Team members:

- Eric Hamilton
- Jesse Pingitore
- Andrew Idak

## Summary Description

A command line based version of [Tetris](https://en.wikipedia.org/wiki/Tetris?oldformat=true), written in Rust. Ideally, will be using a [Ncurses](https://en.wikipedia.org/wiki/Ncurses?oldformat=true) wrapper crate (such as [curses-rs](https://github.com/jeaye/ncurses-rs) or [cursive](https://crates.io/crates/cursive)) to handle the GUI component of the app.

## Additional Details

- The application should start up upon running `cargo run`. Once started, a single-player version of tetris would begin (depending on stretch goals, this would actually prompt the user for which gamemode they wish to start in, be it single player or multiplayer).
- A sketch of intended components (key functions, key data structures, separate
  modules).
- Thoughts on testing. These might include critical functions or data structures
  that will be given `#[test]` functions. Also consider using the Also consider
  using the [`test_case`](https://crates.io/crates/test-case) crate,
  [`quickcheck`](https://crates.io/crates/quickcheck) crate,
  [`proptest`](https://crates.io/crates/proptest) crate, or [`cargo
fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) tool.
- Thoughts on a “minimal viable product” and “stretch goals”. Be sure to review
  the final project grading rubric and consider organizing the project around a
  core deliverable that will almost certainly be achieved and then a number of
  extensions and features that could be added to ensure that project is of
  suitable size/scope/effort.
- Expected functionality to be completed at the Checkpoint.
