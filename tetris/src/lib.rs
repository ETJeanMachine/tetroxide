use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const MAX_ROW: usize = 40;
const MAX_COL: usize = 10;

/// A Tetromino is a tetromino in tetris. They are all made up of exactly 4 blocks.
/// It can be one of 7 different variants:
/// - `I` Pieces, also called Line Pieces.
/// - `O` Pieces, also called Square Pieces.
/// - `T` Pieces.
/// - `L`/`J` Pieces.
/// - `S`/`Z` Pieces, also called "skew".
#[derive(Debug, Clone, Copy, EnumIter)]
enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}
#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
enum State {
    Up,
    Right,
    Down,
    Left,
}
impl State {
    /// This returns the rotational enum for if you rotate either clockwise or
    /// counter-clockwise.
    fn rotate(&self, clockwise: bool) -> Self {
        if clockwise {
            match self {
                State::Up => State::Right,
                State::Right => State::Down,
                State::Down => State::Left,
                State::Left => State::Up,
            }
        } else {
            match self {
                State::Up => State::Left,
                State::Right => State::Up,
                State::Down => State::Right,
                State::Left => State::Down,
            }
        }
    }
}
/// A bag is a data structure used by Tetris to represent the queue of incoming
/// pieces.
struct Bag(Vec<Tetromino>);
impl Bag {
    /// Creates a new bag with randomly shuffled Tetromino's. A bag always has
    /// at most 7 tetromino's inside of it, one of each of the main pieces, as
    /// to ensure that a player isn't constantly getting the same tetromino over
    /// and over, but there is still an element of randomness.
    fn new() -> Self {
        let mut bag = Bag(Vec::with_capacity(7));
        bag.fill();
        bag
    }
    /// Fills an empty bag with randomly shuffled Tetromino's. If the bag isn't
    /// empty, it does nothing, as it is unable to fill a bag already full.
    /// Otherwise, the bag is filled up with shuffled elements.
    fn fill(&mut self) {
        if !self.0.is_empty() {
            return;
        }
        for tetromino in Tetromino::iter() {
            self.0.push(tetromino);
        }
        self.0.shuffle(&mut thread_rng());
    }
}
impl Iterator for Bag {
    type Item = Tetromino;

