mod tetroxide {
    use tetris::tetris::Tetris;
    use std::thread;

    enum Inputs {
        RotateCcw,
        RotateCw,
        HardDrop,
        Drop,
        Left,
        Right,
    }

    pub struct Game(Tetris);
    impl Game {
        pub fn new() -> Self {
            Game(Tetris::new())
        }

        pub fn run(&mut self) {
            let tet = &mut self.0;

            loop {
                tet.drop();
                tet.clean_board();
            }
        }
    }
}