pub mod tetris {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::collections::VecDeque;
    use std::fmt::Display;
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
    pub enum Tetromino {
        I = 1,
        O = 2,
        T = 3,
        J = 4,
        L = 5,
        S = 6,
        Z = 7,
    }
    impl Tetromino {
        /// Gives the "shape" of a tetromino, given the default origin state is
        /// at (0, 0).
        fn shape(&self, rotation: State) -> [(i32, i32); 4] {
            match self {
                Tetromino::I => match rotation {
                    State::Up => [(-1, 0), (0, 0), (1, 0), (2, 0)],
                    State::Right => [(0, -1), (0, 0), (0, 1), (0, 2)],
                    State::Down => [(-2, 0), (-1, 0), (0, 0), (1, 0)],
                    State::Left => [(0, -2), (0, -1), (0, 0), (0, 1)],
                },
                Tetromino::O => [(0, -1), (1, -1), (0, 0), (1, 0)],
                Tetromino::T => match rotation {
                    State::Up => [(0, -1), (-1, 0), (0, 0), (1, 0)],
                    State::Right => [(0, -1), (0, 0), (1, 0), (0, 1)],
                    State::Down => [(-1, 0), (0, 0), (1, 0), (0, 1)],
                    State::Left => [(0, -1), (-1, 0), (0, 0), (0, 1)],
                },
                Tetromino::J => match rotation {
                    State::Up => [(-1, -1), (-1, 0), (0, 0), (1, 0)],
                    State::Right => [(0, -1), (1, -1), (0, 0), (0, 1)],
                    State::Down => [(-1, 0), (0, 0), (1, 0), (1, 1)],
                    State::Left => [(0, 1), (0, 0), (-1, 1), (0, -1)],
                },
                Tetromino::L => match rotation {
                    State::Up => [(1, -1), (-1, 0), (0, 0), (1, 0)],
                    State::Right => [(0, -1), (0, 0), (0, 1), (1, 1)],
                    State::Down => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
                    State::Left => [(-1, -1), (0, -1), (0, 0), (0, 1)],
                },
                Tetromino::S => match rotation {
                    State::Up => [(0, -1), (1, -1), (-1, 0), (0, 0)],
                    State::Right => [(0, -1), (0, 0), (1, 0), (1, 1)],
                    State::Down => [(0, 0), (1, 0), (-1, 1), (0, 1)],
                    State::Left => [(-1, -1), (-1, 0), (0, 0), (0, 1)],
                },
                Tetromino::Z => match rotation {
                    State::Up => [(-1, -1), (0, -1), (0, 0), (1, 0)],
                    State::Right => [(1, -1), (1, 0), (0, 0), (0, 1)],
                    State::Down => [(-1, 0), (0, 0), (0, 1), (1, 1)],
                    State::Left => [(0, -1), (0, 0), (-1, 0), (-1, 1)],
                },
            }
        }
    }
    impl Into<u8> for Tetromino {
        fn into(self) -> u8 {
            match self {
                Tetromino::I => 0,
                Tetromino::O => 1,
                Tetromino::T => 2,
                Tetromino::J => 3,
                Tetromino::L => 4,
                Tetromino::S => 5,
                Tetromino::Z => 6,
            }
        }
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
    pub struct Pos(usize, usize);
    impl Pos {
        pub fn new(row: usize, col: usize) -> Self {
            // Asserting the pos is within bounds we want or else we panic.
            assert_eq!(row < MAX_ROW, col < MAX_COL);
            Pos(row, col)
        }
        pub fn coords(&self) -> (usize, usize) {
            (self.0, self.1)
        }
        pub fn try_move(&self, x: i32, y: i32) -> Option<Self> {
            let (row, col) = (self.0 as i32, self.1 as i32);
            if row + y < 0 || row + y >= MAX_ROW as i32 || col + x < 0 || col + x >= MAX_COL as i32
            {
                None
            } else {
                Some(Pos((row + y) as usize, (col + x) as usize))
            }
        }
        pub fn in_range(row: i32, col: i32) -> bool {
            return row >= 0 && row < MAX_ROW as i32 && col >= 0 && col < MAX_COL as i32;
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
                origin: Pos::new(20, 4),
                rotation: State::Up,
            }
        }

