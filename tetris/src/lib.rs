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
#[derive(Debug, EnumIter)]
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
    fn move_dir(&self, dir: State) -> Option<Self> {
        let (row, col) = self.coords();
        match dir {
            State::Up => {
                if self.0 > 0 {
                    Some(Pos(row - 1, col))
                } else {
                    None
                }
            }
            State::Right => {
                if self.1 < MAX_COL {
                    Some(Pos(row, col + 1))
                } else {
                    None
                }
            }
            State::Down => {
                if self.0 < MAX_ROW {
                    Some(Pos(row + 1, col))
                } else {
                    None
                }
            }
            State::Left => {
                if self.1 > 0 {
                    Some(Pos(row, col - 1))
                } else {
                    None
                }
            }
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
        match new_state.tetromino {
            Tetromino::I => todo!(),
            Tetromino::O => todo!(),
            Tetromino::T => todo!(),
            Tetromino::J => todo!(),
            Tetromino::L => todo!(),
            Tetromino::S => todo!(),
            Tetromino::Z => todo!(),
        }
    }

    /// Tetris's rotational system is complex.
    fn rotate(&mut self, clockwise: bool, board: &[[u8; MAX_COL]; MAX_ROW]) {
        let new_rotation = self.rotation.rotate(clockwise);
        // The new state that we're validating.
        let new_state = ActivePiece {
            tetromino: self.tetromino,
            origin: self.origin,
            rotation: new_rotation,
        };
        // If the basic rotation works, we return.
        if self.validate(&new_state, board) {
            return;
        }
        // Otherwise we try the other 4 tests.
        match self.tetromino {
            Tetromino::O => return,
            Tetromino::I => {}
            _ => {}
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
