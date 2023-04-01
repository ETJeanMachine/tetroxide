use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Tetromino {
    I,
    T,
    O,
    J,
    L,
    S,
    Z,
}
enum Rotation {
    Up,
    Down,
    Left,
    Right,
}
struct Bag(VecDeque<Tetromino>);
impl Bag {
    /// Creates a new bag with randomly shuffled Tetromino's.
    /// A bag always has at most 7 tetromino's inside of it, one
    /// of each of the main pieces, as to ensure that a player isn't constantly
    /// getting the same piece over and over, but there is still an element of randomness.
    fn new() -> Self {
        let mut bag = Bag(VecDeque::new());
        bag.fill();
        bag
    }
    /// Fills an empty bag with randomly shuffled Tetromino's.
    /// If the bag isn't empty, it returns `false`, as it is unable to
    /// fill a bag already full. After successfully filling the bag it returns
    /// `true`.
    fn fill(&mut self) -> bool {
        if self.0.len() > 0 {
            return false;
        }
        let mut temp = vec![
            Tetromino::I,
            Tetromino::T,
            Tetromino::O,
            Tetromino::J,
            Tetromino::L,
            Tetromino::S,
            Tetromino::Z,
        ];
        temp.shuffle(&mut thread_rng());
        self.0 = VecDeque::from(temp);
        true
    }
}
impl Iterator for Bag {
    type Item = Tetromino;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
// impl IntoIterator for Bag {
//     type Item = Tetromino;

//     type IntoIter = Vec<Tetromino>;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

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
