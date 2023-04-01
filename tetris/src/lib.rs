use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
enum RotationState {
    Up,
    Right,
    Down,
    Left,
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

struct Pos(usize, usize);
impl Pos {
    fn new(row: usize, col: usize) -> Self {
        // Asserting the pos is within bounds we want or else we panic.
        assert_eq!(row < 40, col < 10);
        Pos(row, col)
    }
    fn coords(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

struct ActivePiece {
    tetromino: Tetromino,
    origin: Pos,
    rotation: RotationState,
}
impl ActivePiece {
    fn new(tetromino: Tetromino) -> Self {
        panic!()
    }

    fn rotate(&mut self, board: [[u8; 10]; 40]) {
        if let Tetromino::O = self.tetromino {
            return;
        }
    }

    fn drop(&mut self) {}

    fn hard_drop(&mut self, board: [[u8; 10]; 40]) {}
}

struct Tetris {
    board: [[u8; 10]; 40],
    active: ActivePiece,
    queue: VecDeque<Tetromino>,
    bag: Bag,
}
impl Tetris {
    pub fn new() -> Self {
        let board = [[0; 10]; 40];
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