        /// Gets the positions of the squares that the active piece represents.
        fn get_squares(&self) -> [(i32, i32); 4] {
            let (y, x) = (self.origin.0 as i32, self.origin.1 as i32);
            self.tetromino
                .shape(self.rotation)
                .into_iter()
                .map(|(a, b)| (y + b, x + a))
                .collect::<Vec<(i32, i32)>>()
                .try_into()
                .unwrap()
        }

        /// Validates if a new state that we've passed in is valid within the
        /// board. If it is, we update the state and return true to signify that we
        /// updated.
        fn validate(&mut self, new_state: &ActivePiece, board: &[[u8; MAX_COL]; MAX_ROW]) -> bool {
            for (row, col) in new_state.get_squares() {
                if !Pos::in_range(row, col) {
                    return false;
                }
                if board[row as usize][col as usize] != 0 {
                    return false;
                }
            }
            self.origin = new_state.origin;
            self.rotation = new_state.rotation;
            true
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
            // Getting our new rotational state.
            let new_rotation = self.rotation.rotate(clockwise);
            let (row, col) = (self.origin.0 as i32, self.origin.1 as i32);
            // These are the different "origin" states we will be testing.
            let origin = if let Tetromino::I = self.tetromino {
                match (clockwise, new_rotation) {
                    (true, State::Up) | (false, State::Right) => (row - 1, col),
                    (true, State::Right) | (false, State::Down) => (row, col + 1),
                    (true, State::Down) | (false, State::Left) => (row + 1, col),
                    (true, State::Left) | (false, State::Up) => (row, col - 1),
                }
            } else {
                (row, col)
            };
            let mut origins = vec![origin];
            // "Kick data" refers to the possible offset values that can be used for the 4 kick states.
            // There are 8 total different offset value sets; 4 of which are inverted from the other 4.
            // Refers to the I Tetromino.
            let kick_data_i1 = vec![(-2, 0), (1, 0), (-2, -1), (1, -2)];
            let kick_data_i2 = vec![(-1, 0), (2, 0), (-1, -2), (2, 1)];
            // Refers to the other 5 (non-O) Tetrominos.
            let kick_data = vec![(-1, 0), (-1, -1), (0, 2), (-1, 2)];
            // We extend our possible tests with the 4 additional tests:
            origins.extend(
                match self.tetromino {
                    Tetromino::O => return, /* O Tetromino's have no rotational logic. */
                    Tetromino::I => match (self.rotation, new_rotation) {
                        // CW from Spawn State OR CCW to Inverted Spawn State
                        (State::Up, State::Right) | (State::Left, State::Down) => kick_data_i1,
                        // CCW to Spawn State OR CW from Inverted Spawn State
                        (State::Right, State::Up) | (State::Down, State::Left) => {
                            kick_data_i1.into_iter().map(|(x, y)| (-x, -y)).collect()
                        }
                        // CW to Inverted Spawn State OR CCW from Spawn State
                        (State::Right, State::Down) | (State::Up, State::Left) => kick_data_i2,
                        // CCW to Inverted Spawn State OR CW to Spawn State
                        (State::Down, State::Right) | (State::Left, State::Up) => {
                            kick_data_i2.into_iter().map(|(x, y)| (-x, -y)).collect()
                        }
                        _ => unreachable!(), /* THIS SHOULD NEVER HAPPEN. */
                    },
                    _ => match (self.rotation, new_rotation) {
                        // CW from Spawn State OR CCW to Inverted Spawn State
                        (State::Up, State::Right) | (State::Down, State::Right) => kick_data,
                        // CCW to Spawn State OR CW from Inverted Spawn State
                        (State::Right, State::Up) | (State::Right, State::Down) => {
                            kick_data.into_iter().map(|(x, y)| (-x, -y)).collect()
                        }
                        // CW from Inverted Spawn State OR CW to Spawn State
                        (State::Down, State::Left) | (State::Left, State::Up) => {
                            kick_data.into_iter().map(|(x, y)| (-x, y)).collect()
                        }
                        // CCW to Inverted Spawn State OR CCW from Spawn State
                        (State::Left, State::Down) | (State::Up, State::Left) => {
                            kick_data.into_iter().map(|(x, y)| (x, -y)).collect()
                        }
                        _ => unreachable!(), /* THIS SHOULD NEVER HAPPEN. */
                    },
                }
                .into_iter()
                .map(|(x, y)| (row + y, col + x)),
            );
            // Turning these into Positions (when they're possible).
            let tests = origins.into_iter().flat_map(|(row, col)| {
                if Pos::in_range(row, col) {
                    Some(Pos(row as usize, col as usize))
                } else {
                    None
                }
            });
            // Attempting all of our tests.
            for new_pos in tests {
                // Returning if we've successfully validated a given state!
                if self.validate(
                    &ActivePiece {
                        tetromino: self.tetromino,
                        origin: new_pos,
                        rotation: new_rotation,
                    },
                    board,
                ) {
                    return;
                }
            }
        }