    fn next(&mut self) -> Option<Self::Item> {
        // We try to fill it regardless of anything...
        self.fill();
        // Then we simply pop from the top of the bag!
        self.0.pop()
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos(usize, usize);
impl Pos {
    fn new(row: usize, col: usize) -> Self {
        // Asserting the pos is within bounds we want or else we panic.
        assert_eq!(row < MAX_ROW, col < MAX_COL);
        Pos(row, col)
    }
    fn coords(&self) -> (usize, usize) {
        (self.0, self.1)
    }
    fn try_move(&self, x: i32, y: i32) -> Option<Self> {
        let (row, col) = (self.0 as i32, self.1 as i32);
        if row + x < 0 || row + x >= MAX_ROW as i32 || col + y < 0 || col + y >= MAX_COL as i32 {
            None
        } else {
            Some(Pos((row - x) as usize, (col - x) as usize))
        }
    }
}

struct ActivePiece {
    tetromino: Tetromino,
    origin: Pos,
    rotation: State,
}
impl ActivePiece {
    fn new(tetromino: Tetromino) -> Self {
        ActivePiece {
            tetromino,
            origin: Pos::new(19, 4),
            rotation: State::Up,
        }
    }

    /// Validates if a new state that we've passed in is valid within the
    /// board. If it is, we update the state and return true to signify that we
    /// updated.
    fn validate(&mut self, new_state: &ActivePiece, board: &[[u8; MAX_COL]; MAX_ROW]) -> bool {
        let x = new_state.origin.0;
        let y = new_state.origin.1;
        let valid_pos = match new_state.tetromino {
            Tetromino::I => match new_state.rotation {
                State::Up => {
                    (x > 0 && x < MAX_COL - 2 && y < MAX_ROW)
                        && (board[x - 1][y] | board[x][y] | board[x + 1][y] | board[x + 2][y]) == 0
                }
                State::Right => {
                    (x < MAX_COL && y > 0 && y < MAX_ROW - 2)
                        && (board[x][y - 1] | board[x][y] | board[x][y + 1] | board[x][y + 2]) == 0
                }
                State::Down => {
                    (x > 1 && x < MAX_COL - 1 && y < MAX_ROW)
                        && (board[x - 2][y] | board[x - 1][y] | board[x][y] | board[x + 1][y]) == 0
                }
                State::Left => {
                    (x < MAX_COL && y > 1 && y < MAX_ROW - 1)
                        && (board[x][y - 2] | board[x][y - 1] | board[x][y] | board[x][y + 1]) == 0
                }
            },
            Tetromino::O => {
                (x < MAX_COL - 1 && y > 1 && y < MAX_ROW)
                    && (board[x][y - 1] | board[x + 1][y - 1] | board[x][y] | board[x + 1][y]) == 0
            }
            Tetromino::T => match new_state.rotation {
                State::Up => {
                    (x > 0 && x < MAX_COL - 1 && y > 0)
                        && (board[x][y - 1] | board[x - 1][y] | board[x][y] | board[x + 1][y]) == 0
                }
                State::Right => {
                    (x < MAX_COL - 1 && y > 0 && y < MAX_ROW - 1)
                        && (board[x][y - 1] | board[x][y] | board[x + 1][y] | board[x][y + 1]) == 0
                }
                State::Down => {
                    (x > 0 && x < MAX_COL - 1 && y < MAX_ROW - 1)
                        && (board[x - 1][y] | board[x][y] | board[x + 1][y] | board[x][y + 1]) == 0
                }
                State::Left => {
                    (x > 0 && y > 0 && y < MAX_ROW - 1)
                        && (board[x][y - 1] | board[x - 1][y] | board[x][y] | board[x][y + 1]) == 0
                }
            },
            Tetromino::J => match new_state.rotation {
                State::Up => {
                    (x > 0 && x < MAX_COL - 1 && y > 0)
                        && (board[x - 1][y - 1] | board[x - 1][y] | board[x][y] | board[x + 1][y])
                            == 0
                }
                State::Right => {
                    (x < MAX_COL - 1 && y > 0 && y < MAX_ROW - 1)
                        && (board[x][y - 1] | board[x + 1][y - 1] | board[x][y] | board[x][y + 1])
                            == 0
                }
                State::Down => {
                    (x > 0 && x < MAX_COL - 1 && y < MAX_ROW - 1)
                        && (board[x - 1][y] | board[x][y] | board[x + 1][y] | board[x + 1][y + 1])
                            == 0
                }
                State::Left => {
                    (x > 0 && y > 0 && y < MAX_ROW - 1)
                        && (board[x][y + 1] | board[x][y] | board[x - 1][y + 1] | board[x][y + 1])
                            == 0
                }
            },
            Tetromino::L => match new_state.rotation {
                State::Up => {
                    (x > 0 && x < MAX_COL - 1 && y > 0)
                        && (board[x + 1][y - 1] | board[x - 1][y] | board[x][y] | board[x + 1][y])
                            == 0
                }
                State::Right => {
                    (x < MAX_COL - 1 && y > 0 && y < MAX_ROW - 1)
                        && (board[x][y - 1] | board[x][y] | board[x][y + 1] | board[x + 1][y + 1])
                            == 0
                }
                State::Down => {
                    (x > 0 && x < MAX_COL - 1 && y < MAX_ROW - 1)
                        && (board[x - 1][y] | board[x][y] | board[x + 1][y] | board[x - 1][y + 1])
                            == 0
                }
                State::Left => {
                    (x > 0 && y > 0 && y < MAX_ROW - 1)
                        && (board[x - 1][y - 1] | board[x][y - 1] | board[x][y] | board[x][y + 1])
                            == 0
                }
            },
            Tetromino::S => match new_state.rotation {
                State::Up => {
                    (x > 0 && x < MAX_COL - 1 && y > 0)
                        && (board[x][y - 1] | board[x + 1][y - 1] | board[x - 1][y] | board[x][y])
                            == 0
                }
                State::Right => {
                    (x < MAX_COL - 1 && y > 0 && y < MAX_ROW - 1)
                        && (board[x][y - 1] | board[x][y] | board[x + 1][y] | board[x + 1][y + 1])
                            == 0
                }
                State::Down => {
                    (x > 0 && x < MAX_COL - 1 && y < MAX_ROW - 1)
                        && (board[x][y] | board[x + 1][y] | board[x - 1][y + 1] | board[x][y + 1])
                            == 0
                }
                State::Left => {
                    (x > 0 && y > 0 && y < MAX_ROW - 1)
                        && (board[x - 1][y - 1] | board[x - 1][y] | board[x][y] | board[x][y + 1])
                            == 0
                }
            },
            Tetromino::Z => match new_state.rotation {
                State::Up => {
                    (x > 0 && x < MAX_COL - 1 && y > 0)
                        && (board[x - 1][y - 1] | board[x][y - 1] | board[x][y] | board[x + 1][y])
                            == 0
                }
                State::Right => {
                    (x < MAX_COL - 1 && y > 0 && y < MAX_ROW - 1)
                        && (board[x + 1][y - 1] | board[x][y - 1] | board[x][y] | board[x + 1][y])
                            == 0
                }
                State::Down => {
                    (x > 0 && x < MAX_COL - 1 && y < MAX_ROW - 1)
                        && (board[x + 1][y - 1] | board[x][y] | board[x + 1][y] | board[x][y + 1])
                            == 0
                }
                State::Left => {
                    (x > 0 && y > 0 && y < MAX_ROW - 1)
                        && (board[x][y - 1] | board[x - 1][y] | board[x][y] | board[x - 1][y + 1])
                            == 0
                }
            },
        };
        if valid_pos {
            self.origin = new_state.origin;
            self.rotation = new_state.rotation;
        }
        valid_pos
    }

    /// Tetris's rotational system is complex. To refer to it, please see
    /// [the tetris wiki page on the subject](https://tetris.fandom.com/wiki/SRS#Wall_Kicks).
    /// In short summary, each piece goes through 5 different tests when it
    /// attempts to rotate - the 1st being the basic rotational state, and the
    /// following 4 being various "wall kick" states. This is what enables
    /// complex moves like [t-spins](https://tetris.com/article/70/how-to-perform-a-t-spin-in-tetris)
    /// to properly work.
    ///
    /// This function takes in a bool as to if it is going
    /// clockwise/counter-clockwise, and performs the rotation on itself if it
    /// can be successfully done.
    fn rotate(&mut self, clockwise: bool, board: &[[u8; MAX_COL]; MAX_ROW]) {
        // These are the different "origin" states we will be testing.
        let mut tests = vec![Some(self.origin)];
        // Adding the other 4 tests (wall-kicks).
        let new_rotation = self.rotation.rotate(clockwise);
        // We extend our possible tests with the 4 additional tests:
        let origin = self.origin;
        // This is for internal logic for the fact that rotational states are identical
        // whether they are cw/ccw, but they have negative vals swapped. This allows for a more
        // idiomatic way of handling rotation states (maximum of 4 matches rather than 16).
        // Handling the fact that numbers are identical whether cw/ccw, but the dirs are
        // swapped.
        tests.extend(match self.tetromino {
            Tetromino::O => return, /* O Tetromino's have no rotational logic. */
            Tetromino::I => match (self.rotation, new_rotation) {
                (State::Up, State::Right) | (State::Left, State::Down) => vec![
                    origin.try_move(-2, 0),
                    origin.try_move(1, 0),
                    origin.try_move(-2, 1),
                    origin.try_move(1, -2),
                ],
                (State::Right, State::Up) | (State::Down, State::Left) => vec![
                    origin.try_move(2, 0),
                    origin.try_move(-1, 0),
                    origin.try_move(2, -1),
                    origin.try_move(-1, 2),
                ],
                (State::Right, State::Down) | (State::Up, State::Left) => vec![
                    origin.try_move(-1, 0),
                    origin.try_move(2, 0),
                    origin.try_move(-1, -2),
                    origin.try_move(2, 1),
                ],
                (State::Down, State::Right) | (State::Left, State::Up) => vec![
                    origin.try_move(1, 0),
                    origin.try_move(-2, 0),
                    origin.try_move(1, 2),
                    origin.try_move(-2, -1),
                ],
                _ => unreachable!(), /* THIS SHOULD NEVER HAPPEN. */
            },
            _ => match (self.rotation, new_rotation) {
                (State::Up, State::Right) | (State::Down, State::Right) => vec![
                    origin.try_move(-1, 0),
                    origin.try_move(-1, -1),
                    origin.try_move(0, 2),
                    origin.try_move(-1, 2),
                ],
                (State::Right, State::Up) | (State::Right, State::Down) => vec![
                    origin.try_move(1, 0),
                    origin.try_move(1, 1),
                    origin.try_move(0, -2),
                    origin.try_move(1, -2),
                ],
                (State::Down, State::Left) | (State::Left, State::Up) => vec![
                    origin.try_move(1, 0),
                    origin.try_move(1, -1),
                    origin.try_move(0, 2),
                    origin.try_move(1, 2),
                ],
                (State::Left, State::Down) | (State::Up, State::Left) => vec![
                    origin.try_move(-1, 0),
                    origin.try_move(-1, 1),
                    origin.try_move(0, -2),
                    origin.try_move(-1, -2),
                ],
                _ => unreachable!(), /* THIS SHOULD NEVER HAPPEN. */
            },
        });
        // Attempting all of our tests.
        for test in tests {
            if let Some(pos) = test {
                let new_state = ActivePiece {
                    tetromino: self.tetromino,
                    origin: pos,
                    rotation: new_rotation,
                };
                // Returning if we've successfully validated a given state!
                if self.validate(&new_state, board) {
                    return;
                }
            }
        }
    }

    fn drop(&mut self) {}

    fn hard_drop(&mut self, board: &[[u8; MAX_COL]; MAX_ROW]) {}
}

pub struct Tetris {
    board: [[u8; MAX_COL]; MAX_ROW],
    active: ActivePiece,
    queue: VecDeque<Tetromino>,
    bag: Bag,
}
impl Tetris {
    pub fn new() -> Self {
        let board = [[0; MAX_COL]; MAX_ROW];
        let mut bag = Bag::new();
        // Placeholder.
        let mut active = ActivePiece::new(Tetromino::I);
        if let Some(t) = bag.next() {
            // We know the bag will always be full, but we must create a
            // placeholder anyways.
            active = ActivePiece::new(t);
        }
        // The queue is always a size of 4, and contains the next 4 tetrominos
        // from the bag, after the initial piece.
        let mut queue = VecDeque::with_capacity(4);
        while queue.len() < 4 {
            if let Some(t) = bag.next() {
                queue.push_back(t);
            }
        }
        Tetris {
            board,
            active,
            queue,
            bag,
        }
    }
}
