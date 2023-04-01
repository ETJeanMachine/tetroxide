use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A Tetromino is a piece in tetris. They are all made up of exactly 4 blocks.
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
enum Rotation {
    Up,
    Down,
    Left,
    Right,
}
/// A bag is a data structure used by Tetris to represent the queue of incoming
/// pieces.
struct Bag(Vec<Tetromino>);
impl Bag {
    /// Creates a new bag with randomly shuffled Tetromino's. A bag always has
    /// at most 7 tetromino's inside of it, one of each of the main pieces, as
    /// to ensure that a player isn't constantly getting the same piece over
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
        if self.0.len() > 0 {
            return;
        }
        let mut new_bag = vec![];
        for tetromino in Tetromino::iter() {
            new_bag.push(tetromino);
        }
        new_bag.shuffle(&mut thread_rng());
        self.0 = new_bag;
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
impl Pos {}

struct ActivePiece {
    piece: Tetromino,
    pos: Pos,
    rotation: Rotation,
}
impl ActivePiece {
    fn new() {}
}

struct Tetris {
    board: [[u8; 10]; 24],
    queue: VecDeque<Tetromino>,
    active: ActivePiece,
    bag: Bag,
}
// impl Tetris {
//     pub fn new() -> Self {
//         let mut bag =
//         Tetris {
//             board: [[0; 10]; 24],
//             queue: VecDeque::new(),
//             active: ActivePiece { piece: (), pos: () },
//         }
//     }
// }