        /// Attempt to move a piece down by 1 pos
        /// if successful, update active piece position, return `true`
        /// if not, return `false`
        fn drop(&mut self, board: &[[u8; MAX_COL]; MAX_ROW]) -> bool {
            if let Some(new_pos) = self.origin.try_move(0, 1) {
                return self.validate(
                    &ActivePiece {
                        tetromino: self.tetromino,
                        origin: new_pos,
                        rotation: self.rotation,
                    },
                    board,
                );
            }
            false
        }

        fn shift(&mut self, left: bool, board: &[[u8; MAX_COL]; MAX_ROW]) -> bool {
            if let Some(pos) = self.origin.try_move(if left { -1 } else { 1 }, 0) {
                return self.validate(
                    &ActivePiece {
                        tetromino: self.tetromino,
                        origin: pos,
                        rotation: self.rotation,
                    },
                    board,
                );
            }
            false
        }
    }

    pub struct Tetris {
        board: [[u8; MAX_COL]; MAX_ROW],
        active: ActivePiece,
        bag: Bag,
        held: Option<Tetromino>,
        queue: VecDeque<Tetromino>,
        pub is_game_over: bool,
    }
    impl Tetris {
        pub fn default() -> Self {
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
                held: None,
                queue,
                bag,
                is_game_over: false,
            }
        }

        pub fn new(provided_board: Option<[[u8; MAX_COL]; MAX_ROW]>, active_piece : Option<Tetromino>) -> Self { // For testing purposes, allows setting board to arbitrary state
            /*
                Similar to default(), this method initialized a Tetris object, however it allows pre-set board and
                active_piece options. Not using these parameters will result in identical results to calling default().
             */
            let board;
            if let Some(b) = provided_board{ // If arg for board is provided, use it.
                board = b;
            }
            else{
                board = [[0; MAX_COL]; MAX_ROW];
            }
            
            let mut bag = Bag::new();
        
            let mut active : ActivePiece;
            if let Some(t) = active_piece{// If arg for first piece is provided, use it.
                active = ActivePiece::new(t);
            }
            else{
                if let Some(t) = bag.next() { // Note: We might want a way to pre-set the bag as well for testing specific sequences.
                    active = ActivePiece::new(t);
                }
                active = ActivePiece::new(Tetromino::I); // Will never exec.
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
                held: None,
                queue,
                bag,
                is_game_over: false,
            }
        }


        /// Return the next piece in the queue and pull a new piece
        /// from the bag to replace it
        fn next_piece(&mut self) -> Option<Tetromino> {
            if let Some(tet) = self.bag.next() {
                self.queue.push_back(tet);
            }

            self.queue.pop_front()
        }

        /// Call the active piece's drop() to update its position if possible.
        /// If not, write piece to game board and draw new piece.
        pub fn drop(&mut self) {
            if !self.active.drop(&self.board) {
                for (row, col) in self.active.get_squares() {
                    if row < 20 {
                        self.is_game_over = true;
                    }
                    self.board[row as usize][col as usize] = self.active.tetromino as u8;
                }
                if let Some(next_tet) = self.next_piece() {
                    self.active = ActivePiece::new(next_tet);
                }
            }
        }

