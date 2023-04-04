const MAX_ROW: usize = 40;
const MAX_COL: usize = 10;
mod good_get_squares {
    #[test]
    fn test() -> Result<(), String> {
        let board = [[0; MAX_COL]; MAX_ROW];
        let mut bag = Bag::new();
        let piece =  ActivePiece {
            tetromino: Tetromino::I,
            origin: Pos(0,1),
            rotation: State::Right,
        };

        let ans = [Pos(0,0),Pos(0,1),Pos(0,2),Pos(0,3)];
        assert(piece.get_squares() == ans);
    }
}