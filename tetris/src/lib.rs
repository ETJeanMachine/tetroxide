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
        I,
        O,
        T,
        J,
        L,
        S,
        Z,
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
    impl Display for Tetromino {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Tetromino::I => writeln!(f, "[][][][]"),
                Tetromino::O => writeln!(f, "[][]\n[][]"),
                Tetromino::T => writeln!(f, "  []    \n[][][]  "),
                Tetromino::J => writeln!(f, "[]      \n[][][]  "),
                Tetromino::L => writeln!(f, "    []  \n[][][]  "),
                Tetromino::S => writeln!(f, "  [][]  \n[][]    "),
                Tetromino::Z => writeln!(f, "[][]    \n  [][]  "),
            }
        }
    }
    impl From<Tetromino> for u8 {
        fn from(val: Tetromino) -> Self {
            match val {
                Tetromino::I => 1,
                Tetromino::O => 2,
                Tetromino::T => 3,
                Tetromino::J => 4,
                Tetromino::L => 5,
                Tetromino::S => 6,
                Tetromino::Z => 7,
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
            row >= 0 && row < MAX_ROW as i32 && col >= 0 && col < MAX_COL as i32
        }
    }

    #[derive(Debug, Clone, Copy)]
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
        fn rotate(&mut self, clockwise: bool, board: &[[u8; MAX_COL]; MAX_ROW]) -> bool {
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
                    Tetromino::O => return false, /* O Tetromino's have no rotational logic. */
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
                    return true;
                }
            }
            false
        }

        /// Attempt to move a piece down by 1 pos
        /// if successful, update active piece position, return `true`
        /// if not, return `false`
        fn soft_drop(&mut self, board: &[[u8; MAX_COL]; MAX_ROW]) -> bool {
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
        held: (Option<Tetromino>, bool),
        queue: VecDeque<Tetromino>,
        delay_count: u8,
        gravity_count: f64,
        pub score: u32,
        pub level: u32,
        pub lines: u32,
        pub is_game_over: bool,
    }
    impl Default for Tetris {
        fn default() -> Self {
            let board = [[0; MAX_COL]; MAX_ROW];
            let mut bag = Bag::new();
            // Setting the active piece.
            let active = ActivePiece::new(bag.next().unwrap());

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
                bag,
                held: (None, false),
                queue,
                delay_count: 0,
                gravity_count: 0.0,
                score: 0,
                level: 0,
                lines: 0,
                is_game_over: false,
            }
        }
    }

    /// The number of frames we want to wait before trying to lock a piece.
    const LOCK_DELAY: u8 = 30;
    impl Tetris {
        pub fn new(
            provided_board: Option<[[u8; MAX_COL]; MAX_ROW]>,
            active_piece: Option<Tetromino>,
        ) -> Self {
            // For testing purposes, allows setting board to arbitrary state
            /*
               Similar to default(), this method initialized a Tetris object, however it allows pre-set board and
               active_piece options. Not using these parameters will result in identical results to calling default().
            */
            let board;
            if let Some(b) = provided_board {
                // If arg for board is provided, use it.
                board = b;
            } else {
                board = [[0; MAX_COL]; MAX_ROW];
            }

            let mut bag = Bag::new();

            let active = if let Some(t) = active_piece {
                // If arg for first piece is provided, use it.
                ActivePiece::new(t)
            } else {
                ActivePiece::new(bag.next().unwrap())
            };

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
                held: (None, false),
                queue,
                bag,
                delay_count: 0,
                gravity_count: 0.0,
                score: 0,
                level: 0,
                lines: 0,
                is_game_over: false,
            }
        }

        pub fn set_level(&mut self, level: u32) {
            if level >= 15 {
                self.level = 15;
            } else {
                self.level = level;
            }
        }

        /// This advances forward the game by a singular frame.
        /// The game assumes that 60 frames occur per second,
        /// and additionally, internally calculates the speed at which
        /// blocks will fall in this method.
        ///
        /// The standard for figuring out the speed of the blocks is based upon
        /// the Tetris guidelines, which uses the rules from [Tetris Worlds]
        /// (https://tetris.fandom.com/wiki/Tetris_Worlds). So, this formula:
        ///
        /// ```
        /// let time = f64::powf(0.8 - ((level - 1.0) * 0.007), level - 1.0);
        /// ```
        ///
        /// Where `time` refers to the amount of time spent in a single cell.
        pub fn frame_advance(&mut self) {
            // Computes the "gravity" of the current level.
            let l = self.level as f64 - 1.0;
            let time = f64::powf(0.8 - (l * 0.007), l);
            self.gravity_count += 1.0 / (time * 60.0);
            self.delay_count += 1;
            // Getting the total number of cells we need to advance...
            for _ in 0..self.gravity_count as u8 {
                self.gravity_count -= 1.0;
                // Trying to drop the piece - if not, we try to lock it.
                if !self.active.soft_drop(&self.board) {
                    self.try_lock();
                } else {
                    self.delay_count = 0;
                }
            }
        }

        /// Call the active piece's soft_drop() to update its position if possible.
        /// If not, write piece to game board and draw new piece.
        pub fn soft_drop(&mut self) {
            self.score += 1;
            if !self.active.soft_drop(&self.board) {
                self.lock();
            }
        }

        /// Immediately drop piece as far as it will go, and solidify at final
        /// position.
        pub fn hard_drop(&mut self) {
            while self.active.soft_drop(&self.board) {
                self.score += 2;
            }
            self.lock();
            self.try_clear();
        }

        /// Call the active piece's rotate()
        pub fn rotate(&mut self, clockwise: bool) {
            self.active.rotate(clockwise, &self.board);
            // TODO: Logic for auto-locking once in an immobile state.
            if self.try_lock() {
                self.try_clear();
            }
        }

        /// Shifts a piece to the left/right.
        pub fn shift(&mut self, left: bool) {
            self.active.shift(left, &self.board);
            if self.try_lock() {
                self.try_clear();
            }
        }

        /// Hold functionality
        /// If no piece is held, place active piece in hold and draw new piece
        /// If something is held, swap held & active piece.
        ///
        /// A piece can only be removed from held once a lock has occurred.
        pub fn hold(&mut self) {
            if let (Some(tetromino), true) = self.held {
                self.held = (Some(self.active.tetromino), false);
                self.active = ActivePiece::new(tetromino);
            } else if self.held.0.is_none() {
                self.held = (Some(self.active.tetromino), false);
                self.active = ActivePiece::new(self.next_piece());
            }
        }

        /// Return the next piece in the queue and pull a new piece
        /// from the bag to replace it
        fn next_piece(&mut self) -> Tetromino {
            let popped = self.queue.pop_front();
            if let Some(tet) = self.bag.next() {
                self.queue.push_back(tet);
            }
            popped.unwrap()
        }

        /// `try_lock` attempts to lock the piece onto the board. It takes in a bool
        /// specifying whether or not we want to force the locking of the piece - as
        /// for certain moves (such as T-spins and hard drops) we want this to occur.
        fn try_lock(&mut self) -> bool {
            if self.delay_count < LOCK_DELAY {
                // Piece's won't lock if they're not being forced to and they're under
                // the required frame count.
                self.delay_count += 1;
                false
            } else {
                // Here we check to see if the piece is immobile. If it is, we lock it.
                let mut cloned = self.active;
                if !(cloned.shift(true, &self.board)
                    && cloned.shift(false, &self.board)
                    && cloned.rotate(true, &self.board)
                    && cloned.rotate(false, &self.board))
                {
                    self.lock();
                    true
                } else {
                    false
                }
            }
        }

        /// Locks the active piece immediately in place.
        fn lock(&mut self) {
            // Locking the piece onto the board.
            for (row, col) in self.active.get_squares() {
                // Updating the game over state if we're locking above 20.
                if !self.is_game_over {
                    self.is_game_over = row <= 20
                };
                self.board[row as usize][col as usize] = u8::from(self.active.tetromino);
            }
            // Updating the active piece.
            self.active = ActivePiece::new(self.next_piece());
            // Allowing the held piece to be usable (if not already).
            self.held = (self.held.0, true)
        }

        /// Erase filled rows and move rows above down; as well as update the score to match.
        fn try_clear(&mut self) {
            let mut l_count = 0;
            for row in (0..MAX_ROW).rev() {
                loop {
                    let is_solid = self.board[row].iter().all(|&itm| itm != 0);
                    if is_solid {
                        l_count += 1;
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
            // Adding up our score.
            self.lines += l_count;
            self.score += self.level
                * match l_count {
                    1 => 100,
                    2 => 300,
                    3 => 500,
                    4 => 800,
                    _ => 0,
                };
        }
    }
    impl Display for Tetris {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Render the "Held" Area.
            let held_str = self.held.0.map_or(String::new(), |h| h.to_string());
            let mut held_lines = held_str.lines();
            // Rendering the gameboard area.
            let mut board_render = Vec::with_capacity(20);
            for r in 0..20 {
                let row_str: String = self.board[r + 20]
                    .into_iter()
                    .map(|x| if x == 0 { " ." } else { "[]" })
                    .collect();
                board_render.push(row_str);
            }
            // Rendering the piece.
            let mut ghost = self.active;
            while ghost.soft_drop(&self.board) {}
            let (g_squares, a_squares) = (ghost.get_squares(), self.active.get_squares());
            for i in 0..4 {
                let ((g_r, g_c), (a_r, a_c)) = (g_squares[i], a_squares[i]);
                let (g_r, g_c, a_r, a_c) = (g_r as usize, g_c as usize, a_r as usize, a_c as usize);
                // The ghost piece first.
                if g_r >= 20 {
                    board_render[g_r - 20].replace_range(2 * g_c..2 * (g_c + 1), " X");
                }
                // Then the active piece.
                if a_r >= 20 {
                    board_render[a_r - 20].replace_range(2 * a_c..2 * (a_c + 1), "[]");
                }
            }
            // Rendering the "Queue" Area.
            let mut queue: VecDeque<String> =
                self.queue.iter().rev().map(|t| t.to_string()).collect();
            // Top of the Tetris Game.
            writeln!(f, "{:>7}{:>34}", "HELD", "NEXT")?;
            let score_info = format!(
                "{:10}\n{:->9}\n{:<10}\n\n{:10}\n{:->9}\n{:<10}\n\n{:10}\n{:->9}\n{:<10}",
                "SCORE", "", self.score, "LEVEL", "", self.level, "LINES", "", self.lines
            );
            let mut score_lines = score_info.lines();
            let mut queue_string = queue.pop_back().unwrap_or_default();
            let mut queue_lines = queue_string.lines();
            for (row, centre_render) in board_render.iter().enumerate() {
                let (mut left_render, mut right_render) = ("", "");
                if row > 0 {
                    left_render = if row >= 4 {
                        score_lines.next().unwrap_or_default()
                    } else {
                        held_lines.next().unwrap_or_default()
                    };
                    right_render = queue_lines.next().unwrap_or_default();
                }
                writeln!(
                    f,
                    "{:^10}<!{}!>{:^10}",
                    left_render, centre_render, right_render,
                )?;
                if right_render.is_empty() && !queue_string.is_empty() && row != 0 {
                    queue_string = queue.pop_back().unwrap_or_default();
                    queue_lines = queue_string.lines();
                }
            }
            // Bottom of the board.
            writeln!(f, "{:>12}{:=>20}!>", "<!", "")?;
            writeln!(f, "{:>32}", "\\/".repeat(10))?;
            Ok(())
        }
    }
}