        /// Immediately drop piece as far as it will go, and solidify at final
        /// position.
        pub fn hard_drop(&mut self) {
            while self.active.drop(&self.board) {}

            for (row, col) in self.active.get_squares() {
                if row < 20 {
                    self.is_game_over = true;
                }
                self.board[row as usize][col as usize] = self.active.tetromino as u8;
            }
            if let Some(next_tet) = self.next_piece() {
                self.active = ActivePiece::new(next_tet);
            }
        }

        /// Call the active piece's rotate()
        pub fn rotate(&mut self, clockwise: bool) {
            self.active.rotate(clockwise, &self.board);
        }

        pub fn shift(&mut self, left: bool) {
            self.active.shift(left, &self.board);
        }

        /// Erase filled rows and move rows above down
        pub fn clear_lines(&mut self) {
            for row in (0..MAX_ROW).rev() {               
                loop {
                    let is_solid = self.board[row].iter().all(|&itm| itm != 0);

                    if is_solid {
                        self.board[row].iter_mut().for_each(|x| *x = 0);

                        for sub_row in (0..row).rev() {
                            for col in 0..MAX_COL {
                                self.board[sub_row + 1][col] = self.board[sub_row][col];
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        /// Hold functionality
        /// If no piece is held, place active piece in hold and draw new piece
        /// If something is held, swap held & active piece
        pub fn hold(&mut self) {
            if self.held.is_none() {
                self.held = Some(self.active.tetromino);

                if let Some(next) = self.next_piece() {
                    self.active = ActivePiece::new(next);
                }
            } else {
                if let Some(temp_tet) = self.held {
                    self.held = Some(self.active.tetromino);
                    self.active = ActivePiece::new(temp_tet);
                }
            }
        }
    }
    impl Display for Tetris {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Generating the "Held" Area
            let mut held_render = [[' '; 8]; 20];
            if let Some(held) = self.held {
                let shape = held.shape(State::Up);
                for (x, y) in shape {
                    let (r, c) = ((2 + y) as usize, ((2 + (x * 2)) as usize));
                    held_render[r][c] = '[';
                    held_render[r][c + 1] = ']';
                }
            }
            // Generating the gameboard area
            let mut board_render = [[' '; MAX_COL * 2]; 20];
            for row in 0..20 {
                for col in 0..MAX_COL {
                    if self.board[row + 20][col] == 0 {
                        board_render[row][2 * col] = ' ';
                        board_render[row][(2 * col) + 1] = '.';
                    } else {
                        board_render[row][2 * col] = '[';
                        board_render[row][(2 * col) + 1] = ']';
                    }
                }
            }
            // Generating the active piece.
            let (mut min_col, mut max_col) = (11, 0);
            for (row, col) in self.active.get_squares() {
                let (row, col) = (row as usize, col as usize);
                // For "ghosting".
                min_col = min_col.min(col);
                max_col = max_col.max(col);
                if row >= 20 {
                    board_render[row - 20][2 * col] = '[';
                    board_render[row - 20][(2 * col) + 1] = ']';
                }
            }
            // Generating the "Queue" Area
            let mut queue_render = [[' '; 8]; 20];
            let mut ren_row = -1;

            for piece in &self.queue {
                let shape = piece.shape(State::Up);
                ren_row += if let Tetromino::I = piece { 2 } else { 3 };
                for (x, y) in shape {
                    let (r, c) = ((ren_row + y) as usize, ((2 + (x * 2)) as usize));
                    queue_render[r][c] = '[';
                    queue_render[r][c + 1] = ']';
                }
            }
            // Rendering our generated info.
            writeln!(f, "   HELD                              NEXT   ")?;
            for row in 0..20 {
                let h_render: String = held_render[row].into_iter().collect();
                let b_render: String = board_render[row].into_iter().collect();
                let q_render: String = queue_render[row].into_iter().collect();
                writeln!(f, " {} <!{}!> {} ", h_render, b_render, q_render)?;
            }
            write!(f, "          <!")?;
            for c in 0..10 {
                if c >= min_col && c <= max_col {
                    write!(f, "##")?;
                } else {
                    write!(f, "==")?;
                }
            }
            writeln!(f, "!>          ")?;
            writeln!(f, "            \\/\\/\\/\\/\\/\\/\\/\\/\\/\\/            ")?;
            Ok(())
        }
    }
}
