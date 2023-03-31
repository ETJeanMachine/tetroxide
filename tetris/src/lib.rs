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
struct Bag {
    default: Vec<Tetromino>,
    pieces: Vec<Tetromino>,
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
impl Bag {
    fn new() -> Self {
        let temp = vec![
            Tetromino::I,
            Tetromino::T,
            Tetromino::O,
            Tetromino::J,
            Tetromino::L,
            Tetromino::S,
            Tetromino::Z,
        ];

        Bag {
            default: temp.clone(),
            pieces: temp,
        }
    }

    // fn next(&mut self) -> Tetromino {
    //     match self.pieces.pop() {
    //         Some(x) => x,
    //         None => {
    //             self.pieces = self.default;
    //             self.pieces.shuffle(&mut thread_rng());

    //             match self.pieces.pop() {
    //                 Some(y) => y,
    //                 _ => unreachable!()
    //             }
    //         },
    //     }
    // }
}

struct Pos(usize, usize);
impl Pos {}

struct ActivePiece {
    piece: Tetromino,
    pos: Pos,
    rotation: Rotation,
}
impl ActivePiece {
    fn new() {
        
    }
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
